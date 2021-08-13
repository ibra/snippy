use serde::Deserialize;
use serde::Serialize;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The type of the request [get, shorten]
    request_type: String,
    /// The link to be shortened, or the shortened link to be expanded
    link: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Link {
    id: String,
    short_url: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LinkPost {
    expiration_time: u64,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LinkResponse {
    id: String,
    creation_time: u64,
    expiration_time: u64,
    value: String,
    short_url: String,
}

fn main() {
    let args = Cli::from_args();
    if args.request_type == "get" {
        match get_link_info(&args.link) {
            Ok(_info) => (),
            Err(err) => println!("{:?}", err),
        }
    } else if args.request_type == "shorten" {
        match shorten_link(&String::from(args.link)) {
            Ok(_info) => (),
            Err(err) => println!("{:?}", err),
        }
    }
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

    // Parse the response body as JSON and display cleanly.
    let response = res.json::<Link>().await?;
    println!(
        "{}",
        format!(
            "Found link with id {id}.\nLink URL: {short_url}.\nRedirects to: {value}.",
            id = response.id,
            short_url = response.short_url,
            value = response.value
        )
    );
    Ok(())
}

#[tokio::main]
async fn shorten_link(value: &String) -> Result<(), Box<dyn std::error::Error>> {
    let link = LinkPost {
        expiration_time: 1629464589,
        value: value.to_string(),
    };

    let response = reqwest::Client::new()
        .post("https://beta.sniplink.net/api/v1/link")
        .json(&link)
        .send()
        .await?;

    let json = response.json::<LinkResponse>().await?;
    println!(
        "{}",
        format!(
            "Created link with url {url}.\nLink redirects to: {short_url}.\nCreated At {created}.",
            url = json.short_url,
            short_url = json.value,
            created = json.creation_time
        )
    );
    Ok(())
}
