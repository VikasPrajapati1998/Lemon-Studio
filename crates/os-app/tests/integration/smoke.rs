// Copyright (C) 2026 ItsAlphaNeon
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of Lemon Studio: https://github.com/ItsAlphaNeon/Lemon-Studio

//! The smoke test: the built `lemonstudio` starts without a window, renders a small project to a WAV file, and keeps
//! its run log in `logs/YYYY-MM-DD/` of the user folder. It needs no audio device and no plugins beyond the built-in
//! ones. Run it with `cargo test -p os-app --test integration smoke`.

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

/// The longest a run of the program may take before the test gives up on it.
const TIMEOUT: Duration = Duration::from_secs(180);

/// A folder of the test's own, empty, removed when the test ends.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let dir = std::env::temp_dir().join(format!("lemon-smoke-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch folder");
        Scratch(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Runs `lemonstudio args...` with its user folder (settings, logs) at `user`, and waits for it to end.
fn run(args: &[&std::ffi::OsStr], user: &Path) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_lemonstudio"))
        .args(args)
        .env("OPENSTUDIO_USER_DIR", user)
        .env_remove("OPENSTUDIO_LOG_DIR")
        .env_remove("OPENSTUDIO_LOG")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start lemonstudio");
    let started = Instant::now();
    loop {
        if child.try_wait().expect("wait for lemonstudio").is_some() {
            return child.wait_with_output().expect("collect the output");
        }
        if started.elapsed() > TIMEOUT {
            let _ = child.kill();
            panic!("lemonstudio did not end within {TIMEOUT:?}");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// An empty project with the mixer of a new one, written as an `.flp` file in `dir`.
fn write_project(dir: &Path) -> PathBuf {
    let project = os_model::Project {
        ppq: 96,
        tempo: Some(130.0),
        current_pattern: Some(1),
        patterns: vec![os_model::Pattern { number: 1, ..os_model::Pattern::default() }],
        mixer: (0..18u16).map(os_model::MixerTrack::new).collect(),
        ..os_model::Project::default()
    };
    let file = os_flp::project::create(&project).expect("create the project file");
    let path = dir.join("smoke.flp");
    std::fs::write(&path, file.to_bytes().expect("project bytes")).expect("write the project");
    path
}

/// The run logs under `user`: `logs/<day>/<file>.log`, as (day folder name, file name, text).
fn run_logs(user: &Path) -> Vec<(String, String, String)> {
    let mut found = Vec::new();
    let Ok(days) = std::fs::read_dir(user.join("logs")) else { return found };
    for day in days.flatten() {
        for file in std::fs::read_dir(day.path()).into_iter().flatten().flatten() {
            let text = std::fs::read_to_string(file.path()).unwrap_or_default();
            found.push((day.file_name().to_string_lossy().into_owned(), file.file_name().to_string_lossy().into_owned(), text));
        }
    }
    found
}

/// Whether `name` has the shape `shape`, where `9` stands for a digit and every other character for itself.
fn has_shape(name: &str, shape: &str) -> bool {
    name.len() == shape.len() && name.chars().zip(shape.chars()).all(|(c, s)| if s == '9' { c.is_ascii_digit() } else { c == s })
}

#[test]
fn it_renders_a_project_without_a_window_and_keeps_a_dated_log() {
    let scratch = Scratch::new("render");
    let user = scratch.path().join("user");
    let project = write_project(scratch.path());
    let wav = scratch.path().join("smoke.wav");

    let out = run(&["--render".as_ref(), project.as_os_str(), wav.as_os_str(), "--seconds".as_ref(), "1".as_ref(), "--builtin-only".as_ref()], &user);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "the render failed ({:?}):\n{stderr}", out.status.code());

    let audio = std::fs::read(&wav).expect("the render's WAV file");
    assert_eq!(&audio[..4], b"RIFF", "a WAV file");
    assert!(audio.len() > 44, "the WAV file holds audio data");

    let logs = run_logs(&user);
    assert_eq!(logs.len(), 1, "one log file for one run: {logs:?}");
    let (day, file, text) = &logs[0];
    assert!(has_shape(day, "9999-99-99"), "the day folder: {day}");
    assert!(has_shape(file, "9999-99-99_99-99-99.log"), "the file name: {file}");
    assert!(file.starts_with(day.as_str()), "the file is in its own day's folder");
    assert!(text.contains(" INFO  ") && text.contains("started"), "the log's first line:\n{text}");
}

#[test]
fn a_render_without_files_is_refused_with_the_usage_exit_code() {
    let scratch = Scratch::new("usage");
    let user = scratch.path().join("user");
    let out = run(&["--render".as_ref()], &user);
    assert_eq!(out.status.code(), Some(2), "wrong arguments end with 2");
    assert!(String::from_utf8_lossy(&out.stderr).contains("usage: lemonstudio --render"));
    assert!(!run_logs(&user).is_empty(), "even a refused run has its log");
}
