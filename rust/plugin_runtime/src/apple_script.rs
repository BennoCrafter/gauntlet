use deno_core::op2;
use anyhow::anyhow;

#[op2(async)]
#[string]
pub async fn apple_script_run(
    #[string] script: String,
) -> anyhow::Result<String> {
    use std::process::Command;
    use std::io::Write;
    let mut command = Command::new("osascript");
    let mut child = command
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(script.as_bytes())?;
    } else {
        return Err(anyhow!("Failed to open stdin"));
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(anyhow!(String::from_utf8_lossy(&output.stderr).to_string()))
    }
}
