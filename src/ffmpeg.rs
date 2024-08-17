use clap::ValueEnum;
use std::process::{Command, Stdio};
use std::path::Path;
use std::io::{self, Read, Write};
use std::fs::File;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Shape {
    Rectangle,
    Ellipse,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum VideoEffect {
    Sepia,
    Blur,
    Vflip,
}

pub fn cover_watermark(input: &str, output: &str, width: u32, height: u32, x: i32, y: i32, shape: Shape, color: &str) -> io::Result<()> {
    // Check if input file exists
    if !Path::new(input).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Input file not found: {}", input)));
    }

    // Parse the color string to extract RGB values
    // TODO Finish Ellipse as it is not working, 
    // only Rectangle is working at the moment.
    let _rgb = hex_to_rgb(color)?;

    // Construct the FFmpeg filter based on the shape
    let filter = match shape {
        Shape::Rectangle => format!(
            "drawbox=x={}:y={}:w={}:h={}:color={}@1:t=fill", x, y, width, height, color
        ),
        Shape::Ellipse => format!(
            "format=rgba,drawbox=x={}:y={}:w={}:h={}:color={}@1:t=fill,geq=lum='lum(X,Y)':a='if(pow((X-({x}+{w}/2))/(0.5*{w}),2)+pow((Y-({y}+{h}/2))/(0.5*{h}),2)<=1,255,0)'",
            x, y, width, height, color, x=x, y=y, w=width, h=height
        ),
    };

    // Construct the FFmpeg command
    let mut command = Command::new("ffmpeg");
    command.arg("-i")
           .arg(input)
           .arg("-vf")
           .arg(&filter)
           .arg("-c:a")
           .arg("copy")
           .arg("-y") // Overwrite output file if it exists
           .arg(output);

    // Print the command being run
    println!("Running FFmpeg command: {:?}", command);

    // Run the command with piped stdout and stderr
    let mut child = command.stdout(Stdio::piped())
                           .stderr(Stdio::piped())
                           .spawn()?;

    // Read stdout and stderr
    let mut stdout = String::new();
    let mut stderr = String::new();
    if let Some(ref mut stdout_handle) = child.stdout {
        stdout_handle.read_to_string(&mut stdout)?;
    }
    if let Some(ref mut stderr_handle) = child.stderr {
        stderr_handle.read_to_string(&mut stderr)?;
    }

    // Wait for the command to finish and check the status
    let status = child.wait()?;

    // Print stdout and stderr
    println!("FFmpeg stdout:\n{}", stdout);
    println!("FFmpeg stderr:\n{}", stderr);

    if status.success() {
        println!("Watermark covered successfully!");
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("FFmpeg command failed with status: {}", status)))
    }
}

pub fn hex_to_rgb(hex: &str) -> io::Result<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid hex color code"));
    }
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    Ok((r, g, b))
}

pub fn cross_fade_videos(input1: &str, input2: &str, output: &str, duration: f32) -> io::Result<()> {
    // Check if input files exist
    if !Path::new(input1).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("First input file not found: {}", input1)));
    }
    if !Path::new(input2).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Second input file not found: {}", input2)));
    }

    // Get the duration of the first video
    let video1_duration = get_video_duration(input1)?;

    // Calculate the start time for the cross-fade
    let fade_start = video1_duration - duration as f64;

    // Construct the FFmpeg command
    let filter_complex = format!("[0:v][1:v]xfade=transition=fade:duration={}:offset={}[v]", duration, fade_start);

    let mut command = Command::new("ffmpeg");
    command.arg("-i")
           .arg(input1)
           .arg("-i")
           .arg(input2)
           .arg("-filter_complex")
           .arg(&filter_complex)
           .arg("-map")
           .arg("[v]")
           .arg("-y") // Overwrite output file if it exists
           .arg(output);

    // Print the command
    println!("Running FFmpeg command: {:?}", command);

    // Run the command with piped stdout and stderr
    let mut child = command.stdout(Stdio::piped())
                           .stderr(Stdio::piped())
                           .spawn()?;

    // Read stdout and stderr
    let mut stdout = String::new();
    let mut stderr = String::new();
    if let Some(ref mut stdout_handle) = child.stdout {
        stdout_handle.read_to_string(&mut stdout)?;
    }
    if let Some(ref mut stderr_handle) = child.stderr {
        stderr_handle.read_to_string(&mut stderr)?;
    }

    // Wait for the command to finish and check the status
    let status = child.wait()?;

    // Print stdout and stderr
    println!("FFmpeg stdout:\n{}", stdout);
    println!("FFmpeg stderr:\n{}", stderr);

    if status.success() {
        println!("Videos cross-faded successfully!");
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("FFmpeg command failed with status: {}", status)))
    }
}

