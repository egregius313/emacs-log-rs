use emacs::{FromLisp, IntoLisp};
use log::LevelFilter;

use crate::errors::EmacsLogError;

pub(crate) struct LevelSymbol(LevelFilter);

impl FromLisp<'_> for LevelSymbol {
    fn from_lisp(value: emacs::Value) -> emacs::Result<Self> {
        let env = value.env;
        let symbol_name: String = env.call("symbol-name", (value,))?.into_rust()?;
        let level = match symbol_name.as_str() {
            "off" => LevelFilter::Off,
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => return Err(EmacsLogError::InvalidLogLevelSymbol(symbol_name).into()),
        };
        Ok(LevelSymbol(level))
    }
}

impl<'env> IntoLisp<'env> for LevelSymbol {
    fn into_lisp(self, env: &'env emacs::Env) -> emacs::Result<emacs::Value<'env>> {
        let symbol_name = match self.0 {
            LevelFilter::Off => "off",
            LevelFilter::Error => "error",
            LevelFilter::Warn => "warn",
            LevelFilter::Info => "info",
            LevelFilter::Debug => "debug",
            LevelFilter::Trace => "trace",
        };
        env.intern(symbol_name)
    }
}

impl Into<LevelFilter> for LevelSymbol {
    fn into(self) -> LevelFilter {
        self.0
    }
}
