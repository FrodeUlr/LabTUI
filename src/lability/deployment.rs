use serde::{Deserialize, Serialize};

/// Current status of a Lability lab deployment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeploymentStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed(String),
    Stopped,
}

impl std::fmt::Display for DeploymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeploymentStatus::NotStarted => write!(f, "Not Started"),
            DeploymentStatus::InProgress => write!(f, "In Progress"),
            DeploymentStatus::Completed => write!(f, "Completed"),
            DeploymentStatus::Failed(reason) => write!(f, "Failed: {}", reason),
            DeploymentStatus::Stopped => write!(f, "Stopped"),
        }
    }
}

/// Represents a deployment record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
    pub environment_name: String,
    pub status: DeploymentStatus,
    pub log_lines: Vec<String>,
}

impl Deployment {
    pub fn new(environment_name: &str) -> Self {
        Self {
            environment_name: environment_name.to_string(),
            status: DeploymentStatus::NotStarted,
            log_lines: Vec::new(),
        }
    }

    pub fn add_log(&mut self, line: impl Into<String>) {
        self.log_lines.push(line.into());
    }
}

/// Actions that can be requested against a deployment
#[derive(Debug, Clone, PartialEq)]
pub enum DeploymentAction {
    GenerateConfig,
    Start,
    Stop,
    Reset,
    Delete,
}

impl std::fmt::Display for DeploymentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeploymentAction::GenerateConfig => write!(f, "Generate Config Files"),
            DeploymentAction::Start => write!(f, "Start Lab"),
            DeploymentAction::Stop => write!(f, "Stop Lab"),
            DeploymentAction::Reset => write!(f, "Reset Lab"),
            DeploymentAction::Delete => write!(f, "Delete Lab"),
        }
    }
}
