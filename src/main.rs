use actix_cors::Cors;
use anyhow::{Context, Result};

use actix_web::{web, App, HttpServer};
use api_starter::{
    api::{
        app_data::{AppState, ClientData},
        v1::endpoints::auth::{callback, generatetoken},
    },
    config::configure,
};
use uuid::Uuid;

fn get_env_or_arg(env: &str, args: &mut impl Iterator<Item = String>) -> Option<String> {
    std::env::var(env).ok().or_else(|| args.next())
}

fn get_client_data() -> Result<ClientData> {
    let mut args = std::env::args();
    // Grab the client id, convert to a `ClientId` with the `new` method.
    let client_id = get_env_or_arg("TWITCH_CLIENT_ID", &mut args)
        .map(twitch_oauth2::ClientId::new)
        .context("Please set env: TWITCH_CLIENT_ID or pass as first argument")?;

    // Grab the client secret, convert to a `ClientSecret` with the `new` method.
    let client_secret = get_env_or_arg("TWITCH_CLIENT_SECRET", &mut args)
        .map(twitch_oauth2::ClientSecret::new)
        .context("Please set env: TWITCH_CLIENT_SECRET or pass as second argument")?;

    // Grab the redirect URL, this has to be set verbatim in the developer console: https://dev.twitch.tv/console/apps/
    let redirect_url = get_env_or_arg("TWITCH_REDIRECT_URL", &mut args)
        .map(|r| twitch_oauth2::url::Url::parse(&r))
        .context("Please set env: TWITCH_REDIRECT_URL or pass as third argument")??;

    Ok(ClientData {
        client_id,
        client_secret,
        redirect_url,
    })
}

async fn serve() {
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let pg_client = api_starter::api::db::client::get_client(
        &std::env::var("SUPABASE_URL").expect("POSTGREST_ENDPOINT not set"),
        &std::env::var("SUPABASE_SECRET_KEY").expect("POSTGREST_APIKEY not set"),
    );

    let server_id = Uuid::new_v4();

    let client_data = match get_client_data() {
        Ok(data) => data,
        Err(e) => {
            panic!(
                "Error getting client data, unable to find required environment variables {:?}",
                e
            );
        }
    };

    let app_state = AppState {
        server_id,
        client_data,
        cache: Default::default(),
    };

    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(pg_client.clone()))
            .wrap(cors)
            .service(generatetoken)
            .service(callback)
    })
    .bind(format!("0.0.0.0:{}", port))
    .expect("Error starting")
    .run()
    .await;
}

#[actix_web::main]
async fn main() {
    configure();
    serve().await;
}
