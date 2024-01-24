use clap::Parser;


#[derive(Parser)]
#[command(name = env!("CARGO_CRATE_NAME"))]
#[command(version)]
pub(crate) struct Args {
    #[arg(short, long)]
    pub(crate) clean: bool,
}
