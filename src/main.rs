
use clap::Parser;
use ureq::{self, Error};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Url to check
    #[arg(short, long)]
    url: String,

    /// Warning percentage used from project quota
    #[arg(short, long, default_value_t = 80)]
    warning: u8,
    
    /// Critical percentage used from project quota
    #[arg(short, long, default_value_t = 90)]
    critical: u8,
}


fn main() -> Result<(), Error> {
    let args = Args::parse();
    let url = args.url;

    let body = ureq::get(url)
        .call()?
        .body_mut()
        .read_to_vec()?;

    for line in body {
         println!("{}", line)
    }

    Ok(())
}
