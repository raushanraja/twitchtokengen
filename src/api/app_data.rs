use std::sync::Arc;

use retainer::Cache;
use twitch_oauth2::{ClientId, ClientSecret, UserTokenBuilder};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ClientData {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub redirect_url: twitch_oauth2::url::Url,
}

#[derive(Clone)]
pub struct AppState {
    pub server_id: Uuid,
    pub client_data: ClientData,
    pub cache: Arc<Cache<String, UserTokenBuilder>>,
}
