use actix_web_static_files::resource_dir;
use std::process::Command;

fn main() {
    // let _ = Command::new("/bin/bash").args(&["build.sh","build"]).status();
    //
    // https://docs.rs/actix-web-static-files/0.2.4/actix_web_static_files/fn.resource_dir.html
    // Generate resources for ./frontend/dist dir with file name generated.rs
    // stored in path defined by OUT_DIR environment variable.
    // Function name is 'generate'
    resource_dir("./htdocs").build().unwrap();
}
