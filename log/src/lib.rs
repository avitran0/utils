//! a lightweight logger with stdout and file output options.

use std::{
    fmt::Arguments,
    fs::{File, OpenOptions},
    io::{LineWriter, Write},
    path::{Path, PathBuf},
    sync::OnceLock,
};

use parking_lot::Mutex;

/// configuration options for the logger.
#[derive(Debug, Clone)]
pub struct LoggerOptions {
    /// the logging level.
    pub level: Level,
    /// optional file path to write logs to.
    pub file: Option<PathBuf>,
    /// whether to truncate the log file, if present.
    pub truncate: bool,
    /// whether to write logs to stdout.
    pub stdout: bool,
    /// optional module prefix to filter logs.
    pub module: Option<String>,
    /// whether to include debug information (file/line) in logs.
    pub debug: bool,
}

impl Default for LoggerOptions {
    fn default() -> Self {
        Self {
            level: Level::Info,
            file: None,
            truncate: false,
            stdout: true,
            module: None,
            debug: false,
        }
    }
}

impl LoggerOptions {
    /// sets the logging level.
    #[must_use]
    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// sets the log file path.
    #[must_use]
    pub fn file(mut self, file: impl AsRef<Path>) -> Self {
        self.file = Some(file.as_ref().to_path_buf());
        self
    }

    /// enables or disables log file truncation.
    #[must_use]
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    /// enables or disables stdout logging.
    #[must_use]
    pub fn stdout(mut self, stdout: bool) -> Self {
        self.stdout = stdout;
        self
    }

    /// sets a module filter.
    #[must_use]
    pub fn module(mut self, module: &str) -> Self {
        self.module = Some(module.to_owned());
        self
    }

    /// enables or disables debug mode.
    #[must_use]
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}

/// initializes the logger.
/// should only be called once.
pub fn init(options: LoggerOptions) -> std::io::Result<()> {
    let file = match &options.file {
        Some(path) => Some(LineWriter::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(options.truncate)
                .open(path)?,
        )),
        None => None,
    };

    LOGGER.get_or_init(|| Mutex::new(Logger { file, options }));

    Ok(())
}

#[derive(Debug, Clone)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Debug => "DEBUG",
                Self::Info => "INFO",
                Self::Warn => "WARN",
                Self::Error => "ERROR",
            }
        )
    }
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($args:tt)+) => {
        $crate::log(
            $level,
            $crate::Location {
                module: module_path!(),
                file: file!(),
                line: line!(),
            },
            format_args!($($args)+),
        )
    };
}

#[macro_export]
macro_rules! debug {
    ($($args:tt)+) => {
        $crate::log!($crate::Level::Debug, $($args)+)
    };
}

#[macro_export]
macro_rules! info {
    ($($args:tt)+) => {
        $crate::log!($crate::Level::Info, $($args)+)
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)+) => {
        $crate::log!($crate::Level::Warn, $($args)+)
    };
}

#[macro_export]
macro_rules! error {
    ($($args:tt)+) => {
        $crate::log!($crate::Level::Error, $($args)+)
    };
}

pub struct Location {
    pub module: &'static str,
    pub file: &'static str,
    pub line: u32,
}

struct Record<'a> {
    level: Level,
    location: Location,
    args: Arguments<'a>,
}

pub fn log(level: Level, location: Location, args: Arguments) {
    if let Some(logger) = LOGGER.get() {
        logger.lock().log(Record {
            level,
            location,
            args,
        });
    }
}

static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

struct Logger {
    file: Option<LineWriter<File>>,
    options: LoggerOptions,
}

impl Logger {
    fn log(&mut self, record: Record) {
        if let Some(file) = &mut self.file {
            Self::log_dispatch(file, &record, self.options.debug);
        }

        if self.options.stdout {
            Self::log_dispatch(&mut std::io::stdout().lock(), &record, self.options.debug);
        }
    }

    fn log_dispatch(writer: &mut impl Write, record: &Record, debug: bool) {
        match debug {
            true => Self::log_debug(writer, record),
            false => Self::log_no_debug(writer, record),
        }
    }

    fn log_debug(writer: &mut impl Write, record: &Record) {
        let _ = writeln!(
            writer,
            "[{}] [{}:{}] {}",
            record.level, record.location.file, record.location.line, record.args
        );
    }

    fn log_no_debug(writer: &mut impl Write, record: &Record) {
        let _ = writeln!(writer, "[{}] {}", record.level, record.args);
    }
}
