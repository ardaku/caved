extern crate cc;

// gcc ffplay.c cmdutils.c -lSDL2 -lm -lavcodec -lavdevice -lavutil -lavfilter -lswresample -lswscale -lavformat -I../

fn main() {
    cc::Build::new()
//        .warnings(false)
        .include("../ffmpeg-4.1")
        .file("ffplay.c")
        .file("cmdutils.c")
        .compile("ffplay");

    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avdevice");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=avfilter");
    println!("cargo:rustc-link-lib=swresample");
    println!("cargo:rustc-link-lib=swscale");
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=SDL2");
}
