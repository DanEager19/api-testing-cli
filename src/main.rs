
use std::error::Error;
use std::env;
use std::process;

pub struct Request {
    uri: String,
    route: String,
    port: String,
    method: String
}

impl Request {
    pub fn new(args: &[String]) -> Result<Request, &str> {
        if args.len() < 4 {
            return Err("Insufficent Arguments!");
        }
        let uri = args[1].clone();
        let route = args[2].clone();
        let port = args[3].clone();
        let method = args[4].clone();
        
        Ok(Request { uri, route, port, method })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let req = Request::new(&args).unwrap_or_else(|err| {
        println!("Problem Parsing arguments: {}", err);
        process::exit(1);
    });

    let full_uri = format!("{}:{}{}", req.uri, req.port, req.route);
    
    if req.method == "GET" {
        let res = reqwest::get(full_uri)
            .await?
            .text()
            .await?;

        println!("{:#?}", res);
    };

    Ok(())
}