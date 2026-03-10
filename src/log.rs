//! a lightweight logger with stdout and file output options.

use std::{
    fs::{File, OpenOptions},
    io::{self, LineWriter, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use parking_lot::Mutex;

pub use log::*;

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

/// a simple logger implementation.
pub struct Logger {
    writer: Option<Mutex<LineWriter<File>>>,
    options: LoggerOptions,
}

impl Logger {
    /// installs the logger globally.
    pub fn install(options: LoggerOptions) {
        Self::new(options)
            .expect("Failed to create logger")
            .init()
            .expect("Failed to install logger");
    }

    /// creates a new logger instance.
    pub fn new(mut options: LoggerOptions) -> io::Result<Self> {
        if let Ok(level_env) = std::env::var("RUST_LOG")
            && let Ok(level) = Level::from_str(&level_env)
        {
            options.level = level;
        }

        let writer = if let Some(file_path) = &options.file {
            let log_path = if file_path.is_absolute() {
                file_path
            } else {
                let exe_path = std::env::current_exe()?;
                let exe_parent = exe_path
                    .parent()
                    .ok_or_else(|| io::Error::other("Executable path has no parent directory"))?;

                let file_name = file_path
                    .file_name()
                    .ok_or_else(|| io::Error::other("Invalid file path"))?;

                &exe_parent.join(file_name)
            };

            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(options.truncate)
                .append(!options.truncate)
                .open(log_path)?;

            Some(Mutex::new(LineWriter::new(file)))
        } else {
            None
        };

        Ok(Self { writer, options })
    }

    /// initializes the logger.
    pub fn init(self) -> Result<(), SetLoggerError> {
        let max_level = self.options.level.to_level_filter();
        set_boxed_logger(Box::new(self))?;
        set_max_level(max_level);
        Ok(())
    }

    fn write_log(&self, record: &Record) {
        if let Some(module) = &self.options.module
            && let Some(rec_module) = record.module_path()
            && !rec_module.starts_with(module)
        {
            return;
        }

        if self.options.stdout {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            self.write(record, &mut handle);
        }

        if let Some(writer) = &self.writer {
            let mut writer = writer.lock();
            self.write(record, &mut *writer);
        }
    }

    fn write(&self, record: &Record, writer: &mut impl Write) {
        let _ = if self.options.debug {
            if let (Some(file), Some(line)) = (record.file(), record.line()) {
                writeln!(
                    writer,
                    "[{}] [{}:{}] {}",
                    record.level(),
                    file,
                    line,
                    record.args()
                )
            } else {
                writeln!(writer, "[{}] {}", record.level(), record.args())
            }
        } else {
            writeln!(writer, "[{}] {}", record.level(), record.args())
        };
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.options.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.write_log(record);
        }
    }

    fn flush(&self) {
        if let Some(writer) = &self.writer {
            let _ = writer.lock().flush();
        }
        let _ = io::stdout().flush();
    }
}
