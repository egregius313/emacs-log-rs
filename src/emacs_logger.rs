use std::mem::transmute;
use std::sync::{Arc, Mutex, OnceLock};
use log::{LevelFilter, Log, Metadata, Record, set_max_level};
use emacs::Env;

use crate::errors;

static LOGGER: OnceLock<EmacsLogger> = OnceLock::new();

pub struct EmacsLogger<'env> {
    env: Arc<Mutex<&'env Env>>,
    max_level: LevelFilter,
}

impl<'env> EmacsLogger<'env> {
    pub fn new(env: &'env Env, max_level: LevelFilter) -> Self {
        EmacsLogger {
            env: Arc::new(Mutex::new(env)),
            max_level,
        }
    }

    pub fn init(self) -> Result<(), errors::EmacsLogError> {
        set_max_level(self.max_level);

        // Safety: We are transmuting the lifetime of `self` to `'static` because
        // `OnceLock` requires a `'static` lifetime.
        let static_self: EmacsLogger<'static> = unsafe { transmute(self) };

        LOGGER.set(static_self).map_err(|_| errors::EmacsLogError::LoggerAlreadyInitialized)?;

        log::set_logger(LOGGER.wait()).unwrap();
        Ok(())
    }

    pub fn max_level(&self) -> LevelFilter {
        self.max_level
    }

    pub fn set_max_level(&mut self, level: LevelFilter) {
        self.max_level = level;
        set_max_level(level);
    }
}

impl Log for EmacsLogger<'_> {
    fn enabled(&self, meta: &Metadata<'_>) -> bool {
        meta.level() <= self.max_level
    }

    fn log(&self, record: &Record) {
        let env = self.env.lock().unwrap();

        if !self.enabled(record.metadata()) {
            return;
        }

        let _ = env.message(
            &format!("[{}] - {}", record.level(), record.args())
        );
    }

    fn flush(&self) {}
}

unsafe impl Send for EmacsLogger<'_> {}
unsafe impl Sync for EmacsLogger<'_> {}