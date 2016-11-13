#[macro_use] extern crate tarpc;
extern crate getopts;
extern crate black_scholes;

use black_scholes::server::*;

fn main() {
    let addr = "127.0.0.1:9000";
    let shutdown = black_scholes::Server.spawn(addr).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(30000));
    shutdown.shutdown();
}
