use dioxus::prelude::*;
use tracing::{info, Level};

use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
pub async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}

#[server]
pub async fn getsomething() -> Result<String, ServerFnError> {
    Ok("ok".to_string())
}
