use mecha_edge_sdk::display::{display::DisplayInterface, Display};

use tonic::{Request, Response, Status};

use crate::display::{
    display_service_server::DisplayService, BrightnessRequest, BrightnessResponse, DevicePath,
    Empty,
};
pub struct DisplayServices {
    display: Display,
}

#[tonic::async_trait]
impl DisplayService for DisplayServices {
    async fn set_device(&self, request: Request<DevicePath>) -> Result<Response<Empty>, Status> {
        // self.display.set_device(&request.get_ref().path);
        Ok(Response::new(Empty {}))
    }

    async fn get_device(&self, _request: Request<Empty>) -> Result<Response<DevicePath>, Status> {
        let path = self.display.get_device().to_string();
        let response = DevicePath { path };
        Ok(Response::new(response))
    }

    async fn set_brightness(
        &self,
        request: Request<BrightnessRequest>,
    ) -> Result<Response<Empty>, Status> {
        let brightness = request.get_ref().brightness;
        match self.display.set_brightness(brightness.try_into().unwrap()) {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_brightness(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<BrightnessResponse>, Status> {
        match self.display.get_brightness() {
            Ok(brightness) => {
                let response = BrightnessResponse {
                    brightness: brightness.into(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn info(&self, _request: Request<Empty>) -> Result<Response<Empty>, Status> {
        self.display.info();
        Ok(Response::new(Empty {}))
    }
}
