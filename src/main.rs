use hyper::{Client, Uri};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    r#type: String,
    link: String,
}

fn main() {
    let args = Cli::from_args();
}

async fn get_link_info() {
    let client = Client::new();

    let url: Uri = "http://httpbin.org/response-headers?foo=bar"
        .parse()
        .unwrap();

    match client.get(url).await {
        Ok(res) => println!("Response: {}", res.status()),
        Err(err) => println!("Error: {}", err),
    }
}
