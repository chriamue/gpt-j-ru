use gptj;

use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
enum Command {
    Generate {
        #[structopt(short = "length", long = "token_max_length", default_value = "200")]
        token_max_length: u16,

        #[structopt(short = "temp", long = "temperature", default_value = "0.9")]
        temperature: f32,

        #[structopt(short = "p", long = "top_p", default_value = "0.9")]
        top_p: f32,

        #[structopt(short = "stop", long = "stop_sequence")]
        stop_sequence: Option<String>,

        context: String,
    },
    Classify {
        #[structopt(required = true)]
        labels: Vec<String>,
        sequence: String,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(subcommand)]
    command: Option<Command>,

    #[structopt(
        short = "u",
        long = "url",
        default_value = "http://api.vicgalle.net:5000"
    )]
    url: String,

    context: Option<String>,
    
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let gpt = gptj::GPT { url: opt.url };
    match opt.command {
        Some(Command::Generate {
            token_max_length,
            temperature,
            top_p,
            stop_sequence,
            context
        }) => {
            let response = gpt
                .generate(
                    context,
                    token_max_length,
                    temperature,
                    top_p,
                    stop_sequence,
                )
                .await
                .unwrap();
            println!("{}", response.text);
        }
        Some(Command::Classify {labels, sequence}) => {
            let response = gpt
            .classify(
                &labels,
                sequence
            )
            .await
            .unwrap();
        println!("{:?}", response);
        },
        None => {
            let response = gpt
                .generate(
                    opt.context.unwrap_or_default(),
                    200,
                    0.9,
                    0.9,
                    None,
                )
                .await
                .unwrap();
            println!("{}", response.text);
        }
    }
}
