enum Methods {
    GET,
    POST,
}

struct Request {
    uri: String,
    route: String,
    port: u32,
    method: Methods
}

impl Request {
    fn method_type(&self.method) {
        match {
            self.method => String::from("GET") = {

            },
            self.method => String::from("POST") = {
                
            },
        }
    }
}

fn main() {
    let req = Request {
        uri: String::from("http://localhost"),
        uri: String::from("/games"),
        port: 5000,
        method: String::from("GET"),
    }
    println!("Hello, world!");
}
