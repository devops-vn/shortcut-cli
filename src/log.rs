use slog::{Drain, Logger};
use slog_term::{CompactFormat, PlainDecorator, TermDecorator};
use std::sync::Mutex;

use slog_async::Async;
use std::fs::OpenOptions;

use crate::error::EasyErrorHandling;

// Logging to the terminal
pub fn init_term() -> Logger {
    let init_log = Logger::root(
        Mutex::new(CompactFormat::new(TermDecorator::new().build()).build()).fuse(),
        o!(),
    );

    init_log
}

// Logging to a file
pub fn init_file() -> Logger {
    let log_path = "/var/log/shortcut-cli.log";
    let log_file_error = String::from(format!("Cannot create or open log file: {}.", log_path));
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap_or_panic(&log_file_error);

    let init_log = Logger::root(
        Async::new(
            CompactFormat::new(PlainDecorator::new(log_file))
                .build()
                .fuse(),
        )
        .build()
        .fuse(),
        o!(),
    );

    init_log
}
