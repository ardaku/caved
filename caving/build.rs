extern crate cc;

fn main() {
    cc::Build::new()
        .include("../ffmpeg-4.1")
        .file("ffplay.c")
        .file("cmdutils.c")
        .compile("ffplay");

    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-search=src/x86_64");

    // FFMPEG Static Libraries:
    println!("cargo:rustc-link-lib=static=avformat"); // -lavformat -lm -lbz2 -lz
        println!("cargo:rustc-link-lib=static=avcodec"); // -lavcodec -pthread -lm -lz
            println!("cargo:rustc-link-lib=static=swresample"); // -lswresample -lm
        println!("cargo:rustc-link-lib=bz2");
        println!("cargo:rustc-link-lib=z");

    // TODO: Use Vulkan/OpenGL scaling instead of this library.
    println!("cargo:rustc-link-lib=static=swscale"); // -lswscale -lm 
        println!("cargo:rustc-link-lib=static=avutil"); // -lavutil -pthread -lm
}
