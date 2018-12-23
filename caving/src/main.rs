// This example program may be used for any purpose proprietary or not.

#[macro_use]
extern crate adi;
extern crate barg;
extern crate libc;
extern crate miniz_oxide;

use std::env;
use std::ffi::CString;
use std::fs::File;
use std::io::Write;

use adi::{
    hid,
    screen::{ self, Viewer, ColorChannels, prelude::{self, *} },
    App,
};
use barg::*;
use libc::*;
// use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};

main!(
    Ctx,
    struct Ctx {
        // Time
        time: f32,
        // Frames
        frames: u32,
        // Barg Font
        font: FontChain<'static>,
        // FPS / Info String
        info: String,
        // The mode
        mode: fn(app: &mut Ctx),
        // Texture that holds a frame.
        texture: Texture,
        // Viewer for frame
//        viewer: Viewer,
    }
);

impl App for Ctx {
    fn new() -> Ctx {
        let file = load_file_from_arg();

        println!("CAVING > Playing \"{}\"", file);

        play(&file).unwrap();

        unsafe {
            println!("CAVING > video_width: {}", GLOBAL.video_width);
            println!("CAVING > video_height: {}", GLOBAL.video_height);
            println!("CAVING > pixel_format: {:?}", GLOBAL.pixel_format);
            println!("CAVING > audio_channels: {}", GLOBAL.audio_channels);
            println!("CAVING > audio_samplerate: {}", GLOBAL.audio_samplerate);
            println!("CAVING > audio_sampleformat: {}", GLOBAL.audio_sampleformat);
        }

        let texture = screen::texture(unsafe {
            (GLOBAL.video_width as u16, GLOBAL.video_height as u16) }, unsafe {&VFrame(vec![255; GLOBAL.video_width as usize * GLOBAL.video_height as usize * 4]) }
        );
        let tc = screen::texcoords(&[
	        (0.0, 0.0),
	        (0.0, 1.0),
	        (1.0, 1.0),
	        (1.0, 0.0),

	        (0.0, 0.0),
	        (0.0, 1.0),
	        (1.0, 1.0),
	        (1.0, 0.0),
        ]);

        let model = ModelBuilder::new()
            .vert(&[
	            prelude::Move(-0.5, -0.5, 1.0),
	            prelude::Line(0.5, -0.5, 1.0),
	            prelude::Line(0.5, 0.5, 1.0),
	            prelude::Line(-0.5, 0.5, 1.0),
            ])
            .dface(matrix!())
            .close();

        let viewer = Viewer::new(vector!(), vector!());
        let _a = viewer.add_textured(&model, matrix!(), &texture, tc, false);

        ::std::mem::forget(model);
        ::std::mem::forget(tc);
        ::std::mem::forget(viewer);

        Ctx {
            time: 0.0,
            frames: 0,
            font: FontChain::default(),
            info: "".to_string(),
            mode: mode_load,
            texture,
//            viewer,
        }
    }

