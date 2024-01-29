use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, required = true)]
    pub path: PathBuf,

    #[clap(short, long, default_value = "https://f.naa.gg")]
    pub upload_url: String,

    #[clap(short = 't', long, default_value = "")]
    pub upload_token: String,
}
