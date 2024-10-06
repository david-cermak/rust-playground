use mdns::mdns;

fn main() {
    if let Err(e) = mdns::init() {
        eprintln!("Error initializing mDNS: {}", e);
        return;
    }

    println!("mDNS initialized successfully.");

    mdns::query_service();
}
