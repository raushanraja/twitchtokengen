use std::sync::{Arc, Mutex};

use actix_web::{
    web::{self},
    HttpRequest, HttpResponse, Responder,
};
use anyhow::{Context, Result};
use log::debug;
use postgrest::Postgrest;
use retainer::CacheExpiration;
use serde::{Deserialize, Serialize};
use twitch_oauth2::{Scope, UserTokenBuilder};

use crate::api::app_data::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
    pub code: String,
    pub access_token: String,
    pub refresh_token: String,
    pub login: String,
}

impl UserToken {
    pub fn new(
        user_id: String,
        code: String,
        access_token: String,
        refresh_token: String,
        login: String,
    ) -> Self {
        UserToken {
            created_at: chrono::Utc::now(),
            user_id,
            code,
            access_token,
            refresh_token,
            login,
        }
    }
}

#[actix_web::get("/generatetoken")]
pub async fn generatetoken(_req: HttpRequest, appstate: web::Data<AppState>) -> impl Responder {
    let client_id = &appstate.client_data.client_id;
    let cache = Arc::new(Mutex::new(appstate.cache.clone()));
    let redirect_url = &appstate.client_data.redirect_url;
    let client_secret = &appstate.client_data.client_secret;

    // Create the builder!
    let mut builder = UserTokenBuilder::new(
        client_id.clone(),
        client_secret.clone(),
        redirect_url.clone(),
    );
    builder.add_scope(Scope::BitsRead);
    builder.add_scope(Scope::ChannelModerate);
    builder.add_scope(Scope::ModeratorManageBannedUsers);
    builder.add_scope(Scope::ChannelBot);
    builder.add_scope(Scope::UserReadChat);
    builder.add_scope(Scope::UserBot);

    let (url, csrf_token) = builder.generate_url();

    let state = csrf_token.secret().to_string();
    debug!("Saving State: {}", state);
    match cache.lock() {
        Ok(cache_guard) => {
            cache_guard
                .insert(state.clone(), builder, CacheExpiration::none())
                .await;
        }
        Err(e) => {
            debug!("Failed to acquire cache lock: {:?}", e);
            return HttpResponse::InternalServerError()
                .body("Error generating url, try again token");
        }
    }

    debug!("Redirecting URL: {}", url);
    HttpResponse::Found()
        .append_header(("Location", url.to_string()))
        .finish()
}

#[actix_web::get("/")]
pub async fn callback(
    req: HttpRequest,
    appstate: web::Data<AppState>,
    pgclient: web::Data<Postgrest>,
) -> impl Responder {
    let full_url = req.full_url();
    let url = full_url.as_str();

    if let Ok(twitch_url) = twitch_oauth2::url::Url::parse(url) {
        let query_map = twitch_url
            .query_pairs()
            .collect::<std::collections::HashMap<_, _>>();
        match (
            &query_map.get("code"),
            &query_map.get("scope"),
            &query_map.get("state"),
        ) {
            (Some(code), Some(scope), Some(state)) => {
                debug!("scopes: {:?}", scope);
                debug!("code: {:?}", code);
                debug!("state: {:?}\n\n", state);

                let state = state.to_string();
                debug!("Getting State: {}", state);
                let builder = match appstate.cache.remove(&state).await {
                    Some(builder) => builder,
                    None => {
                        return HttpResponse::InternalServerError()
                            .body("Error getting token builders")
                    }
                };

                let Ok(user_token) = get_user_token(builder, code, &state).await else{
                    return HttpResponse::InternalServerError().body("Error getting user token");
                };

                if let Ok(resp) = pgclient
                    .from("user_token")
                    .insert(serde_json::to_string(&user_token).unwrap())
                    .execute()
                    .await
                {
                    println!("{:?}", resp);
                } else {
                    return HttpResponse::InternalServerError().body("Error inserting user token");
                }

                debug!("User Token: {:?}", user_token);
                HttpResponse::Ok().body("You can close this tab now")
            }
            _ => HttpResponse::BadRequest().body("Invalid URL"),
        }
    } else {
        HttpResponse::BadRequest().body("Invalid URL")
    }
}

pub async fn get_user_token(
    builder: UserTokenBuilder,
    code: &str,
    state: &str,
) -> Result<UserToken> {
    let reqwest = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .context("Error building reqwest client")?;

    let token = builder
        .get_user_token(&reqwest, state, code)
        .await
        .context("Error getting user token")?;

    Ok(UserToken::new(
        token.user_id.to_string(),
        code.to_string(),
        token.access_token.secret().to_string(),
        token
            .refresh_token
            .is_some()
            .then(|| token.refresh_token.unwrap().secret().to_string())
            .unwrap_or_default(),
        token.login.to_string(),
    ))
}
