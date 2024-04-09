use std::net::{IpAddr, Ipv4Addr};

/// Represents a network device.
#[derive(Debug)]
struct NetworkDevice {
    ip_address: IpAddr,
    port: u16,
    name: String,
}

impl NetworkDevice {
    /// Creates a new `NetworkDevice` instance.
    fn new(ip_address: IpAddr, port: u16, name: String) -> Self {
        NetworkDevice {
            ip_address,
            port,
            name,
        }
    }
}

/// Function to discover network devices.
fn discover_devices(subnet: &str) -> Vec<NetworkDevice> {
    // Simulate discovering devices in the subnet
    let devices = vec![
        NetworkDevice::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), 8080, "Device1".to_string()),
        NetworkDevice::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), 8080, "Device2".to_string()),
        NetworkDevice::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 3)), 8080, "Device3".to_string()),
    ];

    devices
}

/// Function to connect to a network device.
fn connect_to_device(device: &NetworkDevice) {
    // Simulate connecting to the device
    println!("Connecting to {} at {}:{}...", device.name, device.ip_address, device.port);
    // Add your connection logic here
}

fn main() {
    let subnet = "192.168.1";
    let devices = discover_devices(subnet);

    for device in devices {
        connect_to_device(&device);
    }
}