pub fn apply_effect(input: &str, output: &str, effect: &VideoEffect) -> io::Result<()> {
    let filter = match effect {
        VideoEffect::Sepia => "colorchannelmixer=.393:.769:.189:0:.349:.686:.168:0:.272:.534:.131",
        VideoEffect::Blur => "boxblur=5:1",
        VideoEffect::Vflip => "vflip",
    };

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-vf")
        .arg(filter)
        .arg(output)
        .status()
        .expect("Failed to execute FFmpeg command");

    if status.success() {
        println!("Video effect applied successfully!");
    } else {
        eprintln!("Error applying video effect");
    }

    Ok(())
}

pub fn reverse_video(input: &str, output: &str) -> std::io::Result<()> {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-vf")
        .arg("reverse")
        .arg("-af")
        .arg("areverse")
        .arg(output)
        .status()
        .expect("Failed to execute FFmpeg command");

    if status.success() {
        println!("Video reversed successfully!");
    } else {
        eprintln!("Error reversing video");
    }

    Ok(())
}

pub fn split_video(input: &str, output_dir: &str, parts: u32) -> io::Result<()> {
    let output_path = Path::new(output_dir);
    if !output_path.exists() {
        std::fs::create_dir_all(output_path)?;
    }

    let output_pattern = output_path.join("part%03d.mp4").to_str().unwrap().to_string();

    // Get the duration of the input video
    let duration = get_video_duration(input)?;

    // Calculate segment duration
    let segment_duration = duration / parts as f64;

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-f")
        .arg("segment")
        .arg("-segment_time")
        .arg(segment_duration.to_string())
        .arg("-reset_timestamps")
        .arg("1")
        .arg("-c")
        .arg("copy")
        .arg(&output_pattern)
        .status()?;

    if status.success() {
        println!("Video split successfully into {} parts!", parts);
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "FFmpeg command failed"))
    }
}

pub fn stretch_video(input: &str, output: &str, target_duration: f64) -> io::Result<()> {
    // Check if input file exists
    if !Path::new(input).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Input file not found: {}", input)));
    }

    // Get the duration of the input video
    let original_duration = get_video_duration(input)?;

    // Calculate the stretch factor
    let stretch_factor = target_duration / original_duration;

    // Construct the FFmpeg command
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-filter:v")
        .arg(format!("setpts={}*PTS", stretch_factor))
        .arg("-filter:a")
        .arg(format!("atempo={}", 1.0 / stretch_factor))
        .arg("-y") // Overwrite output file if it exists
        .arg(output)
        .output()?;

    if output.status.success() {
        println!("Video stretched successfully from {:.2} seconds to {:.2} seconds!", original_duration, target_duration);
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, format!("FFmpeg command failed: {}", error_message)))
    }
}

pub fn trim_video(input: &str, output: &str, start: f32, end: f32) -> io::Result<()> {
    // Check if input file exists
    if !Path::new(input).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Input file not found: {}", input)));
    }

    // Get video information
    let (has_video, has_audio) = get_stream_info(input)?;

    if !has_video {
        return Err(io::Error::new(io::ErrorKind::Other, "Input file has no video stream"));
    }

    // Construct the FFmpeg command
    let mut command = Command::new("ffmpeg");
    command.arg("-i")
           .arg(input)
           .arg("-ss")
           .arg(start.to_string())
           .arg("-to")
           .arg(end.to_string());

    // Add video encoding options
    command.arg("-map").arg("0:v");
    
    if has_audio {
        command.arg("-map").arg("0:a");
    }

    // Force encoding to ensure output is not empty
    command.arg("-c:v").arg("libx264")
           .arg("-preset").arg("fast");

    if has_audio {
        command.arg("-c:a").arg("aac");
    }

    command.arg("-y") // Overwrite output file if it exists
           .arg(output);

    // Print the command being run
    println!("Running FFmpeg command: {:?}", command);

    // Run the command with piped stdout and stderr
    let mut child = command.stdout(Stdio::piped())
                           .stderr(Stdio::piped())
                           .spawn()?;

    // Read stdout and stderr
    let mut stdout = String::new();
    let mut stderr = String::new();
    if let Some(ref mut stdout_handle) = child.stdout {
        stdout_handle.read_to_string(&mut stdout)?;
    }
    if let Some(ref mut stderr_handle) = child.stderr {
        stderr_handle.read_to_string(&mut stderr)?;
    }

    // Wait for the command to finish and check the status
    let status = child.wait()?;

    // Print stdout and stderr
    println!("FFmpeg stdout:\n{}", stdout);
    println!("FFmpeg stderr:\n{}", stderr);

    if status.success() {
        println!("Video trimmed successfully!");
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("FFmpeg command failed with status: {}", status)))
    }
}

