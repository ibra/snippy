use futures::executor::block_on;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    request_type: String,
    link: String,
}

fn main() {
    let args = Cli::from_args();
    if args.request_type == "get" {
        let link_info = get_link_info(&args.link);
        block_on(link_info);
    }
}

#[tokio::main]
async fn get_link_info(link_id: &String) {
    println!("Link id is {} ", link_id);
}
