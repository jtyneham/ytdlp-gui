//! Explicit, user-initiated dependency maintenance.
//! No executable is downloaded or replaced on startup.
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn run(program: &str, args: &[&str]) -> Result<Output, String> {
    let mut command = Command::new(program);
    command.args(args).stdin(Stdio::null());
    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);
    command.output().map_err(|e| format!("{program}: {e}"))
}

fn output_text(output: Output) -> Result<String, String> {
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if output.status.success() {
        Ok(if stdout.is_empty() { stderr } else { stdout })
    } else {
        Err(if stderr.is_empty() { stdout } else { stderr })
    }
}

pub async fn check(yt_dlp_path: Option<PathBuf>) -> String {
    let binary = yt_dlp_path.unwrap_or_else(|| "yt-dlp".into());
    let yt = run(&binary.to_string_lossy(), &["--version"])
        .and_then(output_text)
        .unwrap_or_else(|e| format!("unavailable ({e})"));
    let ffmpeg = run("ffmpeg", &["-version"])
        .and_then(output_text)
        .map(|s| s.lines().next().unwrap_or("unknown").to_string())
        .unwrap_or_else(|e| format!("unavailable ({e})"));
    format!("Installed yt-dlp: {yt}\nInstalled FFmpeg: {ffmpeg}")
}

pub async fn update_ytdlp(yt_dlp_path: Option<PathBuf>) -> String {
    let binary = yt_dlp_path.unwrap_or_else(|| "yt-dlp".into());
    let result = run(&binary.to_string_lossy(), &["-U"]).and_then(output_text);
    match result {
        Ok(message) => format!("yt-dlp: {message}"),
        Err(e) => format!("yt-dlp update failed: {e}. If installed with pip or a package manager, update it using that manager."),
    }
}

pub async fn update_ffmpeg() -> String {
    #[cfg(target_os = "windows")]
    {
        // winget upgrades only an existing, winget-recognized package.
        // Never download an unverified archive or overwrite an arbitrary FFmpeg path.
        match run("winget", &["upgrade", "--id", "Gyan.FFmpeg", "--exact", "--accept-source-agreements", "--accept-package-agreements"])
            .and_then(output_text)
        {
            Ok(message) => format!("FFmpeg: {message}"),
            Err(e) => format!("FFmpeg update unavailable: {e}. This action only supports winget-managed Gyan.FFmpeg."),
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        "FFmpeg updater is available on Windows only.".into()
    }
}
