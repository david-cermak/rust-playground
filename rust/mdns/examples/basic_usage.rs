// examples/basic_usage.rs

use mdns;

fn main() {
    // Initialize mDNS
    mdns::mdns_init();

    // Query for a specific host
    mdns::mdns_query_host_rust("example.local");

    // Deinitialize mDNS
    mdns::mdns_deinit();
}
