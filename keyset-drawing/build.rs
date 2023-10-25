use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

fn main() {
    let ttx_dir = ["resources", "fonts"].iter().collect::<PathBuf>();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is not set")).join(&ttx_dir);

    fs::create_dir_all(&out_dir)
        .unwrap_or_else(|err| panic!("failed to create directory {}: {err:?}", out_dir.display()));

    let fonts = ["demo"];

    for font in fonts {
        let ttx = ttx_dir.join(format!("{font}.ttx"));
        let ttf = out_dir.join(format!("{font}.ttf"));

        let contents = ttx_to_ttf(&ttx);
        fs::write(&ttf, contents)
            .unwrap_or_else(|err| panic!("failed to write to {}: {err:?}", ttf.display()));

        println!(
            "cargo:rustc-env={}_TTF={}",
            font.to_uppercase(),
            ttf.display()
        );
        println!("cargo:rerun-if-changed={}", ttx.display());
    }

    // Rerun if a different ttx is on PATH (e.g. one from a virtualenv)
    println!("cargo:rerun-if-env-changed=PATH");
}

fn ttx_to_ttf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();

    let output = Command::new("ttx")
        .args(["-o", "-"]) // Output to stdout (which we capture)
        .arg(path)
        .output()
        .expect("failed to run ttx");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let code = output.status.code();
        #[cfg(unix)]
        let code = code.or(output.status.signal());
        let code = code.map(|i| i.to_string()).unwrap_or("unknown".to_string());

        panic!("{stderr}\nttx failed with exit status {code}");
    }

    if output.stdout.is_empty() {
        // Work around bug (?) in older TTX where it writes to a file called "-". (I'm calling it a
        // bug because --help says it should write to stdout, although there doesn't seem to be any
        // code to actually handle this prior to 4.39)
        if let Ok(output) = fs::read("-") {
            let _ = fs::remove_file("-");
            output
        } else {
            panic!("failed to capture output from ttx");
        }
    } else {
        output.stdout
    }
}
