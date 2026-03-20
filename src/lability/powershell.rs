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
/// On non-Windows platforms this function returns a descriptive error so
/// the application can surface it in the UI rather than crashing.
pub fn run_powershell(command: &str) -> Result<PsResult> {
    let ps_exe = powershell_exe();
    let output = Command::new(ps_exe)
        .args([
            "-NonInteractive",
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            command,
        ])
        .output()
        .with_context(|| format!("Failed to launch {ps_exe}"))?;

    Ok(PsResult {
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        success: output.status.success(),
    })
}

/// Build the PowerShell command to start a Lability lab.
pub fn start_lab_command(config_path: &str, config_data_path: &str) -> String {
    format!(
        "Import-Module Lability; Start-LabConfiguration -ConfigurationData '{config_data_path}' -Path '{config_path}' -Verbose"
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

fn powershell_exe() -> &'static str {
    if cfg!(target_os = "windows") {
        "powershell.exe"
    } else {
        "pwsh"
    }
}
