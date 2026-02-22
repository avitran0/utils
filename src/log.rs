use std::{
    fs::{File, OpenOptions},
    io::{LineWriter, Write as _},
};

use parking_lot::Mutex;

pub use log::*;

pub struct FileLogger {
    writer: Mutex<LineWriter<File>>,
    level: Level,
    debug: bool,
}

impl FileLogger {
    pub fn install(file_name: &str, level: Level, debug: bool) {
        Self::new(file_name, level, debug).unwrap().init();
    }

    pub fn new(file_name: &str, level: Level, debug: bool) -> std::io::Result<Self> {
        let mut path = std::env::current_exe()?;
        path.pop();
        path.push(file_name);
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        let writer = Mutex::new(LineWriter::new(file));

        Ok(Self {
            writer,
            level,
            debug,
        })
    }

    pub fn init(self) {
        let max_level = self.level.to_level_filter();
        log::set_boxed_logger(Box::new(self)).unwrap();
        log::set_max_level(max_level);
    }

    pub fn write_log(&self, record: &log::Record) {
        let mut writer = self.writer.lock();
        if self.debug
            && let Some(file) = record.file()
            && let Some(line) = record.line()
        {
            let _ = writeln!(
                writer,
                "[{}] [{}:{}] {}",
                record.level(),
                file,
                line,
                record.args()
            );
        } else {
            let _ = writeln!(writer, "[{}] {}", record.level(), record.args());
        }
    }
}

impl Log for FileLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            self.write_log(record);
        }
    }

    fn flush(&self) {
        let _ = self.writer.lock().flush();
    }
}
