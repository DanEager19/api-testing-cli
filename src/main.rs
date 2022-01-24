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
pub async fn run(method:&str, url:&str) -> Result<(), Box<dyn Error>> {
    eprintln!("Fetching {:?}...", url);
    if method == "GET" {
        let res = reqwest::get(url)
            .await?
            .text()
            .await?;

        println!("{:#?}", res);
    };
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let req = Request::new(&args).unwrap_or_else(|err| {
        println!("Problem Parsing arguments: {}", err);
        process::exit(1);
    });

    let url = format!("{}:{}{}", req.uri, req.port, req.route);

    if let Err(e) = run(&req.method, &url) {
        println!("Application Error: {}", e);
        process::exit(1);
    };
}