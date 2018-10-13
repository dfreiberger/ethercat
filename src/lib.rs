extern crate libc;
extern crate memmap;
extern crate ethercat_sys as ec;

pub mod master;
pub mod types;

pub use self::types::Result;
pub use self::master::{Master, Domain, SlaveConfig};