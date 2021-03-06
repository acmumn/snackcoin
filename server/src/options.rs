use std::net::IpAddr;

#[derive(Debug, StructOpt)]
#[structopt(raw(global_setting = "::structopt::clap::AppSettings::ColoredHelp"))]
pub struct Options {
    /// The address to serve on.
    #[structopt(short = "h", long = "host", default_value = "0.0.0.0")]
    pub addr: IpAddr,

    /// The port to serve on.
    #[structopt(short = "p", long = "port", default_value = "8080")]
    pub port: u16,

    /// Silence all log output.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// Increase log verbosity (-v, -vv, -vvv, etc).
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: usize,
}
