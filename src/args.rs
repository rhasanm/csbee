use std::path::PathBuf;
use clap::Parser;


#[derive(Parser, Debug)]
pub struct Args {
    #[arg(required=true)]
    pub filename: PathBuf,
}