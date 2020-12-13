#![feature(decl_macro)]

mod handlers;
mod server;
mod types;

fn main() {
    crate::server::server().launch();
}
