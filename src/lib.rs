// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

#![deny(missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]

//! Birlstone: RuStructuredText processing library
//! Convert ReStructuredText documentation to HTML files.

mod rst;

pub use rst::parser::ReStructuredText;

#[cfg(test)]
mod tests {
    use super::ReStructuredText;

    #[test]
    fn create_rst_parser() {
        let rst = ReStructuredText::new();
        assert!(rst.version() == 1);
    }
}
