// tests/integration_test.rs
use mdns::mdns;

#[test]
fn test_mdns_init() {
    assert!(mdns::init().is_ok());
}
