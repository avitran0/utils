use std::{
    fs::{File, OpenOptions},
    io::{LineWriter, Write as _},
    path::{Path, PathBuf},
    str::FromStr,
};

use parking_lot::Mutex;

pub use log::*;

pub struct LoggerOptions {
    pub level: Level,
    pub file: Option<PathBuf>,
    pub stdout: bool,
    pub module: Option<String>,
    pub debug: bool,
}

impl Default for LoggerOptions {
    fn default() -> Self {
        Self {
            level: Level::Info,
            file: None,
            stdout: true,
            module: None,
            debug: false,
        }
    }
}

impl LoggerOptions {
    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn file(mut self, file: impl AsRef<Path>) -> Self {
        self.file = Some(file.as_ref().to_path_buf());
        self
    }

    pub fn stdout(mut self, stdout: bool) -> Self {
        self.stdout = stdout;
        self
    }

    pub fn module(mut self, module: &str) -> Self {
        self.module = Some(module.to_owned());
        self
    }

    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    fn runtime(self, level: Level) -> RuntimeOptions {
        RuntimeOptions {
            level,
            stdout: self.stdout,
            module: self.module,
            debug: self.debug,
        }
    }
}

struct RuntimeOptions {
    level: Level,
    stdout: bool,
    module: Option<String>,
    debug: bool,
}

pub struct Logger {
    writer: Option<Mutex<LineWriter<File>>>,
    options: RuntimeOptions,
}

impl Logger {
    pub fn install(options: LoggerOptions) {
        Self::new(options).unwrap().init();
    }

    pub fn new(options: LoggerOptions) -> std::io::Result<Self> {
        let writer = if let Some(file) = &options.file {
            let mut path = std::env::current_exe()?;
            path.pop();
            path.push(file);
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)?;
            Some(Mutex::new(LineWriter::new(file)))
        } else {
            None
        };

        let level_env = std::env::var("RUST_LOG").ok();
        let level = if let Some(level_env) = &level_env {
            match Level::from_str(level_env) {
                Ok(level) => level,
                Err(_) => options.level,
            }
        } else {
            options.level
        };

        let options = options.runtime(level);

        Ok(Self { writer, options })
    }

    pub fn init(self) {
        let max_level = self.options.level.to_level_filter();
        log::set_boxed_logger(Box::new(self)).unwrap();
        log::set_max_level(max_level);
    }

    fn write_log(&self, record: &Record) {
        if let Some(module) = &self.options.module
            && let Some(rec_module) = record.module_path()
            && !rec_module.starts_with(module)
        {
            return;
        }

        if self.options.stdout {
            self.write(record, &mut std::io::stdout());
        }
        if let Some(writer) = &self.writer {
            let mut writer = writer.lock();
            self.write(record, writer.get_mut());
        }
    }

    fn write(&self, record: &Record, writer: &mut impl std::io::Write) {
        let _ = if self.options.debug
            && let (Some(file), Some(line)) = (record.file(), record.line())
        {
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
        };
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.options.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            self.write_log(record);
        }
    }

    fn flush(&self) {
        if let Some(writer) = &self.writer {
            let _ = writer.lock().flush();
        }
    }
}
