mod server;
use server::Server;

struct User {
  name: String,
  id: u8
}


fn main() {
    let mut srv = Server::new(3000);

    srv.create();

    srv.add_route("/home", "Home!");
    srv.add_route("/login", "Login");
    srv.add_route("/logout", "Logout");
    println!("{:?}", srv);

    srv.listen().unwrap();
}
