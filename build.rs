use std::process::Command;

fn main() {
    // let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    compile_scss();

    compile_resources();
}

pub(crate) fn compile_scss() {
    let scss_input = "data/main.scss";
    let css_output = "data/main.css";
    let mut sass_compiler = Command::new("sass")
        .args(["compile", scss_input, css_output])
        .spawn()
        .unwrap_or_else(|_| panic!("sass not found"));

    sass_compiler.wait().unwrap();
}

pub(crate) fn compile_resources() {
    glib_build_tools::compile_resources(
        &["data"],
        "data/whisker.gresource.xml",
        "whisker.gresource",
    )
}
