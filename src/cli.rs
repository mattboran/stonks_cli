use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    pub symbol: String,
}

impl Cli { 
    pub fn from_args() -> Cli { <Cli as StructOpt>::from_args() }
}

