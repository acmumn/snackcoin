use tarhandler::TarHandler;

const ASSETS: &[u8] = include_bytes!("../../../dist/site.tar");

pub fn make_handler() -> TarHandler {
    TarHandler::from_bytes(ASSETS).expect("Invalid ASSETS")
}
