/** Shell utilities */
use std::process::Command;

/** Execute a shell command using the system shell. */
pub fn execute_shell_command(cmd: &str) -> Result<i32, String> {
    #[cfg(target_os = "windows")]
    let output = Command::new("cmd").args(["/C", cmd]).status();

    #[cfg(not(target_os = "windows"))]
    let output = Command::new("sh").arg("-c").arg(cmd).status();

    match output {
        Ok(status) => Ok(status.code().unwrap_or_default()),
        Err(e) => Err(format!("Failed to execute command: {}", e)),
    }
}
