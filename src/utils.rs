use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use walkdir::DirEntry;

/// Returns true if the given entry is file
///
/// # Arguments
///
/// * `entry` - Entry
pub fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}

/// Returns true if the given entry is directory
///
/// # Arguments
///
/// * `entry` - Entry
pub fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

/// Returns true if the given entry is hidden file or directory
///
/// # Arguments
///
/// * `entry` - Entry
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().starts_with('.')
}

/// Get progress bar
///
/// # Arguments
///
/// * `length` - Length for progress bar
pub fn get_progress_bar(length: u64) -> ProgressBar {
    let pb = ProgressBar::new(length);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:30.cyan/blue}] {pos/len} ({eta}) {msg}",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    pb
}
