use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

fn main() {
    let ttx_dir = ["src", "default"].iter().collect::<PathBuf>();
    let ttx_path = ttx_dir.join("default.ttx");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is not set"));
    let out_path = out_dir.join("default.ttf");

    let output = Command::new("ttx")
        .args(["-o", "-"]) // Output to stdout (which we capture)
        .arg(&ttx_path)
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

    let output = if output.stdout.is_empty() {
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
    };

    fs::write(&out_path, output)
        .unwrap_or_else(|err| panic!("failed to write to {}: {:?}", out_path.display(), err));

    println!("cargo:rustc-env=DEFAULT_TTF={}", out_path.display());

    println!("cargo:rerun-if-changed={}", ttx_path.display());
}
