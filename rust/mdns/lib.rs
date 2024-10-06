pub mod mdns {
    use libc::c_int;

    extern "C" {
        pub fn mdns_init() -> c_int;
        pub fn mdns_query_service();
        // Add more C function bindings as needed
    }

    pub fn init() -> Result<(), String> {
        let result = unsafe { mdns_init() };
        if result == 0 {
            Ok(())
        } else {
            Err("Failed to initialize mDNS".into())
        }
    }

    pub fn query_service() {
        unsafe {
            mdns_query_service();
        }
    }
}
