use crate::wifi::{
    wifi_server::Wifi, Empty, NetworkResult, NetworkResults, RemoveNetworkRequest,
    RemoveNetworkResponse, ScanResult, ScanResults, WifiConnectRequest, WifiConnectResponse,
};

use mecha_edge_sdk::wifi::WifiModule;
use tonic::{Request, Response, Status};

const WIFI_CONNECT_SUCCESS_MESSAGE: &str = "WiFi connection successful";
const WIFI_CONNECT_FAILURE_MESSAGE: &str = "WiFi connection failed";
const NETWORK_REMOVAL_SUCCESS_MESSAGE: &str = "WiFi network removed successfully";
const NETWORK_REMOVAL_FAILURE_MESSAGE: &str = "WiFi network removal failed";

#[derive(Default)]
pub struct WifiService {}
trait ResponseMessage {
    fn set_success(&mut self, success: bool);
    fn set_message(&mut self, message: String);
}

impl ResponseMessage for WifiConnectResponse {
    fn set_success(&mut self, success: bool) {
        self.success = success;
    }

    fn set_message(&mut self, message: String) {
        self.message = message;
    }
}

impl ResponseMessage for RemoveNetworkResponse {
    fn set_success(&mut self, success: bool) {
        self.success = success;
    }

    fn set_message(&mut self, message: String) {
        self.message = message;
    }
}

impl WifiService {
    fn handle_response<T: ResponseMessage>(
        &self,
        result: Result<(), &str>,
        response: &mut T,
        success_message: &str,
        failure_message: &str,
    ) {
        match result {
            Ok(_) => {
                response.set_success(true);
                response.set_message(success_message.to_string());
            }
            Err(_) => {
                response.set_success(false);
                response.set_message(failure_message.to_string());
            }
        }
    }

    async fn connect_to_wifi(&self, ssid: &str, psk: &str) -> Result<(), &str> {
        let connect_wifi = WifiModule::get_connect_wifi(ssid, psk).await;

        match connect_wifi {
            Ok(_) => Ok(()),
            Err(_) => Err(WIFI_CONNECT_FAILURE_MESSAGE),
        }
    }

    async fn remove_wifi_network(&self, network_id: usize) -> Result<(), &str> {
        let remove_network = WifiModule::remove_wifi_network(network_id).await;

        match remove_network {
            Ok(_) => Ok(()),
            Err(_) => Err(NETWORK_REMOVAL_FAILURE_MESSAGE),
        }
    }
}

#[tonic::async_trait]
impl Wifi for WifiService {
    async fn get_wifi(&self, _request: Request<Empty>) -> Result<Response<ScanResults>, Status> {
        // Implement your async get_wifi logic here
        let mut scan_results = ScanResults::default();

        log::info!("Starting All Wifi List Function");
        let wifi_service = WifiModule::new();

        //get wifi list from mecha_edge_sdk
        let wifi_list = wifi_service.get_wifi_list().await.unwrap();
        //add wifi list to scan_results
        for wifi in wifi_list {
            let scan_result = ScanResult {
                mac: wifi.mac,
                frequency: wifi.frequency,
                signal: wifi.signal as i32,
                flags: wifi.flags,
                name: wifi.name,
            };
            scan_results.results.push(scan_result);
        }

        Ok(Response::new(scan_results))
    }

    async fn get_known_wifi(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<NetworkResults>, Status> {
        // Implement your async get_known_wifi logic here
        let mut scan_results = NetworkResults::default();
        log::info!("Starting Known Wifi List Function");

        //get wifi list from mecha_edge_sdk
        let wifi_list = WifiModule::get_known_wifi_list().await.unwrap();
        //add wifi list to scan_results
        for wifi in wifi_list {
            let scan_result = NetworkResult {
                network_id: wifi.network_id as i32,
                flags: wifi.flags,
                ssid: wifi.ssid,
            };
            scan_results.results.push(scan_result);
        }

        Ok(Response::new(scan_results))
    }

    async fn wifi_connect(
        &self,
        request: Request<WifiConnectRequest>,
    ) -> Result<Response<WifiConnectResponse>, Status> {
        let mut wifi_connect_response = WifiConnectResponse::default();
        let request_data = request.into_inner();

        self.handle_response(
            self.connect_to_wifi(&request_data.ssid, &request_data.psk)
                .await,
            &mut wifi_connect_response,
            WIFI_CONNECT_SUCCESS_MESSAGE,
            WIFI_CONNECT_FAILURE_MESSAGE,
        );

        Ok(Response::new(wifi_connect_response))
    }

    async fn remove_network(
        &self,
        request: Request<RemoveNetworkRequest>,
    ) -> Result<Response<RemoveNetworkResponse>, Status> {
        let mut remove_network_response = RemoveNetworkResponse::default();
        let request_data = request.into_inner();

        self.handle_response(
            self.remove_wifi_network(request_data.network_id as usize)
                .await,
            &mut remove_network_response,
            NETWORK_REMOVAL_SUCCESS_MESSAGE,
            NETWORK_REMOVAL_FAILURE_MESSAGE,
        );

        Ok(Response::new(remove_network_response))
    }
}
