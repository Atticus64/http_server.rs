mod server;
mod utils;
mod data;
use server::Server;
use crate::data::User;

fn main() {
    let mut srv = Server::new(3000);

    srv.create();
    let content = "<html> <head> <title>Mi pagina web</title></head><body><h1>Hi http server in Rust</h1></body></html>";
    let usuario = User::new("Pancho");

    srv.add_route("/home", content);
    srv.add_route("/login", "Login");
    srv.add_route("/logout", "Logout");
    srv.add_route("/user", &usuario.to_json()); 

    srv.listen().unwrap();
}
