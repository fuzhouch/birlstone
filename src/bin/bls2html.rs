// Copyright (c) 2016 Fuzhou Chen
// Licensed under MIT 2.0 license. See LICENSE file for details.

//! A minimal executable file to demonstrate how to use birlstone APIs.
//! Note that the tool must be compiled explicitly with command:
//!
//!     cargo build --features "cmdline"
//!
//! The executable is not built by default to avoid unncessary
//! dependencies when the package is consumed as a library by other
//! packages.

#[cfg(feature = "cmdline")]
extern crate birlstone;
#[cfg(feature = "cmdline")]
extern crate log;
#[cfg(feature = "cmdline")]
extern crate log4rs;

#[cfg(feature = "cmdline")]
#[allow(unused_variables)]
fn main() {
    use birlstone::ReStructuredText;
    use log::LogLevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::{Appender, Config, Logger, Root};

    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("birlstone", LogLevelFilter::Info))
        .build(Root::builder()
            .appender("stdout")
            .build(LogLevelFilter::Info))
        .unwrap();
    let handle = log4rs::init_config(config).unwrap();

    let rst = ReStructuredText::new();
    println!("{}", rst.version());
}

#[cfg(not(feature = "cmdline"))]
fn main() {
    print!("{}\n", "ERROR:");
    print!("{}\n", "    Bls2html is disable. Enable it by command: ");
    print!("{}\n", "        cargo build --features = \"cmdline\"");
}
