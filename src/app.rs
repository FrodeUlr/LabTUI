use crate::lability::{
    config::LabConfig,
    deployment::{Deployment, DeploymentAction, DeploymentStatus},
    powershell,
};
use crate::ui::vm_config::VmField;

/// Which screen is currently shown
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Dashboard,
    VmConfig,
    Network,
    Media,
    Deployment,
    Help,
}

/// Central application state
pub struct App {
    pub current_screen: Screen,
    pub should_quit: bool,

    // Lab config
    pub lab_config: LabConfig,

    // Deployment
    pub deployment: Deployment,

    // Dashboard menu selection (0-based index into menu items)
    pub menu_index: usize,
    pub menu_items_count: usize,

    // VM config screen
    pub selected_node: Option<usize>,
    pub vm_focused_field: VmField,

    // Network screen
    pub selected_network: Option<usize>,

    // Media screen
    pub selected_media: Option<usize>,

    // Deployment screen
    pub selected_action: usize,
}

impl App {
    pub fn new() -> Self {
        let lab_config = LabConfig::new("MyLab");
        let deployment = Deployment::new("MyLab");
        Self {
            current_screen: Screen::Dashboard,
            should_quit: false,
            lab_config,
            deployment,
            menu_index: 0,
            menu_items_count: 6, // number of menu entries in dashboard
            selected_node: None,
            vm_focused_field: VmField::default(),
            selected_network: None,
            selected_media: Some(0),
            selected_action: 0,
        }
    }

    /// Go back to the dashboard
    pub fn go_home(&mut self) {
        self.current_screen = Screen::Dashboard;
    }

    /// Navigate the menu up
    pub fn menu_up(&mut self) {
        if self.menu_index > 0 {
            self.menu_index -= 1;
        }
    }

    /// Navigate the menu down
    pub fn menu_down(&mut self) {
        if self.menu_index + 1 < self.menu_items_count {
            self.menu_index += 1;
        }
    }

    /// Activate the currently highlighted menu item
    pub fn menu_select(&mut self) {
        self.current_screen = match self.menu_index {
            0 => Screen::VmConfig,
            1 => Screen::Network,
            2 => Screen::Media,
            3 => Screen::Deployment,
            4 => Screen::Help,
            5 => {
                self.should_quit = true;
                Screen::Dashboard
            }
            _ => Screen::Dashboard,
        };
    }

    // ── VM Config helpers ──────────────────────────────────────────────────────

    pub fn node_up(&mut self) {
        if let Some(idx) = self.selected_node {
            if idx > 0 {
                self.selected_node = Some(idx - 1);
            }
        } else if !self.lab_config.nodes.is_empty() {
            self.selected_node = Some(0);
        }
    }

    pub fn node_down(&mut self) {
        let len = self.lab_config.nodes.len();
        if len == 0 {
            return;
        }
        match self.selected_node {
            None => self.selected_node = Some(0),
            Some(idx) if idx + 1 < len => self.selected_node = Some(idx + 1),
            _ => {}
        }
    }

    pub fn add_node(&mut self) {
        let idx = self.lab_config.nodes.len();
        let node = crate::ui::vm_config::new_node(idx);
        self.lab_config.nodes.push(node);
        self.selected_node = Some(idx);
    }

    pub fn delete_selected_node(&mut self) {
        if let Some(idx) = self.selected_node
            && idx < self.lab_config.nodes.len() {
                self.lab_config.nodes.remove(idx);
                self.selected_node = if self.lab_config.nodes.is_empty() {
                    None
                } else {
                    Some(idx.saturating_sub(1))
                };
            }
    }

    /// Cycle the VmField focus forward
    pub fn vm_field_next(&mut self) {
        self.vm_focused_field = match self.vm_focused_field {
            VmField::NodeName => VmField::Role,
            VmField::Role => VmField::CpuCount,
            VmField::CpuCount => VmField::StartMemory,
            VmField::StartMemory => VmField::MinMemory,
            VmField::MinMemory => VmField::MaxMemory,
            VmField::MaxMemory => VmField::MediaId,
            VmField::MediaId => VmField::IpAddress,
            VmField::IpAddress => VmField::NodeName,
        };
    }

