use clap::{Parser, ValueEnum, ArgAction};

#[derive(Parser)]
#[clap(version)]
pub struct AppCommands {
    #[clap(index = 1, help = "base path to search")]
    pub path: String,

    #[clap(index = 2, help = "regex pattern, its recommended to wrap it around single quotes(')")]
    pub pattern: String,

    #[clap(short = 'r', long, action = ArgAction::SetTrue, help = "show anything that doesnt match the patterns")]
    pub reverse: bool,

    #[clap(short = 's', default_value_t = Target::Names, value_enum, help = "search target, can either be name of the files/directories or file contents")]
    pub target: Target,

    #[clap(short = 't', default_value = "0", help = "set max thread count")]
    pub max_thread: usize,

    #[clap(short = 'd', default_value = "3", help = "max directory depth to search")]
    pub max_depth: usize,
}

#[derive(Clone, ValueEnum)]
pub enum Target {
    Names,
    Contents,
}