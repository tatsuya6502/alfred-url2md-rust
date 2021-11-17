use clap::{ColorChoice, Parser};
mod process;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Shun Namiki aka Nash <snamiki1212@gmail.com>"
)]
#[clap(color = ColorChoice::Always)]
struct Opts {
    input: String,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    let result = process::url2md(&opts.input).await;
    match result {
        Ok(title) => println!("{} ", title),
        Err(_err) => {}
    }
}
