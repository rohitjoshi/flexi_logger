#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::unused_self)]
#![allow(clippy::needless_doctest_main)]

//! A flexible and easy-to-use logger that writes logs to stderr and/or to files
//! or other output streams.
//!
//! To read the log specification from an environment variable and get the log written to `stderr`,
//! start `flexi_logger` e.g. like this:
//! ```rust
//! flexi_logger::Logger::with_env().start().unwrap();
//! ```
//!
//! See
//!
//! * [Logger](crate::Logger) for a full description of all configuration options,
//! * and the [writers](crate::writers) module for the usage of additional log writers,
//! * and [the homepage](https://crates.io/crates/flexi_logger) for how to get started.
//!
//! There are configuration options to e.g.
//!
//! * decide whether you want to write your logs to stderr or to a file,
//! * configure the path and the filenames of the log files,
//! * use file rotation,
//! * specify the line format for the log lines,
//! * define additional log output streams, e.g for alert or security messages,
//! * support changing the log specification while the program is running,
//!
//! `flexi_logger` uses a similar syntax as [`env_logger`](http://crates.io/crates/env_logger/)
//! for specifying which logs should really be written (but is more graceful with the syntax,
//! and can provide error information).
//!
//! By default, i.e. if feature `colors` is not switched off, the log lines that appear on your
//! terminal are coloured. In case the chosen colors don't fit to your terminal's color theme,
//! you can adapt the colors to improve readability.
//! See the documentation of method [style](crate::style)
//! for a description how this can be done.

mod deferred_now;
mod flexi_error;
mod flexi_logger;
mod formats;
mod log_specification;
mod logger;
mod logger_handle;
mod parameters;
mod primary_writer;

pub mod code_examples;
pub mod writers;

/// Re-exports from log crate
pub use log::{Level, LevelFilter, Record};

pub use crate::deferred_now::DeferredNow;
pub use crate::flexi_error::FlexiLoggerError;
pub use crate::formats::*;
pub use crate::log_specification::{LogSpecBuilder, LogSpecification, ModuleFilter};
pub use crate::logger::{Duplicate, LogTarget, Logger};
pub use crate::logger_handle::LoggerHandle;
pub use crate::parameters::{Age, Cleanup, Criterion, Naming};

/// For backwards compatibility.
#[deprecated]
pub type ReconfigurationHandle = LoggerHandle;

pub(crate) const DEFAULT_BUFFER_CAPACITY: usize = 8 * 1024;
pub(crate) const DEFAULT_FLUSH_WAIT_TIME: std::time::Duration = std::time::Duration::from_secs(1);
