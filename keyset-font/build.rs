use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

const FONTS: [&str; 3] = ["default", "demo", "null"];

pub struct VirtualEnv {
    path: PathBuf,
}

impl VirtualEnv {
    pub fn new(path: PathBuf) -> Self {
        let system_python = ["python3", "python", "py"]
            .into_iter()
            .find_map(|py| which::which_global(py).ok())
            .expect("python not found");

        let venv = ["virtualenv", "venv"]
            .into_iter()
            .find(|module| {
                Command::new(&system_python)
                    .args(["-m", module, "--help"])
                    .stdout(Stdio::null()) // Prevent stdout going to Cargo
                    .status()
                    .is_ok_and(|status| status.success())
            })
            .expect("python module virtualenv or venv not found");

        let path = path.join(".venv");
        let success = Command::new(&system_python)
            .args(["-m", venv])
            .arg(&path)
            .stdout(Stdio::null()) // Prevent stdout going to Cargo
            .status()
            .is_ok_and(|status| status.success());
        if !success {
            panic!("failed to create virtual environment")
        }

        Self { path }
    }

    pub fn python(&self) -> Command {
        let bin_dir = self.path.join("bin");
        let mut command = Command::new(bin_dir.join("python3"));
        command.env("PATH", bin_dir).env("VIRTUAL_ENV", &self.path);
        command
    }
}

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set"));

    let ttx_src = manifest_dir.join("resources").join("fonts");
    let ttf_dst = out_dir.join("resources").join("fonts");
    fs::create_dir_all(&ttf_dst)
        .unwrap_or_else(|err| panic!("failed to create directory {}: {err:?}", out_dir.display()));

    if std::env::var("DOCS_RS").is_ok() {
        // In docs.rs' environment we have no network access to install fonttools, so just write
        // blank files (which is sufficient for docs anyway)
        for font in FONTS {
            let ttf = ttf_dst.join(format!("{font}.ttf"));
            fs::File::create(&ttf).expect("failed to create file");
            println!(
                "cargo:rustc-env={}_TTF={}",
                font.to_uppercase(),
                ttf.display()
            );
        }
    } else {
        let requirements = manifest_dir.join("requirements.txt");
        println!("cargo:rerun-if-changed={}", requirements.display());

        let venv = VirtualEnv::new(out_dir);
        let success = venv
            .python()
            .args(["-m", "pip", "install", "-Ur"])
            .arg(requirements)
            .stdout(Stdio::null()) // Prevent stdout going to Cargo
            .status()
            .is_ok_and(|status| status.success());
        if !success {
            panic!("failed to install requirements in virtual environment");
        }

        for font in FONTS {
            let ttx = ttx_src.join(format!("{font}.ttx"));
            let ttf = ttf_dst.join(format!("{font}.ttf"));

            let success = venv
                .python()
                .args(["-m", "fontTools.ttx"])
                .arg("-o")
                .args([&ttf, &ttx])
                .stdout(Stdio::null()) // Prevent stdout going to Cargo
                .status()
                .is_ok_and(|status| status.success());
            if !success {
                panic!("failed to run ttx");
            }

            println!(
                "cargo:rustc-env={}_TTF={}",
                font.to_uppercase(),
                ttf.display()
            );
            println!("cargo:rerun-if-changed={}", ttx.display());
        }
    }
}
