# Video CLI 

A simple FFMPEG wrapper for common functions. 

## Requirements
Both need to be acessible to the CLI by installing on the system and setting the PATH or placing ffmpeg and ffprobe within the "/usr/local/bin"

- ffmpeg
- ffprobe

## To cover a watermark:
```sh
cargo run -- cover-watermark videos/5.mp4 output/no_watermark.mp4 200 100
```

## To apply an effect:
```sh
cargo run -- effect input.mp4 output.mp4 sepia
```

## To reverse a video:
```sh
cargo run -- reverse input.mp4 output_reversed.mp4
```

## To split a file into smaller files:
```sh
cargo run -- split videos/1.mp4 output/ 5
```

## To stretch a file duration:
```sh
cargo run -- stretch videos/1.mp4 output/1_stretched.mp4 30
```

## Cross fade two video files:
```sh
cargo run -- cross-fade videos/1.mp4 videos/2.mp4 output/crossfaded.mp4 2
```

## Trim a section of a video file with a start time and end time and output a new trimmed video.
```sh
cargo run -- trim videos/7.mp4 output/trimmed.mp4 2 8
```