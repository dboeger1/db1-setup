#[derive(clap::Args, PartialEq, Eq)]
pub(crate) struct Args {
    #[arg(short, long)]
    pub(crate) force: bool,
}
