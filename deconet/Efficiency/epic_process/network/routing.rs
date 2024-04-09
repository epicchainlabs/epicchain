use std::collections::HashMap;

/// Struct representing a routing table entry.
#[derive(Debug)]
pub struct RoutingEntry {
    pub destination: String,
    pub gateway: String,
    pub interface: String,
}

/// Struct representing a routing table.
pub struct RoutingTable {
    entries: HashMap<String, RoutingEntry>,
}

impl RoutingTable {
    /// Creates a new `RoutingTable` instance.
    pub fn new() -> Self {
        RoutingTable {
            entries: HashMap::new(),
        }
    }

    /// Adds a new routing table entry.
    pub fn add_entry(&mut self, destination: &str, gateway: &str, interface: &str) {
        let entry = RoutingEntry {
            destination: destination.to_string(),
            gateway: gateway.to_string(),
            interface: interface.to_string(),
        };
        self.entries.insert(destination.to_string(), entry);
    }

    /// Removes a routing table entry.
    pub fn remove_entry(&mut self, destination: &str) {
        self.entries.remove(destination);
    }

    /// Clears all entries from the routing table.
    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    /// Gets the routing table entry for a given destination.
    pub fn get_entry(&self, destination: &str) -> Option<&RoutingEntry> {
        self.entries.get(destination)
    }

    /// Gets all entries in the routing table.
    pub fn get_entries(&self) -> Vec<&RoutingEntry> {
        self.entries.values().collect()
    }
}

fn main() {
    let mut routing_table = RoutingTable::new();

    // Add routing table entries
    routing_table.add_entry("192.168.1.0", "192.168.1.1", "eth0");
    routing_table.add_entry("10.0.0.0", "10.0.0.1", "eth1");

    // Get and print routing table entries
    for entry in routing_table.get_entries() {
        println!("{:?}", entry);
    }

    // Remove a routing table entry
    routing_table.remove_entry("192.168.1.0");

    // Clear all entries from the routing table
    routing_table.clear_entries();
}

