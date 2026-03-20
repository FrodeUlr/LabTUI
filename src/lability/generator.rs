use crate::lability::config::{LabConfig, SwitchType};
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Convert a byte count to a PowerShell memory expression, e.g. `2GB`, `512MB`.
fn bytes_to_ps_memory(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    if bytes >= GB && bytes.is_multiple_of(GB) {
        format!("{}GB", bytes / GB)
    } else if bytes >= MB && bytes.is_multiple_of(MB) {
        format!("{}MB", bytes / MB)
    } else {
        bytes.to_string()
    }
}

/// Generate the Lability configuration-data file (`.psd1`) content from a `LabConfig`.
pub fn generate_psd1(config: &LabConfig) -> String {
    let mut out = String::new();

    out.push_str("@{\n");
    out.push_str("    AllNodes = @(\n");

    for node in &config.nodes {
        out.push_str("        @{\n");
        out.push_str(&format!(
            "            NodeName                    = '{}';\n",
            node.node_name
        ));

        // Switch names derived from node's network adapters
        let switches: Vec<String> = node
            .network_adapters
            .iter()
            .map(|na| format!("'{}'", na.switch_name))
            .collect();
        out.push_str(&format!(
            "            Lability_SwitchName         = @( {} );\n",
            switches.join(", ")
        ));

        out.push_str(&format!(
            "            Lability_Media              = '{}';\n",
            node.media_id
        ));
        out.push_str(&format!(
            "            Lability_ProcessorCount     = {};\n",
            node.cpu_count
        ));
        out.push_str(&format!(
            "            Lability_StartupMemory      = {};\n",
            bytes_to_ps_memory(node.start_up_memory_bytes)
        ));
        out.push_str(&format!(
            "            Lability_MinimumMemory      = {};\n",
            bytes_to_ps_memory(node.minimum_memory_bytes)
        ));
        out.push_str(&format!(
            "            Lability_MaximumMemory      = {};\n",
            bytes_to_ps_memory(node.maximum_memory_bytes)
        ));

        if let Some(ip) = &node.ip_address
            && !ip.is_empty() {
                out.push_str(&format!(
                    "            IPAddress                   = '{}';\n",
                    ip
                ));
            }

        out.push_str("            PSDscAllowPlainTextPassword = $true;\n");
        out.push_str("            PSDscAllowDomainUser        = $true;\n");

        for (k, v) in &node.custom_data {
            out.push_str(&format!("            {k:<35} = '{v}';\n"));
        }

        out.push_str("        }\n");
    }

    out.push_str("    );\n");
    out.push_str("    NonNodeData = @{\n");
    out.push_str("        Lability = @{\n");
    out.push_str(&format!(
        "            EnvironmentPrefix = '{}';\n",
        config.environment_name
    ));
    out.push_str("            Media             = @();\n");
    out.push_str("            Network           = @(\n");

    for net in &config.networks {
        out.push_str("                @{\n");
        out.push_str(&format!("                    Name              = '{}';\n", net.name));
        let type_str = match net.switch_type {
            SwitchType::Internal => "Internal",
            SwitchType::External => "External",
            SwitchType::Private => "Private",
        };
        out.push_str(&format!("                    Type              = '{}';\n", type_str));
        let mgmt = if net.allow_management_os { "$true" } else { "$false" };
        out.push_str(&format!("                    AllowManagementOS = {};\n", mgmt));
        out.push_str("                }\n");
    }

    out.push_str("            );\n");
    out.push_str("        };\n");
    out.push_str("    };\n");
    out.push_str("}\n");

    out
}

