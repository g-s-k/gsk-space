#[macro_use]
extern crate yew;

use yew::prelude::*;

mod app;
mod header;
mod footer;

fn main() {
    yew::initialize();
    App::<app::App>::new().mount_to_body();
    yew::run_loop();
}
