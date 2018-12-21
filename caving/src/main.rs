use std::env;
use std::os::raw::*;
use std::ffi::CString;

extern "C" {
    fn start(filename: *const c_char /*int argc, char **argv*/) -> ();
}

fn load_file_from_arg() -> String {
    for argument in env::args().skip(1) {
        return argument.to_string();
    }

    println!("usage: caving [filename]");
    ::std::process::exit(1);
}

fn play(filename: &str) -> std::io::Result<()> {
    let c_filename = CString::new(filename.as_bytes()).unwrap();

    unsafe {
        start(c_filename.as_ptr())
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let file = load_file_from_arg();

    println!("CAVING > Playing \"{}\"", file);

    play(&file)
}
