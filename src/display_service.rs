use display_module::{DisplayInterface, DisplayModule};

use tonic::{Request, Response, Status};

#[allow(non_snake_case)]
pub mod display {
    tonic::include_proto!("display");
}

pub use display::{
    display_service_server::DisplayService, display_service_server::DisplayServiceServer,
    BrightnessRequest, BrightnessResponse, DevicePath, Empty,
};

pub struct DisplayServices {
    pub display: DisplayModule,
}

impl Default for DisplayServices {
    fn default() -> Self {
        DisplayServices {
            display: DisplayModule {
                path: String::default(), // You can specify your default value here
            },
        }
    }
}

#[tonic::async_trait]
impl DisplayService for DisplayServices {
    async fn set_device(&self, request: Request<DevicePath>) -> Result<Response<Empty>, Status> {
        let mut display = DisplayModule {
            path: request.get_ref().path.to_string(),
        };

        let path = request.get_ref().path.to_string();
        display.set_device(&path);

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
