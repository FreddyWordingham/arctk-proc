//! Support library of procedural macros.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::module_name_repetitions,
    clippy::panic,
    clippy::expect_used,
    clippy::wildcard_enum_match_arm
)]

extern crate proc_macro;
extern crate proc_macro2;

mod file;
mod input;
mod output;
mod save;

use crate::proc_macro::TokenStream;

/// Create the procedural macro Input.
#[proc_macro_derive(Input)]
#[inline]
#[must_use]
pub fn input_derive(input: TokenStream) -> TokenStream {
    input::input_derive_impl(input)
}

/// Create the procedural macro File.
#[proc_macro_derive(File)]
#[inline]
#[must_use]
pub fn file_derive(input: TokenStream) -> TokenStream {
    file::file_derive_impl(input)
}

/// Create the procedural macro Output.
#[proc_macro_derive(Output)]
#[inline]
#[must_use]
pub fn output_derive(input: TokenStream) -> TokenStream {
    output::output_derive_impl(input)
}

/// Create the procedural macro Save.
#[proc_macro_derive(Save)]
#[inline]
#[must_use]
pub fn save_derive(input: TokenStream) -> TokenStream {
    save::save_derive_impl(input)
}
