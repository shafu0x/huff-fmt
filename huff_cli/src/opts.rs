use clap::{Args, Parser, ValueHint};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[command(flatten)]
    pub inputs: Inputs,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct Inputs {
    /// Path to huff contract
    #[clap(value_hint = ValueHint::FilePath, value_name = "PATH")]
    #[arg(short, long)]
    pub path: Option<String>,

    /// Input string
    pub input: Option<String>,
}
