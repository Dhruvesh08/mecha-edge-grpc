use env_logger::Env;
use log::{error, info};
use once_cell::sync::OnceCell;
use wifi_ctrl::{
    sta::{self, NetworkResult, RequestClient, ScanResult},
    Result,
};

static LOGGER_INITIALIZED: OnceCell<()> = OnceCell::new();

pub struct WifiManager {
    requester: RequestClient,
}

impl WifiManager {
    pub fn new() -> Result<Self> {
        initialize_logger();
        let mut setup = sta::WifiSetup::new()?;
        let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
        setup.set_socket_path(proposed_path);
        let requester = setup.get_request_client();
        Ok(Self { requester })
    }

    pub async fn shutdown(&self) -> Result<()> {
        self.requester.shutdown().await?;
        Ok(())
    }

    pub async fn get_scan_results(&self) -> Result<Vec<ScanResult>> {
        info!("Requesting scan");
        let scan = self.requester.get_scan().await?;
        info!("Scan complete");
        self.shutdown().await?;
        Ok(scan.to_vec())
    }

    pub async fn get_known_networks(&self) -> Result<Vec<NetworkResult>> {
        info!("Requesting known networks");
        let networks = self.requester.get_networks().await?;
        info!("Known networks retrieved");
        self.shutdown().await?;
        Ok(networks)
    }

    pub async fn connect_to_network(&self, ssid: &str, psk: &str) -> Result<()> {
        let networks = self.get_known_networks().await?;

        for network in networks {
            if network.ssid == ssid {
                info!("Connecting to known network");
                self.requester.select_network(network.network_id).await?;
                return self.shutdown().await;
            }
        }

        info!("Creating new network");
        let network_id = self.requester.add_network().await?;
        self.requester
            .set_network_ssid(network_id, ssid.to_string())
            .await?;
        self.requester
            .set_network_psk(network_id, psk.to_string())
            .await?;
        self.requester.select_network(network_id).await?;
        self.shutdown().await?;
        Ok(())
    }

    pub async fn remove_network(&self, network_id: usize) -> Result<()> {
        info!("Removing network id: {}", network_id);
        self.requester.remove_network(network_id).await?;
        self.shutdown().await?;
        Ok(())
    }
}

async fn broadcast_listener(mut broadcast_receiver: sta::BroadcastReceiver) -> Result<()> {
    while let Ok(broadcast) = broadcast_receiver.recv().await {
        info!("Broadcast: {:?}", broadcast);
    }
    Ok(())
}

fn initialize_logger() {
    LOGGER_INITIALIZED.get_or_init(|| {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    });
}
