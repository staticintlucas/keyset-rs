#![allow(clippy::expect_fun_call)]

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
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let font_dir = manifest_dir.join("resources").join("fonts");
    let ttx_files = files_with_extension(&font_dir, "ttx")
        .expect(&format!("failed to list files in {font_dir:?}"));

    for ttx in ttx_files {
        let name = ttx.file_stem().unwrap();
        let env_var = name.to_string_lossy().to_uppercase() + "_TTF";
        let ttf = ttx.with_extension("ttf");

        assert!(
            ttf.exists(),
            "TTF file {ttf:?} not found!\n\nPlease run `ttx -o {ttf:?} {ttx:?}`"
        );

        let ttx_mtime = ttx
            .metadata()
            .and_then(|m| m.modified())
            .expect(&format!("error retrieving metadata for {ttx:?}"));
        let ttf_mtime = ttf
            .metadata()
            .and_then(|m| m.modified())
            .expect(&format!("error retrieving metadata for {ttf:?}"));

        assert!(
            ttf_mtime >= ttx_mtime - Duration::from_micros(1),
            "TTF file {ttf:?} is out of date!

Please run `ttx -o {ttf:?} {ttx:?}`

{ttf:?}: {ttf_mtime:?}
{ttx:?}: {ttx_mtime:?}"
        );

        println!("cargo:rustc-env={env_var}={}", ttf.display());
        println!("cargo:rerun-if-changed={}", ttx.display());
    }
}
