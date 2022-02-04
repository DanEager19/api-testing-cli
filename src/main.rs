use text_io::read;
use std::error::Error;
use std::process;
use reqwest;
use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
pub struct Request {
    uri: String,
    route: String,
    port: String,
    method: String
}

#[tokio::main]
pub async fn get_request(url:&str) -> Result<(), Box<dyn Error>> {
    println!("Sending GET request to {:?}...", url);
    let client = reqwest::Client::new();

    let res = client.get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await?;
    
    println!("{:?}", res);
    
    Ok(())
}

#[tokio::main]
pub async fn post_request(url:&str) -> Result<(), Box<dyn Error>> {
    let mut data = HashMap::new();
    let mut i: i32 = 0;

    println!("How many data entries?");
    let loop_count: i32 = read!();

    while i < loop_count {
        println!("Enter the name: ");
        let name: String = read!();
        println!("Enter the value: ");
        let value: String = read!();

        data.insert(name, value);
        i += 1;
    }
    println!("Sending POST request to {:?}...", url);
    let client = reqwest::Client::new();
    let res = client.post(url)
        .header("CONTENT_TYPE", "application/json")
        .json(&data)
        .send()
        .await?;
        
    println!("{:?}", res);

    Ok(())
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
    let req = Request::parse();

    let url = format!("{}:{}{}", req.uri, req.port, req.route);

    if let Err(e) = run(&req.method, &url) {
        println!("Application Error: {}", e);
        process::exit(1);
    };
}