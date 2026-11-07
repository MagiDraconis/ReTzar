# ReTzar for Android (work in progress)

The Android build is a thin shell around the desktop front end. `src/lib.rs`
includes `../src/frontend` unchanged and adds only what Android does
differently: the `android_main` entry point, where the data lives, and touch
input (`../src/frontend/touch.rs`, compiled on Android only). The root
`Cargo.toml` and `Cargo.lock` are not touched, so this tree merges cleanly with
upstream.

## Build

CI does it (`.github/workflows/android.yml`) and uploads the APK. Locally:

```sh
rustup target add aarch64-linux-android
cargo install cargo-ndk
export ANDROID_NDK_HOME=/path/to/ndk          # NDK r26 or newer
cd android
cp ../Cargo.lock Cargo.lock                   # same versions as the desktop build
cargo ndk -t arm64-v8a -o app/src/main/jniLibs build --release
gradle assembleDebug                          # Gradle 8.9+, JDK 17
adb install -r app/build/outputs/apk/debug/app-debug.apk
```

## Game data

ReTzar needs your own copy of Tzar. Copy its folder (the one with `Maps` and
`DATA`/`IMAGES`, or `Packs`) to the app's data folder:

```sh
adb shell mkdir -p /sdcard/Android/data/org.retzar.android/files
adb push /path/to/Tzar /sdcard/Android/data/org.retzar.android/files/Tzar
```

Start the app once first if the folder does not exist yet. Errors go to
`adb logcat -s ReTzar`.

## State

Step 1 of the port: build, boot, data folder, one finger as the left mouse
button. Known gaps, in the order they are planned:

1. Folder picker (Storage Access Framework) instead of `adb push`.
2. Touch gestures for the game: orders (long press), camera pan, pinch zoom.
3. Sound and MIDI (native backends exist for Windows and macOS only).
4. Videos (the intro is skipped).

Nothing here has been run on a device yet. While the finger moves, bevy_winit
logs "could not set cursor position" because Android cannot warp the pointer;
the message is harmless.
