use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Struct representing network configuration settings.
pub struct NetworkConfig {
    pub ip_address: IpAddr,
    pub port: u16,
}

impl NetworkConfig {
    /// Creates a new `NetworkConfig` instance with default settings.
    pub fn new() -> Self {
        NetworkConfig {
            ip_address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
        }
    }

    /// Sets the IP address for the network configuration.
    pub fn set_ip_address(&mut self, ip_address: IpAddr) {
        self.ip_address = ip_address;
    }

    /// Sets the port for the network configuration.
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    /// Returns the socket address based on the IP address and port.
    pub fn get_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip_address, self.port)
    }

    /// Returns the IP address from the network configuration.
    pub fn get_ip_address(&self) -> IpAddr {
        self.ip_address
    }

    /// Returns the port from the network configuration.
    pub fn get_port(&self) -> u16 {
        self.port
    }

    /// Resets the network configuration to default settings.
    pub fn reset_to_default(&mut self) {
        self.ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        self.port = 8080;
    }
}

fn main() {
    let mut config = NetworkConfig::new();
    println!("Default IP address: {:?}", config.get_ip_address());
    println!("Default port: {:?}", config.get_port());

    config.set_ip_address(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    config.set_port(9090);

    println!("Updated IP address: {:?}", config.get_ip_address());
    println!("Updated port: {:?}", config.get_port());

    let socket_addr = config.get_socket_addr();
    println!("Socket address: {:?}", socket_addr);

    config.reset_to_default();
    println!("Reset to default IP address: {:?}", config.get_ip_address());
    println!("Reset to default port: {:?}", config.get_port());
}
