use reqwest::Error;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    request_type: String,
    link: String,
}

#[derive(Deserialize, Debug)]
struct Link {
    id: String,
    shortUrl: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersonResponse {
    data: String,
    method: String,
    headers: HashMap<String, String>,
}

fn main() {
    let args = Cli::from_args();
    if args.request_type == "get" {
        match get_link_info(&args.link) {
            Ok(info) => println!("{:?}", info),
            Err(err) => println!("{:?}", err),
        }
    }
    //check if request type is post or shorten
    // else if args.request_type == "shorten" {
    //     match shorten_link(&args.link) {
    //         Ok(short_link) => println!("{:?}", short_link),
    //         Err(err) => println!("{:?}", err),
    //     }
    // }}
}

#[tokio::main]
async fn get_link_info(link_id: &String) -> Result<(), Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder().build()?;

    // Perform the actual execution of the network request
    let res = client
        .get(format!(
            "https://beta.sniplink.net/api/v1/link/{id}",
            id = link_id
        ))
        .send()
        .await?;

    // Parse the response body as Json in this case
    let response = res.json::<Link>().await?;
    println!(
        "{}",
        format!(
            "Found link with id {id}.\nLink URL: {short_url}\nRedirects to: {value}",
            id = response.id,
            short_url = response.shortUrl,
            value = response.value
        )
    );
    Ok(())
}

// #[tokio::main]
// async fn shorten_link(link_id: &String) -> Result<(), Box<dyn std::error::Error>> {
//     // Build the client using the builder pattern
//     let client = reqwest::Client::builder().build()?;
//     // Perform the actual execution of the network request
//     // let res = client
//     //     .post(format!(""
// }
