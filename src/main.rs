use chrono::prelude::*;
use chrono::{Duration, Utc};
use clipboard_win::{formats, set_clipboard};
use serde::Deserialize;
use serde::Serialize;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The type of the request [get, shorten]
    request_type: String,
    /// The link to be shortened, or the shortened link to be expanded
    link: String,

    /// The expiration duration of the link in days.
    #[structopt(
        short = "d",
        long = "duration",
        default_value = "7",
        required_if("request_type", "get")
    )]
    duration: u16,

    ///Make it so the link isn't copied to your clipboard   
    #[structopt(short = "nc", long = "nocopy")]
    no_copy: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Link {
    id: String,
    short_url: String,
    value: String,
    expiration_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LinkPost {
    expiration_time: i64,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LinkResponse {
    id: String,
    creation_time: i64,
    expiration_time: i64,
    value: String,
    short_url: String,
}

fn main() {
    let args = Cli::from_args();
    if args.request_type == "get" {
        match get_link_info(&args.link, &args.no_copy) {
            Ok(_info) => (),
            Err(err) => println!("{:?}", err),
        }
    } else if args.request_type == "shorten" {
        match shorten_link(
            &String::from(args.link),
            args.duration.into(),
            &args.no_copy,
        ) {
            Ok(_info) => (),
            Err(err) => println!("{:?}", err),
        }
    }
}

#[tokio::main]
async fn get_link_info(link_id: &String, no_copy: &bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let res = client
        .get(format!(
            "https://beta.sniplink.net/api/v1/link/{id}",
            id = link_id
        ))
        .send()
        .await?;

    let response = res.json::<Link>().await?;
    let creation_date = timestamp_to_date(response.expiration_time).format("%d-%m-%Y %H:%M:%S");

    println!(
        "{}",
        format!(
            "Found link with id {id}.\nLink URL: {short_url}.\nRedirects to: {value}.\nCreated at: {date}",
            id = response.id,
            short_url = response.short_url,
            value = response.value,
            date = creation_date
        )
    );

    if !no_copy {
        set_clipboard(formats::Unicode, response.value).expect("Failed when copying to clipboard!");
        println!("Copied Link To Clipboard! (Disable with --nocopy)");
    }
    Ok(())
}

#[tokio::main]
async fn shorten_link(
    value: &String,
    duration: i64,
    no_copy: &bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let link = LinkPost {
        expiration_time: (Utc::now() + Duration::days(duration)).timestamp(),
        value: value.to_string(),
    };

    let response = reqwest::Client::new()
        .post("https://beta.sniplink.net/api/v1/link")
        .json(&link)
        .send()
        .await?;

    let post_response = response.json::<LinkResponse>().await?;

    let creation_date = timestamp_to_date(post_response.creation_time).format("%d-%m-%Y %H:%M:%S");
    let expiration_date =
        timestamp_to_date(post_response.expiration_time).format("%d-%m-%Y %H:%M:%S");

    println!(
        "{}",
        format!(
            "Created link with url {url}.\nLink redirects to: {short_url}.\nCreated At {created}.\nLink expires at {expiry}",
            url = post_response.short_url,
            short_url = post_response.value,
            created = creation_date,
            expiry = expiration_date
        )
    );
    if !no_copy {
        set_clipboard(formats::Unicode, post_response.short_url)
            .expect("Failed when copying to clipboard!");
        println!("Copied Link To Clipboard! (Disable with --nocopy)");
    }
    Ok(())
}

fn timestamp_to_date(timestamp: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    datetime
}
