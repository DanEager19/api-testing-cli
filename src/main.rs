use std::error::Error;
use std::env;
use std::process;
use futures::stream::TryStreamExt;
use reqwest::{Body};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

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
pub async fn post_request(url:&str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending POST request to {:?}...", url);
    let file = File::open("sample.json").await?;

    let client = reqwest::Client::new();
    let _res = client
        .post(url)
        .body(file_to_body(file))
        .send()
        .await?;

    Ok(())
}

#[tokio::main]
pub async fn get_request(url:&str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending GET request to {:?}...", url);

    let res = reqwest::get(url)
        .await?
        .text()
        .await?;

    println!("{:?}", res);

    Ok(())
}

fn file_to_body(file: File) -> Body {
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    body
}

pub fn run(method:&str, url:&str) -> Result<(), Box<dyn Error>> {
    if method == "GET" {
        if let Err(e) = get_request(url) {
            println!("Application Error: {}", e);
            process::exit(1);
        };
    } else if method == "POST" {
        if let Err(e) = post_request(url) {
            println!("Application Error: {}", e);
            process::exit(1);
        };
    } else {
        eprintln!("No method given!");
        process::exit(1);
    }

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