// This example program may be used for any purpose proprietary or not.

#[macro_use]
extern crate adi;
extern crate barg;
extern crate libc;
extern crate miniz_oxide;

use std::env;
use std::ffi::CString;
// use std::fs::File;

use adi::{
    hid,
    screen::{
        self,
        prelude::{*},
        Viewer,
    },
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
        #[allow(unused)]
        viewer: Viewer,
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
            println!(
                "CAVING > video_spf: {}/{}",
                GLOBAL.video_spf.num, GLOBAL.video_spf.den
            );
        }

        let texture =
            screen::texture(unsafe { (GLOBAL.video_width as u16, GLOBAL.video_height as u16) });
        let tc = screen::texcoords(&[
            (0.0, 0.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
            //	        (0.0, 0.0),
            //	        (0.0, 1.0),
            //	        (1.0, 1.0),
            //	        (1.0, 0.0),
        ]);

        let model = ModelBuilder::new()
            .vert(&[
                barg::Move(-1.0, -1.0),
                barg::Line(-1.0, 1.0),
                barg::Line(1.0, 1.0),
                barg::Line(1.0, -1.0),
            ])
            .face(matrix!().t(vector!(0.0, 0.0, 1.0)))
            .close();

        let viewer = Viewer::new(vector!(), vector!());
        let _a = viewer.add_textured(&model, matrix!(), &texture, tc, false);

        //        ::std::mem::forget(model);
        //        ::std::mem::forget(tc);
        //        ::std::mem::forget(viewer);

        Ctx {
            time: 0.0,
            frames: 0,
            font: FontChain::default(),
            info: "".to_string(),
            mode: mode_play,
            texture,
            viewer,
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
        video_spf: *mut AVRational,
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
struct AVRational {
    den: i32, // Numerator
    num: i32, // Denominator
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

struct Global {
    video_spf: AVRational,
    set: bool,
    //    image: Option<VFrame>,
    video_width: c_int,
    video_height: c_int,
    pixel_format: AVPixelFormat,
    audio_channels: c_int,
    audio_samplerate: c_int,
    audio_sampleformat: AVSampleFormat,
    pixels: *mut u8,
    pitch: usize,
}

static mut GLOBAL: Global = Global {
    video_spf: AVRational { num: 0, den: 0 },
    set: true,
    //    image: None,
    video_width: 0,
    video_height: 0,
    pixel_format: AVPixelFormat::None,
    audio_channels: 0,
    audio_samplerate: 0,
    audio_sampleformat: AVSampleFormat::None,
    pixels: std::ptr::null_mut(),
    pitch: 0,
    //    video: None,
    //    audio: None,
};

unsafe extern "C" fn video_write(
    data_y: *mut c_void,
    data_cb: *mut c_void,
    data_cr: *mut c_void,
    pitch_y: size_t,
    pitch_cb: size_t,
    pitch_cr: size_t,
) {
    GLOBAL.set = false;

    let mut pixels = GLOBAL.pixels; //vframe.0.as_mut_ptr();
    let pitch = GLOBAL.pitch as isize;
    let twice_pitch = pitch << 1;
    let half_height = (GLOBAL.video_height >> 1) as isize;
    let half_width = (GLOBAL.video_width >> 1) as isize;
    let pitch_y = pitch_y as isize;
    let pitch_cb = pitch_cb as isize;
    let pitch_cr = pitch_cr as isize;

    for i in 0..half_height {
        let data_y = data_y.offset((i << 1) * pitch_y) as *mut _ as *mut u8;
        let data_cb = data_cb.offset(i * pitch_cb) as *mut _ as *mut u8;
        let data_cr = data_cr.offset(i * pitch_cr) as *mut _ as *mut u8;
        let mut pixels_b = pixels;

        for j in 0..half_width {
            let twice_j0 = j << 1;
            let twice_j1 = twice_j0 + 1;
            let y0 = (*data_y.offset(twice_j0) as i32 - 16) * 298;
            let y1 = (*data_y.offset(twice_j1) as i32 - 16) * 298;
            let y2 = (*data_y.offset(twice_j0 + pitch_y) as i32 - 16) * 298;
            let y3 = (*data_y.offset(twice_j1 + pitch_y) as i32 - 16) * 298;
            let cb = *data_cb.offset(j) as i32 - 128;
            let cr = *data_cr.offset(j) as i32 - 128;

            let r = (409 * cr) + 128;
            let g = (-100 * cb) + (-208 * cr) + 128;
            let b = (516 * cb) + 128;

            // Up Left
            *pixels_b.offset(0) = ((y0 + r) >> 8).min(255).max(0) as u8;
            *pixels_b.offset(1) = ((y0 + g) >> 8).min(255).max(0) as u8;
            *pixels_b.offset(2) = ((y0 + b) >> 8).min(255).max(0) as u8;

            // Up Right
            *pixels_b.offset(4) = ((y1 + r) >> 8).min(255).max(0) as u8;
            *pixels_b.offset(5) = ((y1 + g) >> 8).min(255).max(0) as u8;
            *pixels_b.offset(6) = ((y1 + b) >> 8).min(255).max(0) as u8;

            let pixels2 = pixels_b.offset(pitch);

            // Down Left
            *pixels2.offset(0) = ((y2 + r) >> 8).min(255).max(0) as u8;
            *pixels2.offset(1) = ((y2 + g) >> 8).min(255).max(0) as u8;
            *pixels2.offset(2) = ((y2 + b) >> 8).min(255).max(0) as u8;

            // Down Right
            *pixels2.offset(4) = ((y3 + r) >> 8).min(255).max(0) as u8;
            *pixels2.offset(5) = ((y3 + g) >> 8).min(255).max(0) as u8;
            *pixels2.offset(6) = ((y3 + b) >> 8).min(255).max(0) as u8;

            pixels_b = pixels_b.offset(8);
        }

        pixels = pixels.offset(twice_pitch);
    }
}

unsafe extern "C" fn audio_write(_data: *mut c_void, _size: size_t) {
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
            &mut GLOBAL.video_spf,
        );

        //        GLOBAL.image = Some(VFrame(vec![255; GLOBAL.video_width as usize * GLOBAL.video_height as usize * 4]));

        //        caving_decode_video(c_filename.as_ptr());
        //        caving_decode_audio(c_filename.as_ptr());
    }

    Ok(()) /*unsafe {
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

fn mode_play(app: &mut Ctx) {
    // Check for exit request
    if hid::Key::Back.pressed(0) {
        adi::old();
    }

    // Begin timing
    app.time += screen::dt();
    app.frames += 1;

    if app.time >= 1.0 {
        app.info = format!(
            "Caving {} - FPS: {}\nDecoding-Playing…",
            env!("CARGO_PKG_VERSION"),
            app.frames,
        );
        app.time -= 1.0;
        app.frames = 0;
    }

    let mut stopping = false;
    screen::texture_set(&mut app.texture, &mut |pixels, pitch| {
        unsafe {
            GLOBAL.set = true;
            GLOBAL.pixels = pixels.as_mut_ptr();
            GLOBAL.pitch = pitch;
            while GLOBAL.set {
                if caving_decode_run(video_write, audio_write) {
                    stopping = true;
                }
            }
        }

        //        let image = unsafe { GLOBAL.image.as_ref().unwrap() };

        //        unsafe {
        //            std::ptr::copy(image.0.as_ptr(), pixels.as_mut_ptr(), image.0.len());
        //        }
    }); /* unsafe {
            (GLOBAL.video_width as u16, GLOBAL.video_height as u16)
        }, unsafe { GLOBAL.image.as_ref().unwrap() });*/

    if stopping {
        app.mode = mode_stop;
    }

    // Render
    screen::draw(&mut |pixels, pitch| {
        let (_w, h) = screen::wh();
        let mut image = Image::new(Size((pitch / 4) as u16, h));

        image.clear_ptr(pixels.as_mut_ptr());

        image.text_ptr(
            [255, 255, 255, 255], // White
            (5.0, 5.0, 36.0),     // Pos. & Size
            &app.font,            // Font
            &app.info,            // String
            pixels.as_mut_ptr(),
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
    screen::draw(&mut |pixels, pitch| {
        let (_w, h) = screen::wh();
        let mut image = Image::new(Size((pitch / 4) as u16, h));

        image.clear_ptr(pixels.as_mut_ptr());
        image.text_ptr(
            [255, 255, 255, 255], // White
            (5.0, 5.0, 36.0),     // Pos. & Size
            &app.font,            // Font
            &app.info,            // String
            pixels.as_mut_ptr(),
        );
    });
}
