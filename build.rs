use std::env;

fn main() {
    let bakkesmod_path = env::var("BAKKESMOD_LIB_PATH")
        .expect("enviroment variable 'BAKKESMOD_LIB_PATH' not set!");

    println!("cargo:rustc-link-search={}", bakkesmod_path);
    println!("cargo:rustc-link-lib=pluginsdk");
}