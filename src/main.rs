//use clippy
#![deny(clippy::all)]

use tonic::transport::Server;

mod wifi;
mod wifi_module;

mod display;
mod display_module;

mod battery;
mod battery_module;

use crate::wifi_module::WifiService;
use wifi::wifi_server::WifiServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse().unwrap();
    let wifi_service = WifiService::default();

    println!("Mecha Edge Server listening on {}", addr);

    Server::builder()
        .add_service(WifiServer::new(wifi_service))
        .serve(addr)
        .await?;

    Ok(())
}
