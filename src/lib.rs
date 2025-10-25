//! Provides an implementation of [`log::Log`] that is built on top of Emacs's
//! `message` function.
//! 
//! This allows Rust code running inside Emacs to log messages that will appear
//! in the `*Messages*` buffer, using the standard Rust `log` crate interface.
//!! The log level can be configured from Emacs Lisp using the provided
//! `set-log-level` function.
use emacs::{defun};

mod emacs_logger;
pub use emacs_logger::EmacsLogger;
mod errors;
mod level_symbol;
use level_symbol::LevelSymbol;

/// Set the log level of the [`EmacsLogger`].
/// 
/// This function can be called from Emacs Lisp to set the log level of a crate
/// that uses the `emacs-log-rs` logger.
/// 
/// `logger` must be a userptr to an instance of [`EmacsLogger`].
#[defun(name = "emacs-log-rs/set-log-level")]
fn set_log_level(logger: &mut EmacsLogger, level: LevelSymbol) -> emacs::Result<()> {
    logger.set_max_level(level.into());
    Ok(())
}
