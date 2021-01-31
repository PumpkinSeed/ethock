fn main() {
    ethock_lib::server::Entry::new("localhost:8545").serve_silent();
}