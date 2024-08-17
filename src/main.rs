use clap::{Parser, Subcommand};

mod utils;
mod ffmpeg;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Apply an effect to a video
    Effect {
        /// Input video file
        #[arg(value_name = "INPUT")]
        input: String,

        /// Output video file
        #[arg(value_name = "OUTPUT")]
        output: String,

        /// Effect to apply
        #[arg(value_enum)]
        effect: ffmpeg::VideoEffect,
    },
    /// Reverse a video
    Reverse {
        /// Input video file
        #[arg(value_name = "INPUT")]
        input: String,

        /// Output video file
        #[arg(value_name = "OUTPUT")]
        output: String,
    },
    /// Split a video into multiple parts
    Split {
        /// Input video file
        #[arg(value_name = "INPUT")]
        input: String,

        /// Output directory
        #[arg(value_name = "OUTPUT_DIR")]
        output_dir: String,

        /// Number of parts to split the video into
        #[arg(value_name = "PARTS")]
        parts: u32,
    },
    Stretch {
        /// Input video file
        #[arg(value_name = "INPUT")]
        input: String,

        /// Output video file
        #[arg(value_name = "OUTPUT")]
        output: String,

        /// Target duration in seconds
        #[arg(value_name = "DURATION")]
        duration: f64,
    },
    /// Cover a watermark with a shape
    CoverWatermark {
        /// Input video file
        // #[arg(value_name = "INPUT")]
        input: String,

        /// Output video file
        // #[arg(value_name = "OUTPUT")]
        output: String,

        /// Width of the shape (in pixels)
        #[arg(short = 'w', long, value_name = "WIDTH")]
        width: u32,

        /// Height of the shape (in pixels)
        #[arg(short = 'u', long, value_name = "HEIGHT")]
        height: u32,

        /// X-coordinate of the top-left corner of the shape
        #[arg(short = 'x', long, value_name = "X")]
        x: i32,

        /// Y-coordinate of the top-left corner of the shape
        #[arg(short = 'y', long, value_name = "Y")]
        y: i32,

        /// Shape of the cover (rectangle or ellipse)
        #[arg(short = 's', long, value_enum)]
        shape: ffmpeg::Shape,

        /// Color of the shape (in hexadecimal, e.g., '#FF0000' for red)
        #[arg(short = 'c', long, default_value = "#000000")]
        color: String,
    },
    /// Cross-fade between two videos
    CrossFade {
        /// First input video file
        #[arg(value_name = "INPUT1")]
        input1: String,

        /// Second input video file
        #[arg(value_name = "INPUT2")]
        input2: String,

        /// Output video file
        #[arg(value_name = "OUTPUT")]
        output: String,

        /// Duration of the cross-fade in seconds
        #[arg(value_name = "DURATION")]
        duration: f32,
    },
    /// Trim a video
    Trim {
        /// Input video file
        #[arg(value_name = "INPUT")]
        input: String,

        /// Output video file
        #[arg(value_name = "OUTPUT")]
        output: String,

        /// Start time (in seconds)
        #[arg(value_name = "START")]
        start: f32,

        /// End time (in seconds)
        #[arg(value_name = "END")]
        end: f32,
    },
    /// Concatenate multiple videos
    Concat {
        /// Input video files
        #[arg(value_name = "INPUTS", num_args = 1.., required = true)]
        inputs: Vec<String>,

        /// Output video file
        #[arg(value_name = "OUTPUT", short, long)]
        output: String,
    },
}

fn main() {

    if let Err(e) = utils::check_ffmpeg() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = utils::check_ffprobe() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    let cli = Cli::parse();

    match &cli.command {
        Commands::Effect { input, output, effect } => {
            if let Err(e) = ffmpeg::apply_effect(input, output, effect) {
                eprintln!("Error applying effect: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Reverse { input, output } => {
            if let Err(e) = ffmpeg::reverse_video(input, output) {
                eprintln!("Error reversing video: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Split { input, output_dir, parts } => {
            if let Err(e) = ffmpeg::split_video(input, output_dir, *parts) {
                eprintln!("Error splitting video: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Stretch { input, output, duration } => {
            if let Err(e) = ffmpeg::stretch_video(input, output, *duration) {
                eprintln!("Error stretching video: {}", e);
                std::process::exit(1);
            }
        }
        Commands::CoverWatermark { input, output, width, height, x, y, shape, color } => {
            if let Err(e) = ffmpeg::cover_watermark(input, output, *width, *height, *x, *y, *shape, color) {
                eprintln!("Error covering watermark: {}", e);
                std::process::exit(1);
            }
        }
        Commands::CrossFade { input1, input2, output, duration } => {
            if let Err(e) = ffmpeg::cross_fade_videos(input1, input2, output, *duration) {
                eprintln!("Error cross-fading videos: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Trim { input, output, start, end } => {
            if let Err(e) = ffmpeg::trim_video(input, output, *start, *end) {
                eprintln!("Error trimming video: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Concat { inputs, output } => {
            if let Err(e) = ffmpeg::concatenate_videos(inputs, output) {
                eprintln!("Error concatenating videos: {}", e);
                std::process::exit(1);
            }
        }
    }
}
