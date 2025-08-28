use wifi_config::send_wifi_to_network_manager;

/// Simple CLI wrapper for the `wifi_configurator` library.
///
/// Usage:
/// ```sh
/// wifi-config <SSID> <PASSWORD>
/// ```
///
/// - Initializes logger
/// - Reads SSID and password from CLI args
/// - Calls [`send_wifi_to_network_manager`]

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: wifi-config <SSID> <PASSWORD>");
        std::process::exit(1);
    }

    let ssid = &args[1];
    let password = &args[2];
    send_wifi_to_network_manager(ssid, password);
    Ok(())
}
