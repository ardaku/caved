# Caved
Caved stands for "Codecs for Audio and Video Encoding and Decoding".  This is an
oxidized re-implementation of libav (ffmpeg) and SDL_Image.  Currently
statically links to libav for most formats.

## Goals
- APIs to encode/decode all audio/video formats with widespread usage.
- Fast
- No Unsafe
- Pure Rust
- High Level and Low Level APIs
- pix crate-based APIs for graphics
- TBD crate-based APIs for audio

## Roadmap 
### 0.1.0
- FFI into libav
- Static linking libav (using `cc` crate).

### 1.0.0 (Future Features)
- Written all in Rust.

## Getting Started
Examples can be found in the [Documentation](https://docs.rs/caved) and it's
worth checking out [Caving](https://github.com/libcala/caving).

## License
The `ogg_opus` crate is distributed under any of

- The terms of the
  [MIT License](https://github.com/libcala/ogg_opus/blob/master/LICENSE-MIT)
- The terms of the
  [Apache License (Version 2.0)](https://github.com/libcala/ogg_opus/blob/master/LICENSE-APACHE)
- The terms of the
  [Zlib License](https://github.com/libcala/ogg_opus/blob/master/LICENSE-ZLIB)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as described above, without any additional terms or conditions.