/// Generate the DSC configuration script (`.ps1`) content from a `LabConfig`.
pub fn generate_ps1(config: &LabConfig) -> String {
    let env_name = &config.environment_name;
    let mut out = String::new();

    out.push_str("configuration LabConfiguration {\n\n");
    out.push_str("    Import-DscResource -Module PSDesiredStateConfiguration\n");

    // Collect unique extra DSC modules across all nodes + lab-level modules
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert("PSDesiredStateConfiguration".to_string());

    for node in &config.nodes {
        for resource in &node.dsc_resources {
            if seen.insert(resource.clone()) {
                out.push_str(&format!("    Import-DscResource -Module {resource}\n"));
            }
        }
    }
    for module in &config.dsc_modules {
        if seen.insert(module.module_name.clone()) {
            out.push_str(&format!(
                "    Import-DscResource -Module {}\n",
                module.module_name
            ));
        }
    }

    out.push_str("\n    Node $AllNodes.NodeName {\n\n");
    out.push_str("    }\n\n");
    out.push_str("}\n\n");
    out.push_str(&format!(
        "LabConfiguration -ConfigurationData '.\\{env_name}.psd1' -OutputPath '.\\{env_name}'\n"
    ));

    out
}

/// Write both config files to `output_dir` and return their paths as `(psd1, ps1)`.
pub fn write_lab_files(config: &LabConfig, output_dir: &str) -> Result<(String, String)> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir)
        .with_context(|| format!("Cannot create output directory: {output_dir}"))?;

    let psd1_path = dir.join(format!("{}.psd1", config.environment_name));
    let ps1_path = dir.join(format!("{}.ps1", config.environment_name));

    fs::write(&psd1_path, generate_psd1(config))
        .with_context(|| format!("Cannot write {}", psd1_path.display()))?;
    fs::write(&ps1_path, generate_ps1(config))
        .with_context(|| format!("Cannot write {}", ps1_path.display()))?;

    Ok((
        psd1_path.to_string_lossy().into_owned(),
        ps1_path.to_string_lossy().into_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lability::config::{
        DscModule, LabConfig, NetworkAdapter, NodeConfig, SwitchType, VirtualSwitch,
    };

    fn make_config() -> LabConfig {
        let mut cfg = LabConfig::new("TestLab");
        cfg.nodes.push(NodeConfig {
            node_name: "DC01".to_string(),
            ip_address: Some("192.168.1.10".to_string()),
            role: "DomainController".to_string(),
            cpu_count: 2,
            start_up_memory_bytes: 2 * 1024 * 1024 * 1024,
            minimum_memory_bytes: 512 * 1024 * 1024,
            maximum_memory_bytes: 4 * 1024 * 1024 * 1024,
            media_id: "2022_x64_Standard_EN_Eval".to_string(),
            network_adapters: vec![NetworkAdapter {
                switch_name: "CORPNET".to_string(),
                mac_address: None,
            }],
            dsc_resources: vec!["xActiveDirectory".to_string()],
            custom_data: Default::default(),
        });
        cfg.networks.push(VirtualSwitch {
            name: "CORPNET".to_string(),
            switch_type: SwitchType::Internal,
            allow_management_os: false,
        });
        cfg.dsc_modules.push(DscModule::new("xNetworking", "5.7.0.0"));
        cfg
    }

    #[test]
    fn bytes_to_ps_memory_gb() {
        assert_eq!(bytes_to_ps_memory(2 * 1024 * 1024 * 1024), "2GB");
    }

    #[test]
    fn bytes_to_ps_memory_mb() {
        assert_eq!(bytes_to_ps_memory(512 * 1024 * 1024), "512MB");
    }

    #[test]
    fn bytes_to_ps_memory_raw() {
        assert_eq!(bytes_to_ps_memory(1_000_000), "1000000");
    }

    #[test]
    fn psd1_contains_node_name() {
        let cfg = make_config();
        let psd1 = generate_psd1(&cfg);
        assert!(psd1.contains("NodeName                    = 'DC01'"));
    }

    #[test]
    fn psd1_contains_media() {
        let cfg = make_config();
        let psd1 = generate_psd1(&cfg);
        assert!(psd1.contains("Lability_Media              = '2022_x64_Standard_EN_Eval'"));
    }

    #[test]
    fn psd1_contains_memory_as_gb() {
        let cfg = make_config();
        let psd1 = generate_psd1(&cfg);
        assert!(psd1.contains("Lability_StartupMemory      = 2GB"));
        assert!(psd1.contains("Lability_MinimumMemory      = 512MB"));
        assert!(psd1.contains("Lability_MaximumMemory      = 4GB"));
    }

    #[test]
    fn psd1_contains_switch_name() {
        let cfg = make_config();
        let psd1 = generate_psd1(&cfg);
        assert!(psd1.contains("'CORPNET'"));
    }

    #[test]
    fn psd1_contains_network_section() {
        let cfg = make_config();
        let psd1 = generate_psd1(&cfg);
        assert!(psd1.contains("Name              = 'CORPNET'"));
        assert!(psd1.contains("Type              = 'Internal'"));
    }

    #[test]
    fn psd1_contains_ip_address() {
        let cfg = make_config();
        let psd1 = generate_psd1(&cfg);
        assert!(psd1.contains("IPAddress                   = '192.168.1.10'"));
    }

    #[test]
    fn psd1_omits_empty_ip() {
        let mut cfg = make_config();
        cfg.nodes[0].ip_address = None;
        let psd1 = generate_psd1(&cfg);
        assert!(!psd1.contains("IPAddress"));
    }

    #[test]
    fn ps1_contains_configuration_block() {
        let cfg = make_config();
        let ps1 = generate_ps1(&cfg);
        assert!(ps1.contains("configuration LabConfiguration"));
        assert!(ps1.contains("Node $AllNodes.NodeName"));
        // LocalConfigurationManager must NOT appear inside the configuration
        // block — placing it there makes PowerShell DSC compile only a
        // .meta.mof (LCM config) instead of the required .mof node config.
        assert!(!ps1.contains("LocalConfigurationManager"));
    }

    #[test]
    fn ps1_node_block_has_no_lcm() {
        // Regression guard: ensure the Node block never re-introduces
        // LocalConfigurationManager. Its presence would cause DSC to produce
        // only Node01.meta.mof and skip Node01.mof, breaking Start-LabConfiguration.
        let cfg = make_config();
        let ps1 = generate_ps1(&cfg);
        assert!(
            !ps1.contains("LocalConfigurationManager"),
            "LocalConfigurationManager must not appear in the generated PS1 – \
             it causes DSC to generate only .meta.mof instead of .mof"
        );
    }

    #[test]
    fn ps1_contains_dsc_imports() {
        let cfg = make_config();
        let ps1 = generate_ps1(&cfg);
        assert!(ps1.contains("Import-DscResource -Module PSDesiredStateConfiguration"));
        assert!(ps1.contains("Import-DscResource -Module xActiveDirectory"));
        assert!(ps1.contains("Import-DscResource -Module xNetworking"));
    }

    #[test]
    fn ps1_does_not_duplicate_dsc_imports() {
        let mut cfg = make_config();
        // Add same resource at node level and lab module level
        cfg.dsc_modules.push(DscModule::new("xActiveDirectory", "3.0.0.0"));
        let ps1 = generate_ps1(&cfg);
        assert_eq!(
            ps1.matches("Import-DscResource -Module xActiveDirectory").count(),
            1
        );
    }

    #[test]
    fn ps1_ends_with_invocation() {
        let cfg = make_config();
        let ps1 = generate_ps1(&cfg);
        assert!(ps1.contains(
            "LabConfiguration -ConfigurationData '.\\TestLab.psd1' -OutputPath '.\\TestLab'"
        ));
    }

    #[test]
    fn write_lab_files_creates_files() {
        let cfg = make_config();
        let dir = tempfile::tempdir().expect("create temp dir");
        let out = dir.path().to_str().unwrap();
        let (psd1, ps1) = write_lab_files(&cfg, out).expect("write files");
        assert!(std::path::Path::new(&psd1).exists());
        assert!(std::path::Path::new(&ps1).exists());
    }
}
