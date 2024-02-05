#[derive(clap::Args, PartialEq, Eq)]
pub(crate) struct Args {
    #[arg(short, long)]
    comment: Option<String>,

    #[arg(short, long)]
    force: bool,
}
