use std::{
    io::{Error, ErrorKind, Read, Result, Write},
    process::{Command, Stdio},
};

fn main() {
    let blueprint = compile_blueprint(include_bytes!("data/window.blp")).unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    std::fs::write(format!("{}/window.ui", out_dir), blueprint).unwrap();

    compile_resources();
}

pub(crate) fn compile_resources() {
    glib_build_tools::compile_resources(&["data"], "data/rsicon.gresource.xml", "rsicon.gresource")
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
