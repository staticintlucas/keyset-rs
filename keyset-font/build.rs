#![allow(missing_docs, clippy::pedantic, clippy::restriction)] // This is just a build.rs

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, fs, io};

fn files_with_extension(
    path: impl AsRef<Path>,
    extension: impl AsRef<OsStr>,
) -> io::Result<impl Iterator<Item = PathBuf>> {
    Ok(fs::read_dir(path.as_ref())?
        .filter_map(|r| r.map(|f| f.path()).ok())
        .filter(move |p| matches!(p.extension(), Some(e) if e == extension.as_ref())))
}

fn main() {
    let workspace_dir =
        PathBuf::from(env::var_os("CARGO_WORKSPACE_DIR").expect("CARGO_WORKSPACE_DIR not set"));
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));

    let font_dir = manifest_dir.join("resources").join("fonts");
    let ttx_files = files_with_extension(&font_dir, "ttx")
        .unwrap_or_else(|e| panic!("failed to list files in {font_dir:?}: {e:?}"));

    for ttx in ttx_files {
        let env_var = ttx.file_stem().unwrap().to_string_lossy().to_uppercase() + "_TTF";
        let ttf = ttx.with_extension("ttf");

        let ttx_str = ttx.strip_prefix(&workspace_dir).unwrap().to_string_lossy();
        let ttf_str = ttf.strip_prefix(&workspace_dir).unwrap().to_string_lossy();

        assert!(
            ttf.exists(),
            "Font {ttf_str} not found!\n\nPlease run `ttx -o {ttf_str} {ttx_str}`"
        );

        let ttx_mtime = ttx
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or_else(|e| panic!("error retrieving metadata for {ttx_str}: {e:?}"));
        let ttf_mtime = ttf
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or_else(|e| panic!("error retrieving metadata for {ttf_str}: {e:?}"));

        // rather than just checking if the TTF is newer than the TTX, use a 10ms tolerance so
        // if the TTF is created first by `git clone` it won't error out
        let tolerance = Duration::from_millis(50);
        assert!(
            ttf_mtime >= ttx_mtime - tolerance,
            "Font {ttf_str} is out of date!\n\nPlease run `ttx -o {ttf_str} {ttx_str}`\n\n\
            ttf: {ttf_mtime:?}\nttx: {ttx_mtime:?}"
        );

        println!("cargo:rustc-env={env_var}={}", ttf.display());
        println!("cargo:rerun-if-changed={}", ttx.display());
    }
}
