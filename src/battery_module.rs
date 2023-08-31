use mecha_edge_sdk::battery::{Battery, PowerSupplyInfo};
use tonic::{Request, Response, Status};

use crate::battery::{
    power_supply_service_server::PowerSupplyService, PowerSupplyInfoRequest,
    PowerSupplyInfoResponse,
};

#[derive(Default)]
pub struct PowerSupplyServiceHandler;

#[tonic::async_trait]
impl PowerSupplyService for PowerSupplyServiceHandler {
    async fn get_power_supply_info(
        &self,
        request: Request<PowerSupplyInfoRequest>,
    ) -> Result<Response<PowerSupplyInfoResponse>, Status> {
        // let path = request.into_inner().path;
        let battery = Battery::new();
        let info = battery.info();

        let response = PowerSupplyInfoResponse { info };

        Ok(Response::new(response))
    }
}
