#![crate_type = "lib"]
#![crate_name = "scrap_util"]

#![feature(plugin, collections, convert, slice_patterns)]
#![plugin(maud_macros)]

extern crate maud;
extern crate html5ever;
extern crate html5ever_dom_sink;
extern crate hyper as hyper_crate;
extern crate encoding;
extern crate string_cache;

pub mod dom_util;
pub mod attributes_util;
pub mod decode;

pub use html5ever_dom_sink::rcdom::{RcDom};
pub use html5ever::{parse, one_input};
pub use html5ever_dom_sink::common::*;

pub mod hyper {
    pub use hyper_crate::*;
}
