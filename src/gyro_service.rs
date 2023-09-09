use gyroscope_module::GyroscopeModule;

use tonic::{Request, Response, Status};

#[allow(non_snake_case)]
pub mod gyro {
    tonic::include_proto!("gyro");
}

pub use gyro::{
    MotionValueResponse,
    {
        gyro_service_server::{GyroService, GyroServiceServer},
        Empty,
    },
};

#[derive(Default)]
pub struct GyroServices;

#[tonic::async_trait]
impl GyroService for GyroServices {
    async fn get_motion_values(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<MotionValueResponse>, Status> {
        //create new gyro object
        let gyro = GyroscopeModule::new(
            "/sys/bus/iio/devices/iio:device1/in_anglvel_x_raw",
            "/sys/bus/iio/devices/iio:device1/in_anglvel_y_raw",
            "/sys/bus/iio/devices/iio:device1/in_anglvel_z_raw",
        );

        //read axis values
        let (x, y, z) = gyro.read_axis().unwrap();

        let response = MotionValueResponse {
            x_axis: x,
            y_axis: y,
            z_axis: z,
        };

        Ok(Response::new(response))
    }
}
