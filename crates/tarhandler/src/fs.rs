use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

#[derive(Debug)]
enum Dirent {
    Dir(HashMap<String, Dirent>),
    File(Vec<u8>),
}

impl Dirent {
    fn as_dir(&self) -> Option<&HashMap<String, Dirent>> {
        match *self {
            Dirent::Dir(ref entries) => Some(entries),
            _ => None,
        }
    }

    fn as_file(&self) -> Option<&[u8]> {
        match *self {
            Dirent::File(ref data) => Some(data),
            _ => None,
        }
    }
}

/// A filesystem.
#[derive(Debug)]
pub struct Fs {
    entries: HashMap<String, Dirent>,
}

impl Fs {
    /// Creates a new, empty filesystem.
    pub fn new() -> Fs {
        Fs {
            entries: HashMap::new(),
        }
    }

    /// Adds a file to the filesystem, creating any required parent
    /// directories. If a file or directory already exists, returns an error.
    pub fn add_file(
        &mut self,
        mut path: Vec<String>,
        contents: Vec<u8>,
    ) -> Result<()> {
        let path_str = path.join("/");
        let base = match path.pop() {
            Some(x) => x,
            None => {
                return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    format!("Cannot create a file at path `/'"),
                ))
            }
        };

        let dir = self.mkdirp(path)?;
        if dir.contains_key(&base) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("File already exists at path `/{}'", path_str),
            ));
        }
        dir.insert(base, Dirent::File(contents));
        Ok(())
    }

    /// Gets a file from the filesystem, if it exists.
    pub fn get_file<'a>(&'a self, path: &[&str]) -> Option<&'a [u8]> {
        fn helper<'a>(dirent: &'a Dirent, path: &[&str]) -> Option<&'a [u8]> {
            println!("{:?}", path);
            if let Some((hd, tl)) = path.split_first() {
                dirent
                    .as_dir()
                    .and_then(|entries| entries.get(*hd))
                    .and_then(|file| helper(file, tl))
            } else {
                match dirent {
                    &Dirent::File(ref data) => Some(data),
                    &Dirent::Dir(ref entries) => entries
                        .get("index.html")
                        .and_then(|dirent| dirent.as_file()),
                }
            }
        }

        // TODO: This is suboptimal, from a readability perspective. Basically,
        // our helper only works with dirents, but we have a directory table
        // directly in the Fs.
        if let Some((hd, tl)) = path.split_first() {
            self.entries.get(*hd).and_then(|file| helper(file, tl))
        } else {
            static PATH: &[&str] = &["index.html"];
            self.get_file(PATH)
        }
    }

    /// Recursively creates directories. If any of the directories exist, they
    /// are ignored. If a file exists somewhere in the path, returns an error.
    fn mkdirp<'a>(
        &'a mut self,
        mut path: Vec<String>,
    ) -> Result<&'a mut HashMap<String, Dirent>> {
        fn helper<'a>(
            top_dir: &'a mut HashMap<String, Dirent>,
            mut path: Vec<String>,
        ) -> Result<&'a mut HashMap<String, Dirent>> {
            if let Some(hd) = path.pop() {
                let dirent = top_dir
                    .entry(hd)
                    .or_insert_with(|| Dirent::Dir(HashMap::new()));
                match dirent {
                    &mut Dirent::Dir(ref mut subdir) => helper(subdir, path),
                    _ => {
                        let path = path.join("/");
                        return Err(Error::new(
                            ErrorKind::AlreadyExists,
                            format!("File already exists at path `/{}'", path),
                        ));
                    }
                }
            } else {
                Ok(top_dir)
            }
        }

        path.reverse();
        helper(&mut self.entries, path)
    }
}
