// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

//! Define parser interface, ReStructuredText.

// use super::paragraph;
// use super::inline;

const HIGHEST_ACCEPTABLE_DOCUMENT_VERSION : i32 = 1;

/// Toplevel parser interface for ReStructuredText.
#[derive(Debug,Clone,Copy)]
pub struct ReStructuredText {
    version: i32,
}

impl ReStructuredText {
    /// Create a new ReStructuredText parser.
    pub fn new() -> ReStructuredText {
        ReStructuredText {
            version: HIGHEST_ACCEPTABLE_DOCUMENT_VERSION
        }
    }

    /// Get version of current parser.
    pub fn version(&self) -> i32 {
        self.version
    }
}
