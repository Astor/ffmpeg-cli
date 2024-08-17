use std::process::Command;
use std::io::{self};


pub fn check_ffmpeg() -> io::Result<()> {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                io::Error::new(io::ErrorKind::NotFound, 
                    "FFmpeg not found. Please ensure FFmpeg is installed and available in your PATH.")
            } else {
                e
            }
        })?;
    Ok(())
}
    
pub fn check_ffprobe() -> io::Result<()> {
    Command::new("ffprobe")
        .arg("-version")
        .output()
        .map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                io::Error::new(io::ErrorKind::NotFound, 
                    "FFprobe not found. Please ensure FFprobe is installed and available in your PATH.")
            } else {
                e
            }
        })?;
    Ok(())
}
