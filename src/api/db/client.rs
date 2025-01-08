use postgrest::Postgrest;

pub fn get_client(endpoint: &str, apikey: &str) -> Postgrest {
    let endpoint = format!("{}/rest/v1", endpoint);
    Postgrest::new(endpoint).insert_header("apikey", apikey)
}
