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

    println!("Attempting to install '{}'", args.package);

    network(args.package, "check".to_string());
}

#[tokio::main]
async fn network(package: String, mode: String)
{
    if mode == "content"
    {
        let mut url: String = "http://127.0.0.1:8338/content/".to_string();
        url.push_str(&package);

        let response = reqwest::get(url)
            .await
            .unwrap()
            .text()
            .await;
        
        println!("{:?}", response.unwrap());
    }
    else if mode == "check"
    {
        let mut url: String = "http://127.0.0.1:8338/check/".to_string();
        url.push_str(&package);

        let response = reqwest::get(url)
            .await
            .unwrap()
            .text()
            .await;

        println!("{:?}", response.unwrap());
    }
}
