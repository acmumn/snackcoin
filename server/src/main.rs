extern crate failure;
extern crate futures;
extern crate iron;
#[macro_use]
extern crate log;
extern crate logger;
extern crate mount;
extern crate router;
extern crate stderrlog;
#[macro_use]
extern crate structopt;
extern crate tokio_core;

mod options;

use std::process::exit;

use failure::Error;
use structopt::StructOpt;

use options::Options;

fn main() {
    let options = Options::from_args();
    stderrlog::new()
        .quiet(options.quiet)
        .verbosity(options.verbose + 2)
        .init()
        .expect("Failed to start logger");

    if let Err(err) = run(options) {
        error!("{}", err);
        exit(1);
    }
}

fn run(options: Options) -> Result<(), Error> {
    unimplemented!()
}
