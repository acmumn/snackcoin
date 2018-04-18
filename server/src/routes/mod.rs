mod api;
mod statics;

use mount::Mount;

pub fn make_handler() -> Mount {
    let mut mount = Mount::new();
    mount
        .mount("/api/", api::make_handler())
        .mount("/", statics::make_handler());
    mount
}