    /// Cycle the VmField focus backward
    pub fn vm_field_prev(&mut self) {
        self.vm_focused_field = match self.vm_focused_field {
            VmField::NodeName => VmField::IpAddress,
            VmField::Role => VmField::NodeName,
            VmField::CpuCount => VmField::Role,
            VmField::StartMemory => VmField::CpuCount,
            VmField::MinMemory => VmField::StartMemory,
            VmField::MaxMemory => VmField::MinMemory,
            VmField::MediaId => VmField::MaxMemory,
            VmField::IpAddress => VmField::MediaId,
        };
    }

    /// Increment the focused numeric field for the selected node
    pub fn vm_field_increment(&mut self) {
        if let Some(idx) = self.selected_node
            && let Some(node) = self.lab_config.nodes.get_mut(idx) {
                match self.vm_focused_field {
                    VmField::CpuCount => node.cpu_count = (node.cpu_count + 1).min(64),
                    VmField::StartMemory => {
                        node.start_up_memory_bytes =
                            (node.start_up_memory_bytes + 512 * 1024 * 1024).min(64 * 1024 * 1024 * 1024)
                    }
                    VmField::MinMemory => {
                        node.minimum_memory_bytes =
                            (node.minimum_memory_bytes + 512 * 1024 * 1024).min(64 * 1024 * 1024 * 1024)
                    }
                    VmField::MaxMemory => {
                        node.maximum_memory_bytes =
                            (node.maximum_memory_bytes + 512 * 1024 * 1024).min(64 * 1024 * 1024 * 1024)
                    }
                    _ => {}
                }
            }
    }

    /// Decrement the focused numeric field for the selected node
    pub fn vm_field_decrement(&mut self) {
        if let Some(idx) = self.selected_node
            && let Some(node) = self.lab_config.nodes.get_mut(idx) {
                match self.vm_focused_field {
                    VmField::CpuCount => {
                        if node.cpu_count > 1 {
                            node.cpu_count -= 1;
                        }
                    }
                    VmField::StartMemory => {
                        node.start_up_memory_bytes = node
                            .start_up_memory_bytes
                            .saturating_sub(512 * 1024 * 1024)
                            .max(512 * 1024 * 1024)
                    }
                    VmField::MinMemory => {
                        node.minimum_memory_bytes = node
                            .minimum_memory_bytes
                            .saturating_sub(512 * 1024 * 1024)
                            .max(256 * 1024 * 1024)
                    }
                    VmField::MaxMemory => {
                        node.maximum_memory_bytes = node
                            .maximum_memory_bytes
                            .saturating_sub(512 * 1024 * 1024)
                            .max(512 * 1024 * 1024)
                    }
                    _ => {}
                }
            }
    }

    // ── Network helpers ────────────────────────────────────────────────────────

    pub fn network_up(&mut self) {
        if let Some(idx) = self.selected_network {
            if idx > 0 {
                self.selected_network = Some(idx - 1);
            }
        } else if !self.lab_config.networks.is_empty() {
            self.selected_network = Some(0);
        }
    }

    pub fn network_down(&mut self) {
        let len = self.lab_config.networks.len();
        if len == 0 {
            return;
        }
        match self.selected_network {
            None => self.selected_network = Some(0),
            Some(idx) if idx + 1 < len => self.selected_network = Some(idx + 1),
            _ => {}
        }
    }

    pub fn add_network(&mut self) {
        let idx = self.lab_config.networks.len();
        let sw = crate::ui::network::new_switch(idx);
        self.lab_config.networks.push(sw);
        self.selected_network = Some(idx);
    }

    pub fn delete_selected_network(&mut self) {
        if let Some(idx) = self.selected_network
            && idx < self.lab_config.networks.len() {
                self.lab_config.networks.remove(idx);
                self.selected_network = if self.lab_config.networks.is_empty() {
                    None
                } else {
                    Some(idx.saturating_sub(1))
                };
            }
    }

    pub fn toggle_switch_type(&mut self) {
        if let Some(idx) = self.selected_network
            && let Some(sw) = self.lab_config.networks.get_mut(idx) {
                crate::ui::network::cycle_switch_type(sw);
            }
    }

    // ── Media helpers ──────────────────────────────────────────────────────────

    pub fn media_up(&mut self) {
        let len = crate::lability::config::default_media_list().len();
        if len == 0 {
            return;
        }
        self.selected_media = Some(match self.selected_media {
            None => 0,
            Some(0) => 0,
            Some(i) => i - 1,
        });
    }

