use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Helper to run the CLI and capture output
fn run_cli(args: &[&str]) -> (String, String, i32) {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("rankhaus")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute command");
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let code = output.status.code().unwrap_or(-1);
    
    (stdout, stderr, code)
}

/// Helper to clean up test files
fn cleanup(path: &str) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_init_command() {
    let filename = "test_init.rankset";
    cleanup(filename);
    
    let (stdout, _stderr, code) = run_cli(&[
        "init",
        "test_init",
        "--user",
        "testuser",
        "--display-name",
        "Test User",
    ]);
    
    assert_eq!(code, 0);
    assert!(stdout.contains("✓ Created"));
    assert!(stdout.contains(filename));
    assert!(PathBuf::from(filename).exists());
    
    cleanup(filename);
}

#[test]
fn test_load_command() {
    let filename = "test_load.rankset";
    cleanup(filename);
    
    // First create a list
    run_cli(&[
        "init",
        "test_load",
        "--user",
        "alice",
        "--display-name",
        "Alice",
    ]);
    
    // Then load it
    let (stdout, _stderr, code) = run_cli(&["load", filename]);
    
    assert_eq!(code, 0);
    assert!(stdout.contains("✓ Loaded"));
    assert!(stdout.contains("test_load"));
    
    cleanup(filename);
}

#[test]
fn test_strategies_list() {
    let (stdout, _stderr, code) = run_cli(&["strategies", "list"]);
    
    assert_eq!(code, 0);
    assert!(stdout.contains("Available strategies"));
    assert!(stdout.contains("merge"));
}

#[test]
fn test_no_list_loaded_error() {
    let (stdout, stderr, code) = run_cli(&["items", "list"]);
    
    assert_ne!(code, 0);
    let output = format!("{}{}", stdout, stderr);
    assert!(output.contains("No list loaded"));
}
