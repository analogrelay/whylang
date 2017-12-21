#![feature(inclusive_range_syntax, inclusive_range, range_contains, fn_traits)]

extern crate lazy_init;

#[macro_use]
mod macros;

mod utils;

pub mod parser;
pub mod text;
pub mod tokenizer;