    pub fn media_down(&mut self) {
        let len = crate::lability::config::default_media_list().len();
        if len == 0 {
            return;
        }
        self.selected_media = Some(match self.selected_media {
            None => 0,
            Some(i) if i + 1 < len => i + 1,
            Some(i) => i,
        });
    }

    /// Assign the selected media to the currently selected node
    pub fn assign_media_to_node(&mut self) {
        let media_list = crate::lability::config::default_media_list();
        if let (Some(m_idx), Some(n_idx)) = (self.selected_media, self.selected_node)
            && let (Some(media), Some(node)) =
                (media_list.get(m_idx), self.lab_config.nodes.get_mut(n_idx))
            {
                node.media_id = media.id.clone();
            }
    }

    // ── Deployment helpers ─────────────────────────────────────────────────────

    pub fn action_up(&mut self) {
        if self.selected_action > 0 {
            self.selected_action -= 1;
        }
    }

    pub fn action_down(&mut self) {
        if self.selected_action + 1 < 4 {
            self.selected_action += 1;
        }
    }

    /// Execute the highlighted deployment action
    pub fn run_deployment_action(&mut self) {
        let action = match self.selected_action {
            0 => DeploymentAction::Start,
            1 => DeploymentAction::Stop,
            2 => DeploymentAction::Reset,
            3 => DeploymentAction::Delete,
            _ => return,
        };

        match action {
            DeploymentAction::Start => self.do_start(),
            DeploymentAction::Stop => self.do_stop(),
            DeploymentAction::Reset => self.do_reset(),
            DeploymentAction::Delete => self.do_delete(),
        }
    }

    fn do_start(&mut self) {
        self.deployment.status = DeploymentStatus::InProgress;
        self.deployment.add_log("[INFO] Starting lab configuration …");

        let cmd = powershell::start_lab_command(".", ".");
        match powershell::run_powershell(&cmd) {
            Ok(result) => {
                for line in result.stdout.lines() {
                    self.deployment.add_log(line);
                }
                if result.success {
                    self.deployment.status = DeploymentStatus::Completed;
                    self.deployment.add_log("[OK] Lab started successfully.");
                } else {
                    for line in result.stderr.lines() {
                        self.deployment.add_log(format!("[ERROR] {line}"));
                    }
                    self.deployment.status =
                        DeploymentStatus::Failed("PowerShell reported an error".to_string());
                }
            }
            Err(e) => {
                self.deployment
                    .add_log(format!("[ERROR] Could not launch PowerShell: {e}"));
                self.deployment.status =
                    DeploymentStatus::Failed(format!("Launch error: {e}"));
            }
        }
    }

    fn do_stop(&mut self) {
        self.deployment.add_log("[INFO] Stopping lab VMs …");
        let cmd = powershell::stop_lab_command(&self.lab_config.environment_name);
        match powershell::run_powershell(&cmd) {
            Ok(result) => {
                if result.success {
                    self.deployment.status = DeploymentStatus::Stopped;
                    self.deployment.add_log("[OK] Lab stopped.");
                } else {
                    for line in result.stderr.lines() {
                        self.deployment.add_log(format!("[ERROR] {line}"));
                    }
                    self.deployment.status =
                        DeploymentStatus::Failed("Stop command failed".to_string());
                }
            }
            Err(e) => {
                self.deployment
                    .add_log(format!("[ERROR] Could not launch PowerShell: {e}"));
                self.deployment.status =
                    DeploymentStatus::Failed(format!("Launch error: {e}"));
            }
        }
    }

    fn do_reset(&mut self) {
        self.deployment.add_log("[INFO] Resetting lab …");
        self.do_delete();
        self.do_start();
    }

    fn do_delete(&mut self) {
        self.deployment.add_log("[INFO] Removing lab …");
        let cmd = powershell::remove_lab_command(&self.lab_config.environment_name);
        match powershell::run_powershell(&cmd) {
            Ok(result) => {
                if result.success {
                    self.deployment.status = DeploymentStatus::NotStarted;
                    self.deployment.add_log("[OK] Lab removed.");
                } else {
                    for line in result.stderr.lines() {
                        self.deployment.add_log(format!("[ERROR] {line}"));
                    }
                    self.deployment.status =
                        DeploymentStatus::Failed("Delete command failed".to_string());
                }
            }
            Err(e) => {
                self.deployment
                    .add_log(format!("[ERROR] Could not launch PowerShell: {e}"));
                self.deployment.status =
                    DeploymentStatus::Failed(format!("Launch error: {e}"));
            }
        }
    }
}
