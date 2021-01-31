use ethock_lib::server;

fn main() {
    server::Entry::new("localhost:8545").serve();
}
