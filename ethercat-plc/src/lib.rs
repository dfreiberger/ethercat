#[macro_use]
extern crate log;
extern crate libc;
extern crate time;
extern crate mlzlog;
extern crate byteorder;
extern crate crossbeam_channel;
extern crate ethercat;

mod image;
mod plc;
mod server;

pub mod beckhoff;

pub use self::image::{ExternImage, ProcessImage};
pub use self::plc::{Plc, PlcBuilder};