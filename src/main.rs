mod server;
use server::Server;

struct User {
  name: String,
  id: u8
}


fn main() {
    let mut srv = Server::new(3000);

    srv.create();
    let content = "<html> <head> <title>Mi pagina web</title></head><body><h1>Hi http server in Rust</h1></body></html>";

    srv.add_route("/home", content);
    srv.add_route("/login", "Login");
    srv.add_route("/logout", "Logout");
    println!("{:?}", srv);

    srv.listen().unwrap();
}
