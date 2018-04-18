//! An Iron handler that serves out of a tar file.

extern crate futures;
extern crate iron;
extern crate tar;

use std::fs::File;
use std::io::{Cursor, Read, Result as IoResult};
use std::path::Path;
use std::sync::Mutex;

use iron::{Handler, IronResult, Request, Response};
use iron::status::NotImplemented;
use tar::Archive;

/// A handler based on a tar file.
pub struct TarHandler<R: 'static + Read + Send = File> {
    archive: Mutex<Archive<R>>,
}

impl TarHandler<File> {
    /// Opens a file and creates a TarHandler for it.
    pub fn open_file<P: AsRef<Path>>(path: P) -> IoResult<TarHandler<File>> {
        File::open(path).map(TarHandler::new)
    }
}

impl TarHandler<Cursor<Vec<u8>>> {
    /// Creates a handler from some bytes.
    pub fn from_bytes(bytes: Vec<u8>) -> TarHandler<Cursor<Vec<u8>>> {
        TarHandler::new(Cursor::new(bytes))
    }
}

impl<R: 'static + Read + Send> TarHandler<R> {
    /// Creates a new TarHandler from a Read, which is a tar file.
    pub fn new(read: R) -> TarHandler<R> {
        TarHandler::from(Archive::new(read))
    }
}

impl<R: 'static + Read + Send> From<Archive<R>> for TarHandler<R> {
    fn from(archive: Archive<R>) -> TarHandler<R> {
        let archive = Mutex::new(archive);
        TarHandler { archive }
    }
}

impl<R: 'static + Read + Send> Handler for TarHandler<R> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        println!("{:?}", req);
        Ok(Response::with((NotImplemented, "TODO")))
    }
}
