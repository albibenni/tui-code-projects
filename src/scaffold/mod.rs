use std::sync::atomic::AtomicBool;

pub static INTERRUPTED: AtomicBool = AtomicBool::new(false);

mod command;
mod flutter;
mod go;
mod go_desktop;
mod java;
mod kotlin_mobile;
mod params;
mod php;
mod python;
mod python_desktop;
mod run;
mod rust;
mod rust_desktop;
mod swift_desktop;
mod swift_mobile;
mod typescript_backend;
mod typescript_desktop;
mod typescript_frontend;
pub mod writer;
mod writer_constants;

#[cfg(test)]
pub(crate) use flutter::{launch_json_for, platforms_for};
pub use params::ScaffoldParams;
pub use run::{cleanup, run_threaded};
