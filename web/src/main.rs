mod app;
mod ast_layout;
mod ast_trace;
mod ast_tutor;
mod engine;
mod examples;
mod highlight;
mod trace;
mod tutor;
mod wyrm_art;

use app::App;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
