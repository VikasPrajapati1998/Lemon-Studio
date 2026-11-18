// Copyright (C) 2026 ItsAlphaNeon
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of Lemon Studio: https://github.com/ItsAlphaNeon/Lemon-Studio

//! The application log: one file per run, kept by date in the user folder, and one per open project, kept by date in the
//! project's own folder.
//!
//! ```text
//! <user folder>/logs/YYYY-MM-DD/YYYY-MM-DD_HH-MM-SS.log      the application: every line of the run
//! <project folder>/logs/YYYY-MM-DD/YYYY-MM-DD_HH-MM-SS.log   the project: every line from when it was opened or saved
//! ```
//!
//! `init` starts the run's file (the folders are made as needed, and a file already there is never touched: the run
//! takes the next free name). The `log_debug!`, `log_info!`, `log_warn!` and `log_error!` macros write a line with the time,
//! the level, the thread and the message, to the file and to stderr, so the terminal shows what it always did. Panics are
//! added to it by `panic_log`. `OPENSTUDIO_LOG_DIR` chooses another folder in place of `<user folder>/logs`, and
//! `OPENSTUDIO_LOG=debug|info|warn|error` the least important level that is written (info by default).
//!
//! The project's file is started when the project gets a place on disk (opened, or saved for the first time) and ends
//! when another project replaces it (`follow_project`, which the app calls on every tick); a project that was never saved
//! has none. `OPENSTUDIO_LOG_DIR` moves the application's folder only.
//!
//! Before `init` (and in tests) lines go to stderr only. A file that cannot be made or written is reported once on
//! stderr and the run goes on without it.

use std::fmt::Arguments;
use std::fs::{File, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};

use chrono::{DateTime, Local};

/// How important a line is, least first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    fn label(self) -> &'static str {
        match self {
            Level::Debug => "DEBUG",
            Level::Info => "INFO ",
            Level::Warn => "WARN ",
            Level::Error => "ERROR",
        }
    }

    /// `OPENSTUDIO_LOG`'s value: a level's name in any case.
    fn parse(text: &str) -> Option<Level> {
        match text.trim().to_ascii_lowercase().as_str() {
            "debug" => Some(Level::Debug),
            "info" => Some(Level::Info),
            "warn" | "warning" => Some(Level::Warn),
            "error" => Some(Level::Error),
            _ => None,
        }
    }
}

/// The run's file, once `init` has made it.
static FILE: OnceLock<Mutex<File>> = OnceLock::new();
/// The least important level written (a `Level` as a number).
static MIN: AtomicU8 = AtomicU8::new(Level::Info as u8);
/// A failed write has been reported on stderr.
static WRITE_FAILED: AtomicBool = AtomicBool::new(false);

/// The open project's log: the project file it belongs to, and its file (`None` when it could not be made).
struct ProjectLog {
    project: PathBuf,
    file: Option<File>,
}

static PROJECT: Mutex<Option<ProjectLog>> = Mutex::new(None);

/// The folder for a day's logs: `root/YYYY-MM-DD`.
fn day_dir(root: &Path, now: DateTime<Local>) -> PathBuf {
    root.join(now.format("%Y-%m-%d").to_string())
}

