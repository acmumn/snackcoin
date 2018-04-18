use std::io::Cursor;

use tarhandler::TarHandler;

const ASSETS: &[u8] = include_bytes!("../../../dist/site.tar");

pub fn make_handler() -> TarHandler<Cursor<Vec<u8>>> {
    TarHandler::from_bytes(ASSETS.to_owned())
}
