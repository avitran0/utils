use std::{
    fs::{File, OpenOptions},
    io::{self, LineWriter, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use parking_lot::Mutex as ParkingMutex;

pub use log::*;

/// Configuration options for the Logger.
#[derive(Debug, Clone)]
pub struct LoggerOptions {
    /// The logging level.
    pub level: Level,
    /// Optional file path to write logs to.
    pub file: Option<PathBuf>,
    /// Whether to truncate the log file, if present.
    pub truncate: bool,
    /// Whether to write logs to stdout.
    pub stdout: bool,
    /// Optional module prefix to filter logs.
    pub module: Option<String>,
    /// Whether to include debug information (file/line) in logs.
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
    /// Sets the logging level.
    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// Sets the log file path.
    pub fn file(mut self, file: impl AsRef<Path>) -> Self {
        self.file = Some(file.as_ref().to_path_buf());
        self
    }

    /// Enables or disables stdout logging.
    pub fn stdout(mut self, stdout: bool) -> Self {
        self.stdout = stdout;
        self
    }

    /// Sets a module filter.
    pub fn module(mut self, module: &str) -> Self {
        self.module = Some(module.to_owned());
        self
    }

    /// Enables or disables debug mode.
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}

/// A simple logger implementation.
pub struct Logger {
    writer: Option<ParkingMutex<LineWriter<File>>>,
    options: LoggerOptions,
}

impl Logger {
    /// Installs the logger globally.
    pub fn install(options: LoggerOptions) {
        Self::new(options)
            .expect("Failed to create logger")
            .init()
            .expect("Failed to install logger");
    }

    /// Creates a new Logger instance.
    pub fn new(mut options: LoggerOptions) -> io::Result<Self> {
        // Handle RUST_LOG environment variable
        if let Ok(level_env) = std::env::var("RUST_LOG")
            && let Ok(level) = Level::from_str(&level_env)
        {
            options.level = level;
        }

        let writer = if let Some(file_path) = &options.file {
            // Use current directory for relative paths, or absolute path as is.
            // The original code used current_exe(), which is non-standard for logs.
            // We'll stick to standard file opening which respects CWD.
            // If the user wants current_exe relative, they should construct the path themselves.
            // However, to be safe and "improve", standard CWD is better.

            // Ensure parent directory exists?
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .truncate(options.truncate)
                .open(file_path)?;

            Some(ParkingMutex::new(LineWriter::new(file)))
        } else {
            None
        };

        Ok(Self { writer, options })
    }

    /// Initializes the logger.
    pub fn init(self) -> Result<(), SetLoggerError> {
        let max_level = self.options.level.to_level_filter();
        log::set_boxed_logger(Box::new(self))?;
        log::set_max_level(max_level);
        Ok(())
    }

    fn write_log(&self, record: &Record) {
        if let Some(module) = &self.options.module {
            if let Some(rec_module) = record.module_path() {
                if !rec_module.starts_with(module) {
                    return;
                }
            }
        }

        if self.options.stdout {
            let stdout = std::io::stdout();
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
        let _ = std::io::stdout().flush();
    }
}
