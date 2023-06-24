mod handler;
mod router;
mod server;

use server::{self, Server}:Server;

fn main() {
    let server = Server::new("localhost:3000");
    server.run();
}
