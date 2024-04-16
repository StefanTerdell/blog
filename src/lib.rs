pub mod app;
pub mod blog;
pub mod github;
pub mod guestbook;
pub mod home;
#[cfg(feature = "ssr")]
pub mod server;
pub mod utils;
pub mod work;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
