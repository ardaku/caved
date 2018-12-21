[![Plop Grizzly](https://plopgrizzly.com/images/logo-bar.png)](https://plopgrizzly.com)

# [Codecs for Audio and Video Encoding and Decoding](https://crates.io/crates/caved)
Codecs for Audio and Video, static bindings to libav (ffmpeg).  Some re-implemented in Rust.
Goal is for all Rust.

`libav` is a great tool for doing multimedia encoding and decoding - but wouldn't it be
even better if it was written in Rust, dual licensed under MIT and Boost version 1?
Here it is - well, at least the beginnings of it.

## Features
`caved`'s current features:
* Nothing yet

## Getting started
```rust
extern crate caved;
use caved::*;
```

## [Contributing](http://plopgrizzly.com/contributing/en#contributing)

## Roadmap to 0.1 (Future Features)
* FFI into libav
* Static linking libav (using `cc` crate).

## Roadmap to 1.0 (Future Features)
* Written all in Rust.

## Change Log
### 0.0.0
* Initial release
