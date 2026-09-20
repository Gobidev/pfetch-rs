use std::io::Write;
use std::process::{Command, Stdio};

fn pfetch() -> Command {
    let bin = std::env::var("CARGO_BIN_EXE_pfetch")
        .unwrap_or_else(|_| String::from("target/debug/pfetch"));
    let mut cmd = Command::new(bin);
    cmd.env_remove("PF_INFO")
        .env_remove("PF_ASCII")
        .env_remove("PF_COLOR")
        .env_remove("PF_SEP")
        .env_remove("PF_SOURCE")
        .env_remove("PF_CUSTOM_LOGOS")
        .env_remove("PF_FAST_PKG_COUNT")
        .env_remove("PF_COL1")
        .env_remove("PF_COL2")
        .env_remove("PF_COL3")
        .env_remove("PF_PAD1")
        .env_remove("PF_PAD2")
        .env_remove("PF_PAD3");
    cmd
}

#[test]
fn version_flag() {
    let output = pfetch().arg("-v").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("pfetch-rs "));
}

#[test]
fn help_flag() {
    let output = pfetch().arg("--help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--file-raw"));
    assert!(stdout.contains("--logo"));
    assert!(stdout.contains("--no-color"));
}

#[test]
fn info_filter() {
    let output = pfetch().args(["--info", "title"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains('@'));
    assert!(!stdout.contains("os "));
}

#[test]
fn separator_override() {
    let output = pfetch()
        .args(["--sep", "~", "--info", "title os"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("os~"));
}

#[test]
fn logo_override() {
    let output = pfetch()
        .args(["--logo", "openbsd", "--info", "title"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains('@'));
}

#[test]
fn raw_logo_from_stdin() {
    let mut child = pfetch()
        .args(["--file-raw", "-", "--info", "title"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(b"RAWLOGO\n").unwrap();
    drop(stdin);

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("RAWLOGO"));
    assert!(stdout.contains('@'));
}

#[test]
fn raw_logo_missing_file_errors() {
    let output = pfetch()
        .args(["--file-raw", "/nonexistent/file.txt"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Could not read logo file"));
}

#[test]
fn piped_output_has_no_ansi_codes() {
    let output = pfetch().output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains('\x1b'));
}

#[test]
fn palette_no_color_in_pipe() {
    let output = pfetch().args(["--info", "palette"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains('\x1b'));
}

#[test]
fn color_flag_forces_ansi() {
    let output = pfetch()
        .args(["--color", "--info", "palette"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains('\x1b'));
}

#[test]
fn version_short_circuits_before_file_raw_error() {
    let output = pfetch()
        .args(["-v", "--file-raw", "/nonexistent"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("pfetch-rs "));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("Could not read logo file"));
}

#[test]
fn help_short_circuits_before_file_raw_error() {
    let output = pfetch()
        .args(["--help", "--file-raw", "/nonexistent"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage: pfetch"));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("Could not read logo file"));
}

#[test]
fn unknown_option_errors() {
    let output = pfetch().arg("--file-raaa").output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown option '--file-raaa'"));
}

#[test]
fn missing_custom_logos_warns_and_continues() {
    let output = pfetch()
        .env("PF_CUSTOM_LOGOS", "/nonexistent/custom_logos")
        .arg("--info")
        .arg("title")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Could not read custom logo file"));
}
