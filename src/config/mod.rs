use dotenvy::dotenv;

pub fn configure() {
    match dotenv() {
        Ok(_) => {}
        Err(e) => panic!("No .env file present: {:?}", e),
    }

    match std::env::var("RUST_LOG") {
        Ok(_) => {}
        Err(_) => std::env::set_var("RUST_LOG", "debug"),
    }

    env_logger::init();
}
