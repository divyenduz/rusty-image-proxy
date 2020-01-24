# Introduction

Helper for https://readbot.app

A web service that removes the colors of an image and caps its width to 600px (max width of a kindle device)

Usage:

`https://<service-url>/link=<image-link>`

# Develop

```
cargo build
cargo run
```

# Resources

- https://doc.rust-lang.org/book/title-page.html
- https://actix.rs/docs/getting-started/
- https://github.com/actix/examples
- https://github.com/image-rs/image
- https://github.com/alexcrichton/curl-rust
- https://timryan.org/2018/07/27/cross-compiling-linux-binaries-from-macos.html
- https://libpixel.com/

- https://preview.readbot.app/images/america.jpg (121.90 KB)
- http://localhost:8088/?link=https://preview.readbot.app/images/america.jpg (16.95 KB After capping w=600px + Grayscale)
- https://preview.readbot.app/images/croatia.png (16.72 KB)
- http://localhost:8088/?link=https://preview.readbot.app/images/croatia.png (6.45 KB After w=600px + Grayscale)

# Not Implemented

See https://github.com/divyenduz/rusty-image-proxy/issues/3

| Function        | Get Parameters                            | Type                 | Default                                 | Status                                                                                                |
| --------------- | ----------------------------------------- | -------------------- | --------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| blur            | blur                                      | float (sigma)        | 0.0                                     | Not Implemented                                                                                       |
| brighten        | brighten                                  | int                  | 0                                       | Not Implemented                                                                                       |
| huerotate       | huerotate                                 | int                  | 0                                       | Not Implemented                                                                                       |
| contrast        | contrast                                  | float                | 0.0                                     | Not Implemented                                                                                       |
| crop            | (crop_x, crop_y, crop_width, crop_height) | (int, int, int, int) | (0, 0, original_width, original_height) | Not Implemented, if any parameter is supplied, use defaults for others, else don't crop               |
| filter3x3       | ?                                         |                      | ?                                       | Not Implemented, I have no idea what this is                                                          |
| flip_horizontal | fliph                                     | boolean              | false                                   | Not Implemented                                                                                       |
| flip_vertical   | flipv                                     | boolean              | false                                   | Not Implemented                                                                                       |
| grayscale       | grayscale                                 | boolean              | false                                   | Currently: hardcoded to always grayscale                                                              |
| invert          | invert                                    | boolean              | false                                   | Not Implemented                                                                                       |
| resize          | (width, height, filter?)                  | (int, int, ?)        | (?, ?, ?)                               | Currently: Harcoded to cap the width at 600. Need to figure out the potential API values for filters. |
| rotate90        | rotate90                                  | boolean              | false                                   | Not Implemented                                                                                       |
| rotate180       | rotate180                                 | boolean              | false                                   | Not Implemented                                                                                       |
| rotate270       | rotate270                                 | boolean              | false                                   | Not Implemented                                                                                       |
| unsharpen       | (unsharpen_sigma, unsharpen_threshold)    | (float, int)         | (0.0, 0)                                | Not Implemented                                                                                       |

Legend:

- ? indicates that I don't know anything about that and can't make a decision yet (If you know, please feel free to change it via a PR).
- Everything else is where I know something, just enough to make a first wild guess. By no means it means that I know a lot about that image processing function.
- (x, y) is a tuple representing multiple get parameters or their types or values
