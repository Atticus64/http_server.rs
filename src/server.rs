use std::result;
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
pub struct Server<'a> {
  ip: &'a str,
  port: i32,
  pub routes: Vec<(String, String)>,
  listener: Option<TcpListener>
}


impl Server<'_> {

    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&stream);
        let request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let method_info = request.get(0).unwrap().split(" ").clone();

        let header: Vec<&str> = method_info.collect();
        let method = header[0];
        let route = header[1];
        let http_version = header[2];

        for data_route in &self.routes {
            let route_candidate = &data_route.0;
            if &route.to_string() == route_candidate {
                let mut response: String  = String::from("HTTP/1.1 200 ");
                let contents = &data_route.1;
                let len = contents.len();
                response += &data_route.1.to_string();
                response += "\r\n";
                let body = format!("Content-Length: {len}\r\n\r\n{contents}");
                response += body.as_str();
                stream.write_all(response.as_bytes()).unwrap();
                break;
            }

        }

        let mut response: String  = String::from("HTTP/1.1 500 ");
        let contents = String::from("{ \"error\": \"Not route found\", \"status\": \":(\" }");
        let len = contents.len();
        response += "Server Error";
        response += "\r\n";
        let body = format!("Content-Type: application/json\r\nContent-Length: {len}\r\nContent-Encoding: utf-8\r\n\r\n{contents}");
        response += body.as_str();
        stream.write_all(response.as_bytes()).unwrap();

        println!("HTTP METHOD: {}", method);
        println!("HTTP route: {}", route);
        println!("HTTP version: {}", http_version);
        println!("request -> {:?}", request);
    }

    pub fn new(port: i32) -> Self {
        let srv = Server {
            ip: "127.0.0.1",
            port: port,
            routes: vec![],
            listener: None
        };

        srv
    }

    pub fn create(&mut self) {
        let ip = self.ip;
        let port = self.port.to_string();
        let mut location = String::new();

        location += ip;
        location += ":";
        location += port.as_str();

        
        self.listener = Some(TcpListener::bind(location).unwrap())
    }

    pub fn add_route(&mut self, route_raw: &str, data_raw: &str) {
        let route = route_raw.to_string();
        let data = data_raw.to_string();
        self.routes.push((route, data))
    }

    pub fn listen(&self) -> Result<(), &str> {
        if self.listener.is_none() {
            return Err("No server created"); 
        }

        let streaming = self.listener.as_ref();

        for stream in streaming.unwrap().incoming() {
            let stream = stream.unwrap();

            self.handle_connection(stream);
        }

        Ok(())
    }
}


