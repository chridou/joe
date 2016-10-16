#![cfg_attr(feature = "nightly", feature(proc_macro))]

#[cfg(feature = "nightly")]
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate diesel;

#[cfg(feature = "nightly")]
#[macro_use]
extern crate diesel_codegen;

extern crate dotenv;

extern crate uuid;

#[cfg(feature = "nightly")]
include!("lib.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));