/// A new file in `root`'s folder for the day of `now`, named `YYYY-MM-DD_HH-MM-SS.log`. A file already there (a second
/// run in the same second) is left alone and the next name is taken: `..._1.log`, `..._2.log`.
fn create_in(root: &Path, now: DateTime<Local>) -> std::io::Result<(File, PathBuf)> {
    let dir = day_dir(root, now);
    std::fs::create_dir_all(&dir)?;
    let stem = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    for n in 0u32.. {
        let name = if n == 0 { format!("{stem}.log") } else { format!("{stem}_{n}.log") };
        let path = dir.join(name);
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((file, path)),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
    unreachable!("the loop only ends by returning")
}

/// Where the logs are kept: `OPENSTUDIO_LOG_DIR`, else `logs` in the user folder.
fn root() -> Option<PathBuf> {
    match std::env::var_os("OPENSTUDIO_LOG_DIR").map(PathBuf::from).filter(|p| p.is_absolute()) {
        Some(dir) => Some(dir),
        None => os_paths::user_file("logs"),
    }
}

/// One line: the time to the millisecond, the level, the thread and the message.
fn format_line(now: DateTime<Local>, level: Level, thread: &str, message: &str) -> String {
    format!("{} {} [{thread}] {message}\n", now.format("%Y-%m-%d %H:%M:%S%.3f"), level.label())
}

/// Starts this run's log file and writes its first lines. Call it once, early in `main`; a second call does nothing.
pub fn init() {
    if let Some(level) = std::env::var("OPENSTUDIO_LOG").ok().as_deref().and_then(Level::parse) {
        MIN.store(level as u8, Ordering::Relaxed);
    }
    if FILE.get().is_some() {
        return;
    }
    let Some(root) = root() else {
        eprintln!("Lemon Studio: no folder for the log files here; logging to the terminal only");
        return;
    };
    match create_in(&root, Local::now()) {
        Ok((file, path)) => {
            if FILE.set(Mutex::new(file)).is_ok() {
                write(Level::Info, format_args!("Lemon Studio {} started; log file {}", crate::version::text(), path.display()));
            }
        }
        Err(e) => eprintln!("Lemon Studio: could not make a log file in {}: {e}; logging to the terminal only", root.display()),
    }
}

/// Writes one line at `level`, to the run's file (once `init` has made it) and to stderr. The macros call it.
pub fn write(level: Level, message: Arguments) {
    if (level as u8) < MIN.load(Ordering::Relaxed) {
        return;
    }
    let thread = std::thread::current();
    let line = format_line(Local::now(), level, thread.name().unwrap_or("unnamed"), &message.to_string());
    eprint!("{line}");
    if let Some(file) = FILE.get() {
        // A thread that panicked while writing leaves the lock poisoned; the file itself is still good to write to.
        let mut file = file.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        report_failure(file.write_all(line.as_bytes()));
    }
    let mut project = PROJECT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(file) = project.as_mut().and_then(|p| p.file.as_mut()) {
        report_failure(file.write_all(line.as_bytes()));
    }
}

/// A failed write is reported on stderr once; the run goes on.
fn report_failure(result: std::io::Result<()>) {
    if let Err(e) = result {
        if !WRITE_FAILED.swap(true, Ordering::Relaxed) {
            eprintln!("Lemon Studio: a log file could not be written: {e}");
        }
    }
}

/// The project's folder for its logs: `logs` beside the project file.
fn project_root(project: &Path) -> Option<PathBuf> {
    project.parent().filter(|d| !d.as_os_str().is_empty()).map(|d| d.join("logs"))
}

/// Makes the log follow the open project: `project` is its file (`None`: a project not saved yet). When it is not the
/// one the log already follows, the old project's file is closed and a new one is started in the new project's folder.
/// The app calls it on every tick, so it is cheap while nothing changes.
pub fn follow_project(project: Option<&Path>) {
    if cfg!(test) {
        return;
    }
    switch_project(project);
}

/// `follow_project` without the test guard: switches to `project`'s log when it is not the one being followed.
fn switch_project(project: Option<&Path>) {
    {
        let current = PROJECT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if current.as_ref().map(|p| p.project.as_path()) == project {
            return;
        }
    }
    // The old project's file ends here, after its last line; the new one starts with its first.
    *PROJECT.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = None;
    let Some(project) = project else { return };
    let file = project_root(project).and_then(|root| match create_in(&root, Local::now()) {
        Ok(made) => Some(made),
        Err(e) => {
            write(Level::Warn, format_args!("no log file for the project {}: {e}", project.display()));
            None
        }
    });
    let path = file.as_ref().map(|(_, p)| p.clone());
    *PROJECT.lock().unwrap_or_else(std::sync::PoisonError::into_inner) =
        Some(ProjectLog { project: project.to_path_buf(), file: file.map(|(f, _)| f) });
    if let Some(path) = path {
        write(Level::Info, format_args!("Project {} opened; its log file {}", project.display(), path.display()));
    }
}

/// Writes a line at the debug level.
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => { $crate::app_log::write($crate::app_log::Level::Debug, format_args!($($arg)*)) };
}

/// Writes a line at the info level.
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => { $crate::app_log::write($crate::app_log::Level::Info, format_args!($($arg)*)) };
}

/// Writes a line at the warning level.
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => { $crate::app_log::write($crate::app_log::Level::Warn, format_args!($($arg)*)) };
}

/// Writes a line at the error level.
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => { $crate::app_log::write($crate::app_log::Level::Error, format_args!($($arg)*)) };
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn at(h: u32, m: u32, s: u32) -> DateTime<Local> {
        Local.with_ymd_and_hms(2026, 11, 18, h, m, s).single().unwrap()
    }

    #[test]
    fn a_run_gets_a_file_named_by_its_start_in_its_day_folder() {
        let root = std::env::temp_dir().join(format!("lemon-app-log-{}", std::process::id()));
        let (_file, path) = create_in(&root, at(9, 5, 7)).unwrap();
        assert_eq!(path, root.join("2026-11-18").join("2026-11-18_09-05-07.log"));
        assert!(path.is_file());
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn a_file_already_there_is_never_overwritten() {
        let root = std::env::temp_dir().join(format!("lemon-app-log-twice-{}", std::process::id()));
        let (mut first, a) = create_in(&root, at(9, 5, 7)).unwrap();
        first.write_all(b"kept\n").unwrap();
        let (_second, b) = create_in(&root, at(9, 5, 7)).unwrap();
        assert_eq!(b.file_name().unwrap(), "2026-11-18_09-05-07_1.log");
        assert_eq!(std::fs::read_to_string(a).unwrap(), "kept\n");
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn a_line_has_the_time_level_thread_and_message() {
        let line = format_line(at(9, 5, 7), Level::Warn, "main", "hello");
        assert!(line.starts_with("2026-11-18 09:05:07."));
        assert!(line.ends_with(" WARN  [main] hello\n"));
    }

    /// The project's log is started in a `logs` folder beside the project file, gets the lines written while it is open,
    /// and stops getting them once the project is closed.
    #[test]
    fn a_project_log_follows_the_open_project() {
        let dir = std::env::temp_dir().join(format!("lemon-project-log-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let project = dir.join("song.lemon");
        let logged = || -> String {
            let day = std::fs::read_dir(dir.join("logs")).unwrap().next().unwrap().unwrap().path();
            let file = std::fs::read_dir(day).unwrap().next().unwrap().unwrap().path();
            std::fs::read_to_string(file).unwrap()
        };
        switch_project(Some(&project));
        write(Level::Info, format_args!("while open"));
        let text = logged();
        assert!(text.contains("while open") && text.contains("song.lemon"), "{text}");
        switch_project(None);
        write(Level::Info, format_args!("after close"));
        assert!(!logged().contains("after close"));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn levels_are_ordered_and_read_from_their_names() {
        assert!(Level::Debug < Level::Info && Level::Info < Level::Warn && Level::Warn < Level::Error);
        assert_eq!(Level::parse(" Warning "), Some(Level::Warn));
        assert_eq!(Level::parse("loud"), None);
    }
}
