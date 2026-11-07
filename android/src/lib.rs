//! Android shell of ReTzar.
//!
//! The game itself is the desktop front end, compiled unchanged into this
//! library; only what Android does differently lives here: the activity entry
//! point, where the data is, and touch input.
#![cfg(target_os = "android")]

// The desktop front end, included from its place in the original tree.
#[path = "../../src/frontend/mod.rs"]
mod frontend;

use bevy::android::{ANDROID_APP, android_activity::AndroidApp};
use retzar::paths::{Locations, is_installation};
use std::{
    ffi::CString,
    os::raw::{c_char, c_int},
};

#[link(name = "log")]
unsafe extern "C" {
    fn __android_log_write(priority: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

const LOG_ERROR: c_int = 6;

/// A line in `adb logcat -s ReTzar`.
fn log(priority: c_int, text: &str) {
    let text = CString::new(text.replace('\0', " ")).unwrap_or_default();
    // SAFETY: both strings are NUL-terminated and outlive the call.
    unsafe {
        __android_log_write(priority, c"ReTzar".as_ptr(), text.as_ptr());
    }
}

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    let _ = ANDROID_APP.set(app.clone());
    std::panic::set_hook(Box::new(|info| {
        log(
            LOG_ERROR,
            &format!("panic: {info}\n{}", std::backtrace::Backtrace::force_capture()),
        );
    }));
    if let Err(error) = start(&app) {
        log(LOG_ERROR, &error.to_string());
    }
}

/// Settings and saves live in the app's private storage; the original game's
/// files are read from a `Tzar` folder in the app's data folders (see README).
fn start(app: &AndroidApp) -> retzar::Result<()> {
    let root = app
        .internal_data_path()
        .ok_or("Android reports no private data folder")?;
    let candidates: Vec<_> = [app.external_data_path(), app.internal_data_path()]
        .into_iter()
        .flatten()
        .map(|dir| dir.join("Tzar"))
        .collect();
    let source = candidates
        .iter()
        .find(|dir| is_installation(dir))
        .ok_or_else(|| {
            format!(
                "Tzar installation not found. Copy the game's folder (Maps, DATA, IMAGES or Packs) to one of: {}",
                candidates
                    .iter()
                    .map(|d| d.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;
    let args = [
        "--root".to_string(),
        root.display().to_string(),
        "--source".to_string(),
        source.display().to_string(),
    ];
    let locations = Locations::resolve(&args)?;
    frontend::run(
        &locations,
        frontend::Launch {
            // The intro movie needs the video backend, still missing here.
            intro: false,
            ..Default::default()
        },
    )
}
