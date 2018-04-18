#[derive(Debug, StructOpt)]
#[structopt(raw(global_setting = "::structopt::clap::AppSettings::ColoredHelp"))]
pub struct Options {
    /// Silence all log output.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// Increase log verbosity (-v, -vv, -vvv, etc).
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: usize,
}
