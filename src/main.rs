//use clippy
#![deny(clippy::all)]

use tonic::transport::Server;

use crate::wifi_service::{WifiServer, WifiService};
mod wifi_service;

use crate::display_service::{DisplayServiceServer, DisplayServices};
mod display_service;

mod battery_servcie;
use crate::battery_servcie::{BatteryService, PowerSupplyServiceServer};

mod gyro_service;
use crate::gyro_service::{GyroServiceServer, GyroServices};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse().unwrap();
    let wifi_service = WifiService::default();
    let display_service = DisplayServices::default();
    let battery_service = BatteryService::default();
    let gyro_service = GyroServices::default();

    println!("Mecha Edge Server listening on {}", addr);

    Server::builder()
        .add_service(WifiServer::new(wifi_service))
        .add_service(DisplayServiceServer::new(display_service))
        .add_service(PowerSupplyServiceServer::new(battery_service))
        .add_service(GyroServiceServer::new(gyro_service))
        .serve(addr)
        .await?;
    Ok(())
}
