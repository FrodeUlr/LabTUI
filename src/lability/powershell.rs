#![allow(dead_code)]
use anyhow::{Context, Result};
use std::process::Command;

/// Result of running a PowerShell command
pub struct PsResult {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

/// Run a PowerShell command string and return the combined output.
///
/// The process runs in `-NonInteractive` mode, which is safe for commands
/// that never prompt the user (module checks, VM stop/remove, etc.).
pub fn run_powershell(command: &str) -> Result<PsResult> {
    run_ps_inner(command, /*interactive=*/ false)
}

/// Run a PowerShell command that requires interactive prompts (e.g. `Get-Credential`).
///
/// Identical to [`run_powershell`] but omits `-NonInteractive` so that
/// PowerShell can read from the terminal.  Use this for commands like
/// `Start-LabConfiguration` where Lability calls `Get-Credential` internally.
pub fn run_powershell_interactive(command: &str) -> Result<PsResult> {
    run_ps_inner(command, /*interactive=*/ true)
}

fn run_ps_inner(command: &str, interactive: bool) -> Result<PsResult> {
    let ps_exe = powershell_exe();
    let mut args: Vec<&str> = Vec::new();
    if !interactive {
        args.push("-NonInteractive");
    }
    args.extend(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", command]);

    let output = Command::new(ps_exe)
        .args(&args)
        .output()
        .with_context(|| format!("Failed to launch {ps_exe}"))?;

    Ok(PsResult {
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        success: output.status.success(),
    })
}

/// Build the PowerShell command to start a Lability lab.
///
/// `env_name`   – the lab / environment name (used to derive file names).
/// `output_dir` – directory where the `.psd1` and `.ps1` files were written.
pub fn start_lab_command(env_name: &str, output_dir: &str) -> String {
    format!(
        "$ErrorActionPreference = 'Stop'; \
         Import-Module Lability; \
         Set-Location '{output_dir}'; \
         & '.\\{env_name}.ps1'; \
         Start-LabConfiguration -ConfigurationData '.\\{env_name}.psd1' -Path '.\\{env_name}' -Verbose"
    )
}

/// Build the PowerShell command to stop all VMs in a lab.
pub fn stop_lab_command(environment_name: &str) -> String {
    format!(
        "Get-VM | Where-Object {{ $_.Name -like '{environment_name}*' }} | Stop-VM -Force"
    )
}

/// Build the PowerShell command to remove all VMs in a lab.
pub fn remove_lab_command(environment_name: &str) -> String {
    format!(
        "Import-Module Lability; Remove-LabConfiguration -EnvironmentPath '{environment_name}' -Verbose"
    )
}

/// Build the PowerShell command to install required DSC modules.
pub fn install_modules_command(module_names: &[String]) -> String {
    let list = module_names
        .iter()
        .map(|m| format!("'{m}'"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("@({list}) | ForEach-Object {{ Install-Module -Name $_ -Force -AllowClobber }}")
}

/// Build the PowerShell command to check Lability installation status.
pub fn check_lability_command() -> &'static str {
    "Get-Module -ListAvailable -Name Lability | Select-Object -ExpandProperty Version"
}

/// Build the PowerShell command to get available Hyper-V switches.
pub fn get_switches_command() -> &'static str {
    "Get-VMSwitch | Select-Object -ExpandProperty Name"
}

/// Always use `pwsh` (PowerShell 7+).
/// Lability requires at minimum PowerShell 6.1, so `powershell.exe`
/// (Windows PowerShell 5.x) is never an acceptable choice.
fn powershell_exe() -> &'static str {
    "pwsh"
}