pub fn get_stream_info(input: &str) -> io::Result<(bool, bool)> {
    let output = Command::new("ffprobe")
        .args(&["-v", "error", "-show_entries", "stream=codec_type", "-of", "csv=p=0", input])
        .output()?;

    if output.status.success() {
        let streams = String::from_utf8_lossy(&output.stdout);
        let has_video = streams.lines().any(|line| line == "video");
        let has_audio = streams.lines().any(|line| line == "audio");
        Ok((has_video, has_audio))
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, format!("FFprobe command failed: {}", error_message)))
    }
}

pub fn get_video_duration(input: &str) -> io::Result<f64> {
    let output = Command::new("ffprobe")
        .args(&["-v", "error", "-show_entries", "format=duration", "-of", "default=noprint_wrappers=1:nokey=1", input])
        .output()?;

    if output.status.success() {
        let duration_str = String::from_utf8_lossy(&output.stdout);
        duration_str.trim().parse::<f64>().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "FFprobe command failed"))
    }
}

#[allow(dead_code)]
pub fn get_video_dimensions(input: &str) -> io::Result<(u32, u32)> {
    let output = Command::new("ffprobe")
        .args(&["-v", "error", "-select_streams", "v:0", "-count_packets", "-show_entries", "stream=width,height", "-of", "csv=p=0", input])
        .output()?;

    if output.status.success() {
        let dimensions = String::from_utf8_lossy(&output.stdout);
        let mut parts = dimensions.trim().split(',');
        let width = parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
        let height = parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
        Ok((width, height))
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, format!("FFprobe command failed: {}", error_message)))
    }
}

pub fn concatenate_videos(inputs: &[String], output: &str) -> io::Result<()> {
    // Check if input files exist
    for input in inputs {
        if !Path::new(input).exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("Input file not found: {}", input)));
        }
    }

    // Create a temporary file list
    let temp_file = "temp_file_list.txt";
    {
        let mut file = File::create(temp_file)?;
        for input in inputs {
            writeln!(file, "file '{}'", input)?;
        }
    }

    // Construct the FFmpeg command
    let mut command = Command::new("ffmpeg");
    command.arg("-f")
           .arg("concat")
           .arg("-safe")
           .arg("0")
           .arg("-i")
           .arg(temp_file)
           .arg("-c")
           .arg("copy")
           .arg("-y") // Overwrite output file if it exists
           .arg(output);

    // Print the command being run
    println!("Running FFmpeg command: {:?}", command);

    // Run the command with piped stdout and stderr
    let mut child = command.stdout(Stdio::piped())
                           .stderr(Stdio::piped())
                           .spawn()?;

    // Read stdout and stderr
    let mut stdout = String::new();
    let mut stderr = String::new();
    if let Some(ref mut stdout_handle) = child.stdout {
        stdout_handle.read_to_string(&mut stdout)?;
    }
    if let Some(ref mut stderr_handle) = child.stderr {
        stderr_handle.read_to_string(&mut stderr)?;
    }

    // Wait for the command to finish and check the status
    let status = child.wait()?;

    // Print stdout and stderr
    println!("FFmpeg stdout:\n{}", stdout);
    println!("FFmpeg stderr:\n{}", stderr);

    // Remove the temporary file
    std::fs::remove_file(temp_file)?;

    if status.success() {
        println!("Videos concatenated successfully!");
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("FFmpeg command failed with status: {}", status)))
    }
}