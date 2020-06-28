use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short)]
    symbol: String,
}