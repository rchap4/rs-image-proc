# Rs-Image-Proc
Simple CLI imaging processing in rust using rs-photon.  



* Brighten an image
* Darken an image
* Enhance Contrast
* Scale image by a factor

Clone and then...
```
cargo build

cargo run -- --help

# or
cargo build --release
```

# CLI Help
```
Photon Image Tool 0.1.0
Simple image processing

USAGE:
    rs-image-proc [FLAGS] [OPTIONS] <input-image> <output-image>

FLAGS:
        --correct    Image correction
    -h, --help       Prints help information
        --resize     Resize image
    -V, --version    Prints version information

OPTIONS:
    -b, --brighten <brightness>    Brightness value
    -c, --contrast <contrast>      Contrast value
    -d, --darken <darken>          Darken value
    -s, --scale <scale>            Percent to scale image

ARGS:
    <input-image>     Input image
    <output-image>    Output image

```