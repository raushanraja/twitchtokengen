# actix_api_starter

This is a starter project for building APIs using Rust.

## Running the Server
To run the server, use the following command:

```sh
cargo run
```

## Configuration

The environment variables can be updated in the `.env` file. These variables are loaded in the `configure` method of the `src/config/mod.rs` file.

## Endpoints

The endpoints are added to the `src/api/v1/endpoints` directory.

## Adding New Endpoints

After adding a new endpoint, make sure to add new service to the end of `App::new().app_data(server_id).wrap(cors).service(root)` to include the new endpoint.


