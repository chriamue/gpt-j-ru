use gptj;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(
        short = "u",
        long = "url",
        default_value = "http://api.vicgalle.net:5000"
    )]
    url: String,

    #[structopt(short = "length", long = "token_max_length", default_value = "200")]
    token_max_length: u16,

    #[structopt(short = "temp", long = "temperature", default_value = "0.9")]
    temperature: f32,

    #[structopt(short = "p", long = "top_p", default_value = "0.9")]
    top_p: f32,

    #[structopt(short = "stop", long = "stop_sequence")]
    stop_sequence: Option<String>,

    context: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let gpt = gptj::GPT { url: opt.url };
    let response = gpt
        .generate(
            opt.context,
            opt.token_max_length,
            opt.temperature,
            opt.top_p,
            opt.stop_sequence,
        )
        .await
        .unwrap();
    println!("{}", response.text);
}
