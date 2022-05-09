use clap::Parser;
use reqwest;

#[derive(clap::Parser)]
struct Args {
   #[clap(subcommand)]
    mode: Mode,

    package: String,
}

#[derive(clap::Subcommand, Debug)]
enum Mode {
   Install,
   Remove,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.mode);
    println!("{}", args.package);

    network();
}

#[tokio::main]
async fn network() {
    let response = reqwest::get("http://10.201.102.67:8338")
        .await
        // each response is wrapped in a `Result` type
        // we'll unwrap here for simplicity
        .unwrap()
        .text()
        .await;
    println!("{}", response.unwrap());
}
