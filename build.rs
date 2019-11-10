use std::process::{Command, Output};
use std::ops::Deref;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use path_dsl::path;

fn print_output(command: &Command, output: &Output) {
    println!("===========COMMAND===========");
    println!("{:#?}", command);
    println!("===========STDOUT===========");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("===========STDERR===========");
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

fn run_qmake() -> std::io::Result<String> {
    let mut command = Command::new("/home/connor/Qt/5.12.5/gcc_64/bin/qmake");
    let (full, dir) = match std::env::var("PROFILE").as_ref().map(|x| x.deref()) {
        Ok("debug") => {
            let dir = "target/debug/calc-ui";
            create_dir_all(dir)?;
            (command.args(&["../../../calc-ui/calc-ui.pro", "CONFIG+=debug", "CONFIG+=qml_debug"]).current_dir(dir), dir)
        },
        Ok("release") => {
            let dir = "target/release/calc-ui";
            create_dir_all(dir)?;
            (command.args(&["../../../calc-ui/calc-ui.pro", "CONFIG+=qtquickcompiler"]).current_dir(dir), dir)
        },
        _ => panic!("Unknown env value in PROFILE")
    };
    let output = full.output()?;
    if output.status.success() {
        Ok(dir.into())
    } else {
        print_output(&command, &output);

        Err(std::io::ErrorKind::Other.into())
    }
}

fn run_make<S: AsRef<Path>>(dir: S) -> std::io::Result<()> {
    let mut command =  Command::new("make");
    let output = command.current_dir(dir).output()?;
    if output.status.success() {
        Ok(())
    } else {
        print_output(&command, &output);

        Err(std::io::ErrorKind::Other.into())
    }
}

fn validate_link<S: AsRef<Path>>(dir: S) -> std::io::Result<()> {
    let p: PathBuf = path!((dir.as_ref()) | "libcalc-ui.so").into();
    let dir_string = dir.as_ref().to_str().unwrap();
    if !p.exists() {
        eprintln!("Path {:?} does not exist.", p);
        Err(std::io::ErrorKind::Other.into())
    } else if !p.is_file() {
        eprintln!("Path {:?} is not a file.", p);
        Err(std::io::ErrorKind::Other.into())
    } else {
        println!("cargo:rustc-link-search=native={}", dir_string);
        println!("cargo:rustc-link-lib=dylib=calc-ui");
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let folder = run_qmake()?;
    run_make(&folder)?;
    validate_link(folder)?;

    Ok(())
}
