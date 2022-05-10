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

enum Network {
    Check,
    Content,
}

fn main() {
    let args = Args::parse();

    println!("Attempting to install '{}'", args.package);
    network(args.package);
}

#[tokio::main]
async fn network(package: String, mode: String) -> std::string::String {
    if (mode == "content") {
        let mut url: String = "http://192.168.197.81:8338/content/".to_string();
        url.push_str(&package);

        let response = reqwest::get(url)
            .await
            .unwrap()
            .text()
            .await;

        return response.unwrap();
    }
}
