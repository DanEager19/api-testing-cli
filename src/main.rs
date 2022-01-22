enum Methods {
    GET,
    POST,
}

struct Request {
    uri: String,
    route: String,
    port: i32,
    method: Methods
}

impl Request {
    fn method_type(&self.method) {
        match {
            self.method => String::from("GET") = {
                handle_response();
            },
            self.method => String::from("POST") = {
                handle_response();
            },
        }
    }
}

//CLI: api-test -p 5000 -u http://localhost -r /games -get
fn main() {
    let req = Request {
        uri: String::from("http://localhost/"),
        route: String::from("/games"),
        port: 5000,
        method: String::from("GET"),
    }
    println!("Hello, world!");
}


//read JSON response into a tmp txt file?
fn handle_response() -> Result<String, io::Error> {
    let f = std::fs::File::open("tmp.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    }
}