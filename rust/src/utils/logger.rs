use std::{thread, fmt::Debug, fs::{File, OpenOptions}, path::PathBuf, sync::{Mutex, MutexGuard}, io::Write};
use chrono::{Utc};

pub trait Logger {
    fn log_info (&self, log: impl Debug);
    fn log_warning (&self, log: impl Debug);
    fn log_error (&self, log: impl Debug);

    /// Loggs the info without blocking the current thread
    fn async_log_info<T: 'static + Debug + Send> (&'static self, log: T) where Self: Send + Sync {
        thread::spawn(move || {
            self.log_info(log)
        });
    }

    /// Loggs the warning without blocking the current thread
    fn async_log_warning<T: 'static + Debug + Send> (&'static self, log: T) where Self: Send + Sync {
        thread::spawn(move || {
            self.log_warning(log)
        });
    }

    /// Loggs the error without blocking the current thread
    fn async_log_error<T: 'static + Debug + Send> (&'static self, log: T) where Self: Send + Sync {
        thread::spawn(move || {
            self.log_error(log)
        });
    }
}

// NOLOG
pub struct NoLog;

impl Logger for NoLog {
    fn log_info (&self, _log: impl Debug) {}
    fn log_warning (&self, _log: impl Debug) {}
    fn log_error (&self, _log: impl Debug) {}

    fn async_log_info<T: 'static + Debug + Send> (&'static self, _log: T) where Self: Send + Sync {}
    fn async_log_warning<T: 'static + Debug + Send> (&'static self, _log: T) where Self: Send + Sync {}
    fn async_log_error<T: 'static + Debug + Send> (&'static self, _log: T) where Self: Send + Sync {}
}

// CONSOLE LOG
pub struct ConsoleLog;

impl Logger for ConsoleLog {
    fn log_info (&self, log: impl Debug) {
        let date = Utc::now();
        println!("{date}: {log:?}\n")
    }

    fn log_warning (&self, log: impl Debug) {
        let date = Utc::now();
        eprintln!("{date}: {log:?}\n")
    }

    fn log_error (&self, log: impl Debug) {
        let date = Utc::now();
        eprintln!("{date}: {log:?}\n")
    }
}

// FILE SYSTEM
pub struct FsLog {
    info: Mutex<File>,
    warning: Mutex<File>,
    error: Mutex<File>
}

impl FsLog {    
    pub fn new (dir: PathBuf) -> std::io::Result<Self> {
        let mut info = dir.clone();
        let mut warning = dir.clone();
        let mut error = dir;

        info.push("info.log");
        warning.push("warning.log");
        error.push("error.log");

        let mut opt = OpenOptions::new();
        let opt = opt.write(true);

        Ok(Self {
            info: opt.open(info).map(|x| Mutex::new(x))?,
            warning: opt.open(warning).map(|x| Mutex::new(x))?,
            error: opt.open(error).map(|x| Mutex::new(x))?,
        })
    }
    
    fn log_at (mut file: MutexGuard<File>, str: String) {
        match file.write(str.as_bytes()) {
            Ok(_) => {}
            Err(e) => panic!("{e:?}")
        }
    }
}

impl Logger for FsLog {
    fn log_info (&self, log: impl Debug) {
        let date = Utc::now();
        let file = self.info.lock().unwrap();
        Self::log_at(file, format!("{date}: {log:?}"))
    }

    fn log_warning (&self, log: impl Debug) {
        let date = Utc::now();
        let file = self.warning.lock().unwrap();
        Self::log_at(file, format!("{date}: {log:?}"))
    }

    fn log_error (&self, log: impl Debug) {
        let date = Utc::now();
        let file = self.error.lock().unwrap();
        Self::log_at(file, format!("{date}: {log:?}"))
    }
}