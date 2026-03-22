#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Lability node (VM) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub node_name: String,
    pub ip_address: Option<String>,
    pub role: String,
    pub cpu_count: u32,
    pub start_up_memory_bytes: u64,
    pub minimum_memory_bytes: u64,
    pub maximum_memory_bytes: u64,
    pub media_id: String,
    pub network_adapters: Vec<NetworkAdapter>,
    pub dsc_resources: Vec<String>,
    pub custom_data: HashMap<String, String>,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node_name: String::from("LabNode"),
            ip_address: None,
            role: String::from("Default"),
            cpu_count: 2,
            start_up_memory_bytes: 2 * 1024 * 1024 * 1024,    // 2 GB
            minimum_memory_bytes: 512 * 1024 * 1024,           // 512 MB
            maximum_memory_bytes: 4 * 1024 * 1024 * 1024,      // 4 GB
            media_id: String::from("2022_x64_Standard_EN_Eval"),
            network_adapters: vec![NetworkAdapter {
                switch_name: String::from("Default Switch"),
                mac_address: None,
            }],
            dsc_resources: Vec::new(),
            custom_data: HashMap::new(),
        }
    }
}

/// Represents a virtual network adapter for a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAdapter {
    pub switch_name: String,
    pub mac_address: Option<String>,
}

/// Represents the entire lab configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LabConfig {
    pub environment_name: String,
    pub default_switch_name: String,
    pub nodes: Vec<NodeConfig>,
    pub dsc_modules: Vec<DscModule>,
    pub networks: Vec<VirtualSwitch>,
}

impl LabConfig {
    pub fn new(environment_name: &str) -> Self {
        Self {
            environment_name: environment_name.to_string(),
            default_switch_name: String::from("Default Switch"),
            nodes: Vec::new(),
            dsc_modules: Vec::new(),
            networks: Vec::new(),
        }
    }
}

/// Represents a DSC (Desired State Configuration) module/resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DscModule {
    pub module_name: String,
    pub module_version: String,
    pub repository: Option<String>,
}

impl DscModule {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            module_name: name.to_string(),
            module_version: version.to_string(),
            repository: None,
        }
    }
}

/// Represents a Hyper-V virtual switch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualSwitch {
    pub name: String,
    pub switch_type: SwitchType,
    pub allow_management_os: bool,
}

impl Default for VirtualSwitch {
    fn default() -> Self {
        Self {
            name: String::from("Default Switch"),
            switch_type: SwitchType::Internal,
            allow_management_os: true,
        }
    }
}

/// Type of Hyper-V virtual switch
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SwitchType {
    Internal,
    External,
    Private,
}

impl std::fmt::Display for SwitchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwitchType::Internal => write!(f, "Internal"),
            SwitchType::External => write!(f, "External"),
            SwitchType::Private => write!(f, "Private"),
        }
    }
}

/// Known Lability media entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaEntry {
    pub id: String,
    pub description: String,
    pub architecture: String,
    pub media_type: String,
}

/// Return the built-in Lability media list
pub fn default_media_list() -> Vec<MediaEntry> {
    vec![
        MediaEntry {
            id: String::from("2022_x64_Standard_EN_Eval"),
            description: String::from("Windows Server 2022 Standard 64-bit English Evaluation"),
            architecture: String::from("x86_64"),
            media_type: String::from("ISO"),
        },
        MediaEntry {
            id: String::from("2022_x64_Datacenter_EN_Eval"),
            description: String::from("Windows Server 2022 Datacenter 64-bit English Evaluation"),
            architecture: String::from("x86_64"),
            media_type: String::from("ISO"),
        },
        MediaEntry {
            id: String::from("2019_x64_Standard_EN_Eval"),
            description: String::from("Windows Server 2019 Standard 64-bit English Evaluation"),
            architecture: String::from("x86_64"),
            media_type: String::from("ISO"),
        },
        MediaEntry {
            id: String::from("2019_x64_Datacenter_EN_Eval"),
            description: String::from("Windows Server 2019 Datacenter 64-bit English Evaluation"),
            architecture: String::from("x86_64"),
            media_type: String::from("ISO"),
        },
        MediaEntry {
            id: String::from("2016_x64_Standard_EN_Eval"),
            description: String::from("Windows Server 2016 Standard 64-bit English Evaluation"),
            architecture: String::from("x86_64"),
            media_type: String::from("ISO"),
        },
        MediaEntry {
            id: String::from("WIN10_x64_Enterprise_EN_Eval"),
            description: String::from("Windows 10 Enterprise 64-bit English Evaluation"),
            architecture: String::from("x86_64"),
            media_type: String::from("ISO"),
        },
        MediaEntry {
            id: String::from("WIN11_x64_Enterprise_EN_Eval"),
            description: String::from("Windows 11 Enterprise 64-bit English Evaluation"),
            architecture: String::from("x86_64"),
            media_type: String::from("ISO"),
        },
    ]
}

/// Common DSC resources used with Lability
pub fn common_dsc_modules() -> Vec<DscModule> {
    vec![
        DscModule::new("xActiveDirectory", "3.0.0.0"),
        DscModule::new("xDnsServer", "2.0.0.0"),
        DscModule::new("xDhcpServer", "3.0.0.0"),
        DscModule::new("xNetworking", "5.7.0.0"),
        DscModule::new("xComputerManagement", "4.1.0.0"),
        DscModule::new("xWebAdministration", "3.3.0.0"),
        DscModule::new("PSDesiredStateConfiguration", "1.1"),
        DscModule::new("xSmbShare", "2.2.0.0"),
        DscModule::new("xRemoteDesktopAdmin", "1.1.0.0"),
        DscModule::new("xWindowsUpdate", "2.8.0.0"),
    ]
}
