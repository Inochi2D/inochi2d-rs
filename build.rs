
use std::{
    env,
    path::PathBuf,
    process::Command
};

const INOCHI2D_REPO: &'static str = "https://github.com/Inochi2D/inochi2d.git";
const INOCHI2D_C_REPO: &'static str = "https://github.com/Inochi2D/inochi2d-c.git";

#[cfg(feature = "opengl")]
const WITH_OPENGL: bool = true;
#[cfg(not(feature = "opengl"))]
const WITH_OPENGL: bool = false;


fn clone_repo(repo: &'static str, name: &'static str) -> PathBuf {
    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap()).join(name);

    if !outdir.exists() {
        let cmd_status = Command::new("git")
            .arg("clone")
            .args(&["--depth", "1"])
            .arg(repo)
            .arg(&outdir)
            .status()
            .unwrap();

        assert!(cmd_status.success(), "Failed to clone repo");
    }
    outdir

}

fn dub_init() {
    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let dub_status = Command::new("dub")
    .arg("add-path")
    .arg(&outdir)
    .arg("--cache=local")
    .arg("--verror")
    .status()
    .unwrap();

    assert!(dub_status.success(), "Failed to setup dub");

}

fn dub_build(dir: &PathBuf, cfg: &'static str) {
    let dub_status = Command::new("dub")
        .current_dir(dir)
        .arg("build")
        .arg("--non-interactive")
        .arg("--config")
        .arg(cfg)


        .arg("--cache=local")
        .arg("--verror")
        .status()
        .unwrap();

    assert!(dub_status.success(), "Failed to dub build");

}

fn main() {

    let inochi2d = clone_repo(INOCHI2D_REPO, "inochi2d");
    let inochi2d_c = clone_repo(INOCHI2D_C_REPO, "inochi2d-c");

    let libdir = inochi2d_c.join("out").clone();

    if !libdir.join("libinochi2d-c.so").exists() {
        dub_init();
        if WITH_OPENGL {
            dub_build(&inochi2d, "full");
            dub_build(&inochi2d_c, "yesgl");
        } else {
            dub_build(&inochi2d, "renderless");
            dub_build(&inochi2d_c, "nogl");
        }
    }

    println!("cargo:rustc-link-lib=inochi2d-c");
    println!("cargo:rustc-link-arg=-L{}", libdir.display());
}
