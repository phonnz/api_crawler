#![feature(decl_macro)]

mod handlers;
mod server;

fn main() {
    crate::server::server().launch();
}
