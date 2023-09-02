use battery_module::{BatteryModule, PowerInfo, PowerSupplyInfo};
use tonic::{Request, Response, Status};

pub struct BatteryService {
    pub battery: BatteryModule,
}

#[allow(non_snake_case)]
pub mod battery {
    tonic::include_proto!("battery");
}

pub use battery::{
    power_supply_service_server::PowerSupplyService,
    power_supply_service_server::PowerSupplyServiceServer, PowerSupplyInfoRequest,
    PowerSupplyInfoResponse,
};

impl Default for BatteryService {
    fn default() -> Self {
        BatteryService {
            battery: BatteryModule::new(),
        }
    }
}

#[tonic::async_trait]
impl PowerSupplyService for BatteryService {
    async fn get_power_supply_info(
        &self,
        request: Request<PowerSupplyInfoRequest>,
    ) -> Result<Response<PowerSupplyInfoResponse>, Status> {
        // let path = request.into_inner().path;
        let battery = BatteryModule::new();
        let info = battery.info();

        let response = PowerSupplyInfoResponse { info };

        Ok(Response::new(response))
    }
}