    fn run(&mut self) {
        (self.mode)(self)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[allow(unused)]
enum AVPixelFormat {
    None = -1,
    Yuv420p,
    Yuyv422,
    Rgb24,
    Bgr24,
    Yuv422p,
    Yuv444p,
    Yuv410p,
    Yuv411p,
    Gray8,
    Monowhite,
    Monoblack,
    Pal8,
    Yuvj420p,
    Yuvj422p,
    Yuvj444p,
    Uyvy422,
    Uyyvyy411,
    Bgr8,
    Bgr4,
    Bgr4Byte,
    Rgb8,
    Rgb4,
    Rgb4Byte,
    Nv12,
    Nv21,

    Argb,
    Rgba,
    Abgr,
    Bgra,

    Gray16be,
    Gray16le,
    Yuv440p,
    Yuvj440p,
    Yuva420p,
    Rgb48be,
    Rgb48le,

    Rgb565be,
    Rgb565le,
    Rgb555be,
    Rgb555le,

    Bgr565be,
    Bgr565le,
    Bgr555be,
    Bgr555le,
    // TODO: Some weird macro stuff, don't know.
    /*  // #if FF_API_VAAPI
    109     VAAPI_MOCO,
    110     VAAPI_IDCT,
    111     VAAPI_VLD,
    112     VAAPI = VAAPI_VLD,
    // #else
    114     VAAPI,
    // #endif
    116
    117     YUV420P16LE,
    118     YUV420P16BE,
    119     YUV422P16LE,
    120     YUV422P16BE,
    121     YUV444P16LE,
    122     YUV444P16BE,
    123     DXVA2_VLD,
    124
    125     RGB444LE,
    126     RGB444BE,
    127     BGR444LE,
    128     BGR444BE,
    129     YA8,
    130
    131     Y400A = YA8,
    132
    133     BGR48BE,
    134     BGR48LE,
    135     YUV420P9BE,
    136     YUV420P9LE,
    137     YUV420P10BE,
    138     YUV420P10LE,
    139     YUV422P10BE,
      YUV422P10LE,
      YUV444P9BE,
      YUV444P9LE,
      YUV444P10BE,
      YUV444P10LE,
      YUV422P9BE,
      YUV422P9LE,
      VDA_VLD,
      GBRP,
      GBRP9BE,
      GBRP9LE,
      GBRP10BE,
      GBRP10LE,
      GBRP16BE,
      GBRP16LE,
      YUVA422P,
      YUVA444P,
      YUVA420P9BE,
      YUVA420P9LE,
      YUVA422P9BE,
      YUVA422P9LE,
      YUVA444P9BE,
      YUVA444P9LE,
      YUVA420P10BE,
      YUVA420P10LE,
      YUVA422P10BE,
      YUVA422P10LE,
      YUVA444P10BE,
      YUVA444P10LE,
      YUVA420P16BE,
      YUVA420P16LE,
      YUVA422P16BE,
      YUVA422P16LE,
      YUVA444P16BE,
      YUVA444P16LE,
      VDPAU,
      XYZ12LE,
      XYZ12BE,
      NV16,
      NV20LE,
      NV20BE,

      RGBA64BE,
      RGBA64LE,
      BGRA64BE,
      BGRA64LE,

      YVYU422,

      VDA,

      YA16BE,
      YA16LE,

      GBRAP,
      GBRAP16BE,
      GBRAP16LE,

      QSV,
      MMAL,

      D3D11VA_VLD,

      CUDA,

      P010LE,
      P010BE,

      YUV420P12BE,
      YUV420P12LE,

      YUV422P12BE,
      YUV422P12LE,

      YUV444P12BE,
      YUV444P12LE,

      GBRP12BE,
      GBRP12LE,

      GBRAP12BE,
      GBRAP12LE,

      GRAY12BE,
      GRAY12LE,

      GBRAP10BE,
      GBRAP10LE,

      D3D11,

      GRAY10BE,
      GRAY10LE,

      NB,        */
}

extern "C" {
//    fn caving_decode_video(filename: *const c_char) -> ();
//    fn caving_decode_audio(filename: *const c_char) -> ();

    fn caving_decode_new(
        src_filename: *const c_char,
        video_width: *mut c_int,
        video_height: *mut c_int,
        pixel_format: *mut AVPixelFormat,
        audio_channels: *mut c_int,
        audio_samplerate: *mut c_int,
        audio_sampleformat: *mut AVSampleFormat,
    ) -> ();

    fn caving_decode_run(
        video_write: unsafe extern "C" fn(
            data_y: *mut c_void,
            data_cb: *mut c_void,
            data_cr: *mut c_void,
            pitch_y: size_t,
            pitch_cb: size_t,
            pitch_cr: size_t,
        ),
        audio_write: unsafe extern "C" fn(data: *mut c_void, size: size_t),
    ) -> bool;
}

fn load_file_from_arg() -> String {
    for argument in env::args().skip(1) {
        return argument.to_string();
    }

    println!("usage: caving [filename]");
    ::std::process::exit(1);
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(unused)]
enum AVSampleFormat {
    None = -1,
    U8,
    S16,
    S32,
    Flt,
    Dbl,

    U8p,
    S16p,
    S32p,
    Fltp,
    Dblp,

    Nb,
}

impl std::fmt::Display for AVSampleFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::AVSampleFormat::*;

        match self {
            None => write!(f, "ERROR: None"),
            U8 => write!(f, "u8"),
            S16 => write!(f, "s16"),
            S32 => write!(f, "s32"),
            Flt => write!(f, "f32"),
            Dbl => write!(f, "f64"),
            U8p => write!(f, "u8p"),
            S16p => write!(f, "s16p"),
            S32p => write!(f, "s32p"),
            Fltp => write!(f, "f32p"),
            Dblp => write!(f, "f64p"),
            Nb => write!(f, "ERROR: Nb"),
        }
    }
}

struct VideoBuffer {
    current: Vec<u8>,
    overflow: Vec<u8>,
}

struct Global {
    set: bool,
    image: Option<VFrame>,
    video_width: c_int,
    video_height: c_int,
    pixel_format: AVPixelFormat,
    audio_channels: c_int,
    audio_samplerate: c_int,
    audio_sampleformat: AVSampleFormat,
//    video: Option<VideoBuffer>,
//    audio: Option<Vec<u8>>,
}

static mut GLOBAL: Global = Global {
    set: true,
    image: None,
    video_width: 0,
    video_height: 0,
    pixel_format: AVPixelFormat::None,
    audio_channels: 0,
    audio_samplerate: 0,
    audio_sampleformat: AVSampleFormat::None,
//    video: None,
//    audio: None,
};

