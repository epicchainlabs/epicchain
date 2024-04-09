use std::net::{IpAddr, Ipv4Addr};

/// Struct representing a network device.
#[derive(Debug)]
pub struct NetworkDevice {
    pub ip_address: IpAddr,
    pub port: u16,
    pub device_type: String,
}

impl NetworkDevice {
    /// Creates a new `NetworkDevice` instance with the given IP address, port, and device type.
    pub fn new(ip_address: IpAddr, port: u16, device_type: &str) -> Self {
        NetworkDevice {
            ip_address,
            port,
            device_type: device_type.to_string(),
        }
    }

    /// Returns the IP address of the network device.
    pub fn get_ip_address(&self) -> IpAddr {
        self.ip_address
    }

    /// Returns the port of the network device.
    pub fn get_port(&self) -> u16 {
        self.port
    }

    /// Returns the type of the network device.
    pub fn get_device_type(&self) -> &str {
        &self.device_type
    }

    /// Checks if the network device is online.
    pub fn is_online(&self) -> bool {
        // Simulated check for device online status
        true
    }

    /// Reboots the network device.
    pub fn reboot(&self) {
        // Simulated reboot action
        println!("Rebooting device: {:?}", self);
    }
}

/// Function to discover network devices on the local network.
pub fn discover_network_devices() -> Vec<NetworkDevice> {
    // Simulating discovery of network devices
    vec![
        NetworkDevice::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)), 8080, "Printer"),
        NetworkDevice::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2)), 8080, "Scanner"),
        NetworkDevice::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 3)), 8080, "Camera"),
    ]
}

/// Function to get online network devices from a list of devices.
pub fn get_online_devices(devices: &[NetworkDevice]) -> Vec<&NetworkDevice> {
    devices.iter().filter(|&device| device.is_online()).collect()
}

/// Function to reboot all online network devices.
pub fn reboot_online_devices(devices: &[NetworkDevice]) {
    for device in devices.iter().filter(|&device| device.is_online()) {
        device.reboot();
    }
}

fn main() {
    let devices = discover_network_devices();

    for device in devices.iter() {
        println!("{:?} - {}:{}", device.get_device_type(), device.get_ip_address(), device.get_port());
    }

    let online_devices = get_online_devices(&devices);
    println!("Online devices: {:?}", online_devices);

    reboot_online_devices(&devices);
}
