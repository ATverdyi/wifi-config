use dbus::{
    arg::Variant,
    blocking::{stdintf::org_freedesktop_dbus::Properties, Connection},
};
use std::{collections::HashMap, time::Duration};

/// Sends Wi-Fi parameters to NetworkManager for connection
pub fn send_wifi_to_network_manager(ssid: &str, password: &str) {
    let conn = Connection::new_system().unwrap();
    let proxy = conn.with_proxy(
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        Duration::from_secs(10),
    );
    let connection_path = dbus::Path::new("/").unwrap();
    let (devices,): (Vec<dbus::Path>,) = proxy
        .method_call("org.freedesktop.NetworkManager", "GetDevices", ())
        .unwrap();

    let mut wifi_device_path: Option<dbus::Path> = None;

    for device in devices {
        let device_proxy = conn.with_proxy(
            "org.freedesktop.NetworkManager",
            &device,
            Duration::from_secs(10),
        );
        let device_type: u32 = device_proxy
            .get("org.freedesktop.NetworkManager.Device", "DeviceType")
            .unwrap();

        // type 2 means Wi-Fi
        if device_type == 2 {
            wifi_device_path = Some(device);
            break;
        }
    }

    if wifi_device_path.is_none() {
        eprintln!("Wi-Fi device not found.");
        return;
    }

    let device_path = wifi_device_path.unwrap();
    // Wi-Fi configuration structure
    let mut connection_settings: HashMap<&str, HashMap<&str, Variant<Box<dyn dbus::arg::RefArg>>>> =
        HashMap::new();

    // Wi-Fi settings
    let mut wifi_settings: HashMap<&str, Variant<Box<dyn dbus::arg::RefArg>>> = HashMap::new();
    wifi_settings.insert("ssid", Variant(Box::new(ssid.as_bytes().to_vec())));
    wifi_settings.insert("mode", Variant(Box::new(String::from("infrastructure"))));
    connection_settings.insert("802-11-wireless", wifi_settings);

    // Wi-Fi security settings
    let mut wifi_security: HashMap<&str, Variant<Box<dyn dbus::arg::RefArg>>> = HashMap::new();
    wifi_security.insert("key-mgmt", Variant(Box::new(String::from("wpa-psk"))));
    wifi_security.insert("psk", Variant(Box::new(String::from(password))));
    connection_settings.insert("802-11-wireless-security", wifi_security);

    let result: Result<(), _> = proxy.method_call(
        "org.freedesktop.NetworkManager",
        "AddAndActivateConnection",
        (connection_settings, device_path, connection_path),
    );

    match result {
        Ok(_) => println!("Wi-Fi configuration successfully sent."),
        Err(e) => eprintln!("Failed to configure Wi-Fi: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        assert!(true);
    }
}
