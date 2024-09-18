use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    pub port: Option<u16>,
}

