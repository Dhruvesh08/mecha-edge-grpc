use tonic::{transport::Server, Request, Response, Status};

mod wifi;

use wifi::wifi_server::{Wifi, WifiServer};
use wifi::{
    Empty, NetworkResult, NetworkResults, RemoveNetworkRequest, RemoveNetworkResponse, ScanResult,
    ScanResults, WifiConnectRequest, WifiConnectResponse,
};

use mecha_device_sdk::{get_known_wifi_list, get_wifi_list};

//wifi struct with default implementation
#[derive(Default)]
pub struct WifiImpl {}

//implement the trait
#[tonic::async_trait]
impl Wifi for WifiImpl {
    async fn get_wifi(&self, _request: Request<Empty>) -> Result<Response<ScanResults>, Status> {
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

        Ok(Response::new(scan_results))
    }

    async fn get_known_wifi(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<NetworkResults>, Status> {
        let mut scan_results = NetworkResults::default();
        log::info!("Starting Known Wifi List Function");

        //get wifi list from mecha_edge_sdk
        let wifi_list = get_known_wifi_list().await.unwrap();

        //add wifi list to scan_results
        for wifi in wifi_list {
            let mut scan_result = NetworkResult::default();
            scan_result.network_id = wifi.network_id as i32;
            scan_result.flags = wifi.flags;
            scan_result.ssid = wifi.ssid;
            scan_results.results.push(scan_result);
        }

        Ok(Response::new(scan_results))
    }

    async fn wifi_connect(
        &self,
        request: Request<WifiConnectRequest>,
    ) -> Result<Response<WifiConnectResponse>, Status> {
        let mut wifi_connect_response = WifiConnectResponse::default();

        log::info!("Starting Wifi Connect Function");

        let request_data = request.into_inner();

        //get ssid and psk from request
        let ssid = request_data.ssid;
        let psk = request_data.psk;

        //get get_connect_wifi from mecha_edge_sdk
        //get_connect_wifi  accepts ssid and psk as parameter
        let connect_wifi = mecha_device_sdk::get_connect_wifi(&ssid, &psk).await;

        match connect_wifi {
            Ok(_) => {
                wifi_connect_response.success = true;
                wifi_connect_response.message = "WiFi connection successful".to_string();
            }
            Err(_) => {
                wifi_connect_response.success = false;
                wifi_connect_response.message = "WiFi connection failed".to_string();
            }
        }

        Ok(Response::new(wifi_connect_response))
    }

    async fn remove_network(
        &self,
        request: Request<RemoveNetworkRequest>,
    ) -> Result<Response<RemoveNetworkResponse>, Status> {
        let mut remove_network_response = RemoveNetworkResponse::default();

        log::info!("Starting Remove Network Function");

        let request_data = request.into_inner();

        //get network_id from request
        let network_id = request_data.network_id;

        //get remove_wifi_network from mecha_edge_sdk
        //remove_wifi_network accepts network_id as parameter
        let remove_network = mecha_device_sdk::remove_wifi_network(network_id as usize).await;

        match remove_network {
            Ok(_) => {
                remove_network_response.success = true;
                remove_network_response.message = "WiFi network removed successfully".to_string();
            }
            Err(_) => {
                remove_network_response.success = false;
                remove_network_response.message = "WiFi network removal failed".to_string();
            }
        }

        Ok(Response::new(remove_network_response))
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
