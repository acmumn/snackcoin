//! An Iron handler that serves out of a tar file.

extern crate futures;
extern crate iron;
extern crate mime_guess;
extern crate tar;

mod fs;

use std::fs::File;
use std::io::{Cursor, Error, ErrorKind, Read, Result};
use std::path::{Component, Path};
use std::sync::RwLock;

use iron::{status, Handler, IronResult, Request, Response};
use mime_guess::get_mime_type;
use tar::{Archive, EntryType};

use fs::Fs;

/// A handler based on a tar file.
pub struct TarHandler {
    root: RwLock<Fs>,
}

impl TarHandler {
    /// Creates a handler from a tar archive.
    fn from_archive<R: Read>(mut archive: Archive<R>) -> Result<TarHandler> {
        let mut root = Fs::new();
        for entry in archive.entries()? {
            let mut entry = entry?;
            if entry.header().entry_type() != EntryType::Regular {
                continue;
            }

            let path = {
                let path = entry.header().path()?;
                path.components()
                    .filter_map(|c| {
                        match c {
                            Component::Normal(c) => match c.to_str() {
                                Some(c) => Some(Ok(c.to_string())),
                                None => Some(Err(Error::new(
                                    ErrorKind::Other,
                                    format!(
                                        "Component {:?} of path {} is not valid Unicode",
                                        c,
                                        path.display()),
                                ))),
                            },
                            Component::CurDir => None,
                            _ => Some(Err(Error::new(
                                ErrorKind::Other,
                                format!(
                                    "Invalid component {:?} of path {}",
                                    c,
                                    path.display()),
                                ))),
                        }
                    })
                    .collect::<Result<Vec<_>>>()?
            };

            let mut body = vec![];
            entry.read_to_end(&mut body)?;

            root.add_file(path, body)?;
        }

        Ok(TarHandler {
            root: RwLock::new(root),
        })
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
        let root = self.root.read().unwrap();
        let path = req.url.path();
        if let Some(file) = root.get_file(&path) {
            let mime = path.last()
                .map(|f| -> &Path { f.as_ref() })
                .and_then(|p| p.extension())
                .and_then(|s| s.to_str())
                .map(get_mime_type)
                .unwrap_or_else(|| "text/html".parse().unwrap());
            Ok(Response::with((status::Ok, mime, file)))
        } else {
            Ok(Response::with(status::NotFound))
        }
    }
}
