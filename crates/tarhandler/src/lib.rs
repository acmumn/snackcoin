//! An Iron handler that serves out of a tar file.

extern crate futures;
extern crate iron;
extern crate tar;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read, Result};
use std::path::Path;
use std::sync::Mutex;

use iron::{Handler, IronResult, Request, Response};
use iron::status::NotImplemented;
use tar::Archive;

enum Fs {
    Dir(String, Vec<Fs>),
    File(String, Vec<u8>),
}

/// A handler based on a tar file.
pub struct TarHandler {
    files: Mutex<Vec<Fs>>,
}

impl TarHandler {
    /// Creates a handler from a tar archive.
    fn from_archive<R: Read>(mut archive: Archive<R>) -> Result<TarHandler> {
        let files = archive.entries()?.collect::<Result<Vec<_>>>()?;
        unimplemented!()
    }

    /// Creates a handler from some bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<TarHandler> {
        TarHandler::new(Cursor::new(bytes))
    }

    /// Creates a new TarHandler from a Read, which is a tar file.
    pub fn new<R: Read>(read: R) -> Result<TarHandler> {
        TarHandler::from_archive(Archive::new(read))
    }

    /// Opens a file and creates a TarHandler for it.
    pub fn open_file<P: AsRef<Path>>(path: P) -> Result<TarHandler> {
        File::open(path).and_then(TarHandler::new)
    }
}

impl Handler for TarHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path();
        let body = format!("{:?}", path);
        println!("{}", body);
        Ok(Response::with((NotImplemented, body)))
    }
}
