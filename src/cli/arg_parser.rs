use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    pub symbols: Vec<String>,
}

impl Cli { 
    pub fn from_args() -> Cli { <Cli as StructOpt>::from_args() }
}

