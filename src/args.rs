use clap::Parser;

#[derive(Parser)]
#[clap(version, about)]
pub struct args {
    
    #[clap(short, long)]
    pub boostrap: Option<String>,

    #[clap(short, long)]
    pub url: String,

}

pub fn parse_args() -> Args {
    Args::parse()
}