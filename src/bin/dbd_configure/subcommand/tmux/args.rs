#[derive(clap::Args, PartialEq, Eq)]
pub(crate) struct Args {
    #[arg(short, long)]
    force: bool,
}
