extern crate failure;
extern crate futures;
extern crate iron;
#[macro_use]
extern crate log;
extern crate logger;
extern crate mount;
#[macro_use]
extern crate router;
extern crate stderrlog;
#[macro_use]
extern crate structopt;
extern crate tarhandler;
extern crate tokio_core;

mod options;
mod routes;

use std::process::exit;

use failure::Error;
use iron::prelude::*;
use logger::Logger;
use structopt::StructOpt;

use options::Options;
use routes::make_handler;

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
    let mut chain = Chain::new(make_handler());
    chain.link(Logger::new(None));

    Iron::new(chain).http((options.addr, options.port))?;
    Ok(())
}