unsafe extern "C" fn video_write(data_y: *mut c_void, data_cb: *mut c_void, data_cr: *mut c_void,
    pitch_y: size_t, pitch_cb: size_t, pitch_cr: size_t,
) {
    unsafe { GLOBAL.set = false; }

    if let Some(vframe/*(pixels, pitch, height)*/) = &mut GLOBAL.image {
        let pixels = vframe.0.as_mut_ptr();
        let pitch = GLOBAL.video_width as isize * 4;

        for i in 0..(GLOBAL.video_height as isize)/*.min(height)*/ {
            let data_y = data_y.offset(i * (pitch_y as isize)) as *mut _ as *mut u8;
            let data_cb = data_cb.offset((i >> 1) * (pitch_cb as isize)) as *mut _ as *mut u8;
            let data_cr = data_cr.offset((i >> 1) * (pitch_cr as isize)) as *mut _ as *mut u8;

            for j in 0..(GLOBAL.video_width as isize) {
                let [r, g, b, a] = ColorChannels::Srgb.from(ColorChannels::YuvNtsc, [*data_y.offset(j), *data_cb.offset(j >> 1), *data_cr.offset(j >> 1), 255]);
                *pixels.offset(i * pitch + j * 4 + 0) = r;
                *pixels.offset(i * pitch + j * 4 + 1) = g;
                *pixels.offset(i * pitch + j * 4 + 2) = b;
            }
        }
    } else {
        panic!("Error on video write!");
    }
}

unsafe extern "C" fn audio_write(data: *mut c_void, size: size_t) {
/*    if let Some(ref mut audio) = GLOBAL.audio {
        let mut file = File::create(format!(".caving/rawz/a-{}", 0)).unwrap();

        file.write_all(
            compress_to_vec(std::slice::from_raw_parts(data as *mut _, size as usize), 8)
                .as_slice(),
        )
        .unwrap();
    } else {
        GLOBAL.audio = Some(vec![]);
        audio_write(data, size)
    }*/
}

fn play(filename: &str) -> std::io::Result<()> {
    let c_filename = CString::new(filename.as_bytes())?;

    unsafe {
        caving_decode_new(
            c_filename.as_ptr(),
            &mut GLOBAL.video_width,
            &mut GLOBAL.video_height,
            &mut GLOBAL.pixel_format,
            &mut GLOBAL.audio_channels,
            &mut GLOBAL.audio_samplerate,
            &mut GLOBAL.audio_sampleformat,
        );

        GLOBAL.image = Some(VFrame(vec![255; GLOBAL.video_width as usize * GLOBAL.video_height as usize * 4]));

//        caving_decode_video(c_filename.as_ptr());
//        caving_decode_audio(c_filename.as_ptr());
    }

    Ok(())/*unsafe {
        caving_decode_new(
            c_filename.as_ptr(),
            &mut GLOBAL.video_width,
            &mut GLOBAL.video_height,
            &mut GLOBAL.pixel_format,
            &mut GLOBAL.audio_channels,
            &mut GLOBAL.audio_samplerate,
            &mut GLOBAL.audio_sampleformat,
        )
    })*/
}

fn mode_load(app: &mut Ctx) {
    // Check for exit request
    if hid::Key::Back.pressed(0) {
        adi::old();
    }

    // Begin timing
    app.time += screen::dt();
    app.frames += 1;

    if app.time >= 1.0 {
        app.info = format!(
            "Caving {} - FPS: {}\nDecoding-Playing… {}",
            env!("CARGO_PKG_VERSION"),
            app.frames,
            true,
        );
        app.time -= 1.0;
        app.frames = 0;
    }

    unsafe { GLOBAL.set = true; }
    while unsafe { GLOBAL.set } {
        if unsafe { caving_decode_run(video_write, audio_write) } {
            app.mode = mode_stop;
        }
    }
    screen::texture_set(&mut app.texture, unsafe {
        (GLOBAL.video_width as u16, GLOBAL.video_height as u16)
    }, unsafe { GLOBAL.image.as_ref().unwrap() });

    // Render
    screen::draw(&mut |pixel_buffer| {
        let (_w, h) = screen::wh();
        let mut image = Image::new(Size((screen::pitch() / 4) as u16, h));

        image.clear_ptr(pixel_buffer);

        image.text_ptr(
            [255, 255, 255, 255], // White
            (5.0, 5.0, 36.0),     // Pos. & Size
            &app.font,            // Font
            &app.info,            // String
            pixel_buffer,
        );
    });
}

// Code that runs every frame.
fn mode_stop(app: &mut Ctx) {
    // Check for exit request
    if hid::Key::Back.pressed(0) {
        adi::old();
    }

    // Begin timing
    app.time += screen::dt();
    app.frames += 1;

    if app.time >= 1.0 {
        app.info = format!(
            "Caving {} - FPS: {}\nFinished!",
            env!("CARGO_PKG_VERSION"),
            app.frames
        );
        app.time -= 1.0;
        app.frames = 0;
    }

    // Render
    screen::draw(&mut |pixel_buffer| {
        let (_w, h) = screen::wh();
        let mut image = Image::new(Size((screen::pitch() / 4) as u16, h));

        image.clear_ptr(pixel_buffer);
        image.text_ptr(
            [255, 255, 255, 255], // White
            (5.0, 5.0, 36.0),     // Pos. & Size
            &app.font,            // Font
            &app.info,            // String
            pixel_buffer,
        );
    });
}
