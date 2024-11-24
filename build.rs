use std::{
    io::{Error, ErrorKind, Read, Result, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // First compile the blueprint
    let blueprint = compile_blueprint(include_bytes!("data/window.blp")).unwrap();
    let ui_path = out_dir.join("window.ui");
    std::fs::write(&ui_path, blueprint).unwrap();

    // Create a symlink in data/ to the generated UI file
    let target_ui = PathBuf::from("data").join("window.ui");
    if target_ui.exists() {
        std::fs::remove_file(&target_ui).unwrap();
    }
    std::os::unix::fs::symlink(&ui_path, &target_ui).unwrap();

    // Now compile resources
    compile_resources();
}

pub(crate) fn compile_resources() {
    glib_build_tools::compile_resources(
        &["data"],
        "data/whisker.gresource.xml",
        "whisker.gresource",
    )
}

pub(crate) fn compile_blueprint(blueprint: &[u8]) -> Result<String> {
    let mut compiler = Command::new("blueprint-compiler")
        .args(["compile", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("blueprint-compiler not found"));

    let mut stdin = compiler.stdin.take().unwrap();
    stdin.write_all(b"using Gtk 4.0;\n")?;
    stdin.write_all(blueprint)?;
    drop(stdin);

    let mut buf = String::new();
    compiler.stdout.unwrap().read_to_string(&mut buf)?;

    if !buf.starts_with('<') {
        return Err(Error::new(ErrorKind::Other, buf));
    }

    Ok(buf)
}
