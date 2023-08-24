use tonic::{transport::Server, Request, Response, Status};

mod wifi;

use wifi::wifi_server::{Wifi, WifiServer};
use wifi::{Empty, NetworkResult, NetworkResults, ScanResult, ScanResults};

use mecha_device_sdk::{get_known_wifi_list, get_wifi_list};

//wifi struct with default implementation
#[derive(Default)]
pub struct WifiImpl {}

//implement the trait
#[tonic::async_trait]
impl Wifi for WifiImpl {
    async fn get_wifi(&self, request: Request<Empty>) -> Result<Response<ScanResults>, Status> {
        let mut scan_results = ScanResults::default();

        log::info!("Starting All Wifi List Function");

        //get wifi list from mecha_edge_sdk
        let wifi_list = get_wifi_list().await.unwrap();

        //add wifi list to scan_results
        for wifi in wifi_list {
            let mut scan_result = ScanResult::default();
            scan_result.mac = wifi.mac;
            scan_result.frequency = wifi.frequency;
            scan_result.signal = wifi.signal as i32;
            scan_result.flags = wifi.flags;
            scan_result.name = wifi.name;
            scan_results.results.push(scan_result);
        }

        // add 5 scan results
        // for i in 0..5 {
        //     let mut scan_result = ScanResult::default();
        //     scan_result.mac = format!("test{}", i);
        //     scan_result.frequency = format!("Strong{}", i);
        //     scan_result.signal = i;
        //     scan_result.flags = format!("wifi{}", i);
        //     scan_result.name = format!("jack{}", i);
        //     scan_results.results.push(scan_result);
        // }

        Ok(Response::new(scan_results))
    }

    async fn get_known_wifi(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<NetworkResults>, Status> {
        let mut scan_results = NetworkResults::default();
        log::info!("Starting Known Wifi List Function");

        //get wifi list from mecha_edge_sdk
        let wifi_list = get_known_wifi_list().await.unwrap();

        //add wifi list to scan_results
        for wifi in wifi_list {
            let mut scan_result = NetworkResult::default();
            scan_result.network_id = wifi.network_id.to_string();
            scan_result.flags = wifi.flags;
            scan_result.ssid = wifi.ssid;
            scan_results.results.push(scan_result);
        }

        Ok(Response::new(scan_results))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse().unwrap();
    let wifi_store = WifiImpl::default();

    println!("Wifi server listening on {}", addr);

    Server::builder()
        .add_service(WifiServer::new(wifi_store))
        .serve(addr)
        .await?;

    Ok(())
}
