use std::{fmt::{Display}, path::PathBuf};
use async_trait::async_trait;
use chrono::{Utc};
use futures::future::{try_join3, join};
use tokio::{sync::{Mutex, MutexGuard}, fs::{File, OpenOptions}, io::AsyncWriteExt};

#[async_trait]
pub trait Logger {
    async fn log_info<D: Display + Send> (&self, log: D);
    async fn log_warning<D: Display + Send> (&self, log: D);
    async fn log_error<D: Display + Send> (&self, log: D);
}

// NOLOG
pub struct NoLog;

#[async_trait]
impl Logger for NoLog {
    async fn log_info<D: Display + Send> (&self, _log: D) {}
    async fn log_warning<D: Display + Send> (&self, _log: D) {}
    async fn log_error<D: Display + Send> (&self, _log: D) {}
}

// CONSOLE LOG
pub struct ConsoleLog;

#[async_trait]
impl Logger for ConsoleLog {
    async fn log_info<D: Display + Send> (&self, log: D) {
        let date = Utc::now();
        println!("{date}: {log}\n")
    }

    async fn log_warning<D: Display + Send> (&self, log: D) {
        let date = Utc::now();
        eprintln!("{date}: {log}\n")
    }

    async fn log_error<D: Display + Send> (&self, log: D) {
        let date = Utc::now();
        eprintln!("{date}: {log}\n")
    }
}

// FILE SYSTEM
pub struct FsLog {
    info: Mutex<File>,
    warning: Mutex<File>,
    error: Mutex<File>
}

impl FsLog {    
    pub async fn new (dir: impl Into<PathBuf>) -> std::io::Result<Self> {
        let dir = dir.into();
        let mut info = dir.clone();
        let mut warning = dir.clone();
        let mut error = dir;

        info.push("info.log");
        warning.push("warning.log");
        error.push("error.log");

        let mut opt = OpenOptions::new();
        let opt = opt.write(true);

        let (info, warning, error) = try_join3(
            opt.open(info), 
            opt.open(warning),
            opt.open(error)
        ).await?;

        Ok(Self {
            info: Mutex::new(info),
            warning: Mutex::new(warning),
            error: Mutex::new(error),
        })
    }
    
    async fn log_at<'a> (mut file: MutexGuard<'a, File>, str: String) {
        match file.write(str.as_bytes()).await {
            Ok(_) => {}
            Err(e) => eprintln!("{e:?}")
        }
    }
}

#[async_trait]
impl Logger for FsLog {
    async fn log_info<D: Display + Send> (&self, log: D) {
        let file = self.info.lock().await;
        let date = Utc::now();
        Self::log_at(file, format!("{date}: {log}")).await
    }

    async fn log_warning<D: Display + Send> (&self, log: D) {
        let file = self.warning.lock().await;
        let date = Utc::now();
        Self::log_at(file, format!("{date}: {log}")).await
    }

    async fn log_error<D: Display + Send> (&self, log: D) {
        let file = self.error.lock().await;
        let date = Utc::now();
        Self::log_at(file, format!("{date}: {log}")).await
    }
}

#[async_trait]
impl<A, B> Logger for (A, B) where A: Send + Sync + Logger, B: Send + Sync + Logger {
    async fn log_info<D: Display + Send> (&self, log: D) {
        join(self.0.log_info(format!("{log}")), self.1.log_info(log)).await;
    }

    async fn log_warning<D: Display + Send> (&self, log: D) {
        join(self.0.log_warning(format!("{log}")), self.1.log_warning(log)).await;
    }

    async fn log_error<D: Display + Send> (&self, log: D) {
        join(self.0.log_error(format!("{log}")), self.1.log_error(log)).await;
    }
}