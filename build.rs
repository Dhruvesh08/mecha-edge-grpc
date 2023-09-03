// use std::{env, path::PathBuf};

// fn main() {
//     let wifi_proto_file = "./proto/wifi.proto";
//     let display_proto_file = "./proto/display.proto";
//     let battery_proto_file = "./proto/battery.proto";

//     tonic_build::configure()
//         .build_server(false)
//         // .out_dir("./src")
//         .compile(
//             &[wifi_proto_file, display_proto_file, battery_proto_file],
//             &["."],
//         )
//         .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

//     // println!(
//     //     "cargo:rerun-if-changed={}",
//     //     wifi_proto_file, display_proto_file
//     // );
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wifi_proto_file = "./proto/wifi.proto";
    let display_proto_file = "./proto/display.proto";
    let battery_proto_file = "./proto/battery.proto";
    let gyro_proto_file = "./proto/gyro.proto";
    tonic_build::configure().build_server(true).compile(
        &[
            wifi_proto_file,
            display_proto_file,
            battery_proto_file,
            gyro_proto_file,
        ],
        &[
            "./proto/wifi",
            "/proto/display",
            "/proto/battery",
            "/proto/gyro",
        ],
    )?;
    Ok(())
}
