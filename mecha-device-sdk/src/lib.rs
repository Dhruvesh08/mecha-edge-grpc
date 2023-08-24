use env_logger::Env;
use log::{error, info};
use once_cell::sync::OnceCell;
use wifi_ctrl::{
    sta::{self, NetworkResult, ScanResult},
    Result,
};

static LOGGER_INITIALIZED: OnceCell<()> = OnceCell::new();

pub async fn get_wifi_list() -> Result<Vec<ScanResult>> {
    LOGGER_INITIALIZED.get_or_init(|| {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    });

    info!("Starting All Wifi List Function");

    let mut setup = sta::WifiSetup::new()?;

    let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
    setup.set_socket_path(proposed_path);

    let broadcast = setup.get_broadcast_receiver();
    let requester = setup.get_request_client();
    let runtime = setup.complete();

    let (_runtime, app, _broadcast) = tokio::join!(
        async move {
            if let Err(e) = runtime.run().await {
                error!("Error: {}", e);
            }
        },
        app(requester),
        broadcast_listener(broadcast),
    );

    let wifi_list = app.unwrap();
    Ok(wifi_list)
}

async fn app(requester: sta::RequestClient) -> Result<Vec<ScanResult>> {
    info!("Requesting scan");
    let scan = requester.get_scan().await?;
    info!("Scan complete");
    info!("Shutting down");
    requester.shutdown().await?;
    Ok(scan.to_vec())
}

pub async fn get_known_wifi_list() -> Result<Vec<NetworkResult>> {
    LOGGER_INITIALIZED.get_or_init(|| {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    });
    info!("Starting Known Wifi List Function");

    let mut setup = sta::WifiSetup::new()?;

    let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
    setup.set_socket_path(proposed_path);

    let broadcast = setup.get_broadcast_receiver();
    let requester = setup.get_request_client();
    let runtime = setup.complete();

    let (_runtime, known_wifi, _broadcast) = tokio::join!(
        async move {
            if let Err(e) = runtime.run().await {
                error!("Error: {}", e);
            }
        },
        known_wifi(requester),
        broadcast_listener(broadcast),
    );

    let wifi_list = known_wifi.unwrap();
    Ok(wifi_list)
}

async fn known_wifi(requester: sta::RequestClient) -> Result<Vec<NetworkResult>> {
    info!("Requesting scan");
    let scan = requester.get_networks().await?;
    info!("Scan complete");
    info!("Shutting down");
    requester.shutdown().await?;
    Ok(scan)
}

async fn broadcast_listener(mut broadcast_receiver: sta::BroadcastReceiver) -> Result {
    while let Ok(broadcast) = broadcast_receiver.recv().await {
        info!("Broadcast: {:?}", broadcast);
    }
    Ok(())
}
