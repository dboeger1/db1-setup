use clap::{
    Parser,
    Subcommand,
};


#[derive(Parser)]
#[command(name = env!("CARGO_CRATE_NAME"))]
#[command(version)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) subcommand: ArgsSubcommand,
}

#[derive(Subcommand, Hash, PartialEq, Eq)]
pub(crate) enum ArgsSubcommand {
    Git,
    Incus,
    Install,
    Neovim,
    Tmux,
    Ssh,
}
