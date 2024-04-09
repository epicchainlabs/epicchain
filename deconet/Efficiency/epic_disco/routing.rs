use std::net::IpAddr;

/// Represents a network route.
#[derive(Debug)]
struct Route {
    destination: IpAddr,
    gateway: Option<IpAddr>,
    interface: String,
}

impl Route {
    /// Creates a new `Route` instance.
    fn new(destination: IpAddr, gateway: Option<IpAddr>, interface: String) -> Self {
        Route {
            destination,
            gateway,
            interface,
        }
    }
}

/// Function to add a new route to the routing table.
fn add_route(destination: IpAddr, gateway: Option<IpAddr>, interface: &str) -> Route {
    let route = Route::new(destination, gateway, interface.to_string());
    // Add the route to the routing table (simulated here)
    println!("Added route: {:?}", route);
    route
}

/// Function to remove a route from the routing table.
fn remove_route(destination: IpAddr) {
    // Remove the route from the routing table (simulated here)
    println!("Removed route to {:?}", destination);
}

fn main() {
    let destination = IpAddr::V4("192.168.1.0".parse().unwrap());
    let gateway = Some(IpAddr::V4("192.168.1.1".parse().unwrap()));
    let interface = "eth0";

    let route = add_route(destination, gateway, interface);
    // Perform routing logic here

    remove_route(destination);
}
