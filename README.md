# TinyBMP

A program that converts images into a very compact file format.

This idea arose from the realisation that all Pokemon sprites in the Pokemon Yellow game are comprised of four colours. As such, this program supports images that contain up to four colours and encodes them as two bits per pixel. The colour palate is encoded in the header.

[The full specification can be found here](./image-format-spec.txt).

## Running

TinyBMP is built in Rust. If you have Rust installed, you can run:

```
cargo run -- <OPTIONS>
```

See below for command-line options

### Encoding

To encode an image as a 4-colour bitmap:

```
cargo run -- -e <input-file> <output-file>
```

In theory, input file can be of any format supported by the [`image`](https://crates.io/crates/image) library, as long as it can be converted to an RGB image and it has a maximum of 4 colours. In practice, this has only been tested with PNGs.

### Decoding

To decode a 4-colour bitmap to a standard image format:

```
cargo run -- -e <input-file> <output-file.ext>
```

The program will detect the desired image format based on the file extension. Only `.png` has been tested.

### Examples

Encoding:

```
cargo run -- -e pikachu.png pikachu.4bm
```

Decoding:

```
cargo run -- -d pikachu.4bm pikachu.png
```

## Efficiency

Empirical results have demonstrated that TinyBMP can outperform PNGs in file size.

Source image:

```
$ wc -c pikachu.png
767 pikachu.png
```

Encoded image:

```
$ wc -c pikachu.4bm
416 pikachu.4bm
```

In this example, the 4-colour bitmap is 54% the size of the original PNG.

Source image details:

```
$ identify -verbose pikachu.png
Image:
  Filename: pikachu.png
  Format: PNG (Portable Network Graphics)
  Mime type: image/png
  Class: DirectClass
  Geometry: 40x40+0+0
  Units: Undefined
  Colorspace: sRGB
  Type: PaletteAlpha
  Base type: Undefined
  Endianness: Undefined
  Depth: 8-bit
  Channel depth:
    red: 8-bit
    green: 8-bit
    blue: 8-bit
    alpha: 1-bit
  Channel statistics:
    Pixels: 1600
    Red:
      min: 23  (0.0901961)
      max: 247 (0.968627)
      mean: 223.445 (0.876255)
      standard deviation: 65.7661 (0.257906)
      kurtosis: 5.31913
      skewness: -2.68738
      entropy: 0.540953
    Green:
      min: 23  (0.0901961)
      max: 247 (0.968627)
      mean: 214.135 (0.839745)
      standard deviation: 72.7332 (0.285228)
      kurtosis: 1.96047
      skewness: -1.90801
      entropy: 0.540953
    Blue:
      min: 0  (0)
      max: 247 (0.968627)
      mean: 192.713 (0.755735)
      standard deviation: 99.8611 (0.391612)
      kurtosis: -0.292038
      skewness: -1.3008
      entropy: 0.631191
    Alpha:
      min: 255  (1)
      max: 255 (1)
      mean: 255 (1)
      standard deviation: 0 (0)
      kurtosis: -1.6384e+52
      skewness: 1e+36
      entropy: 0
  Image statistics:
    Overall:
      min: 0  (0)
      max: 255 (1)
      mean: 221.323 (0.867934)
      standard deviation: 59.5901 (0.233687)
      kurtosis: 3.68872
      skewness: -2.34658
      entropy: 0.428274
  Colors: 4
  Histogram:
    154: (23,23,23,255) #171717FF grey9
    133: (223,111,0,255) #DF6F00FF srgba(223,111,0,1)
    79: (247,247,0,255) #F7F700FF srgba(247,247,0,1)
    1234: (247,247,247,255) #F7F7F7FF grey97
  Rendering intent: Perceptual
  Gamma: 0.454545
  Chromaticity:
    red primary: (0.64,0.33)
    green primary: (0.3,0.6)
    blue primary: (0.15,0.06)
    white point: (0.3127,0.329)
  Background color: white
  Border color: srgba(223,223,223,1)
  Matte color: grey74
  Transparent color: none
  Interlace: None
  Intensity: Undefined
  Compose: Over
  Page geometry: 40x40+0+0
  Dispose: Undefined
  Iterations: 0
  Compression: Zip
  Orientation: Undefined
  Properties:
    date:create: 2023-05-19T13:23:49+00:00
    date:modify: 2023-05-19T13:18:36+00:00
    png:IHDR.bit-depth-orig: 8
    png:IHDR.bit_depth: 8
    png:IHDR.color-type-orig: 6
    png:IHDR.color_type: 6 (RGBA)
    png:IHDR.interlace_method: 0 (Not interlaced)
    png:IHDR.width,height: 40, 40
    png:sRGB: intent=0 (Perceptual Intent)
    signature: d28dcd7b719ecc2b1a40064f01331fa4f999b01ee2bb3e470b24d8936842a774
  Artifacts:
    filename: pikachu.png
    verbose: true
  Tainted: False
  Filesize: 767B
  Number pixels: 1600
  Pixels per second: 4.40523MB
  User time: 0.000u
  Elapsed time: 0:01.000
  Version: ImageMagick 6.9.11-60 Q16 x86_64 2021-01-25 https://imagemagick.org
```
