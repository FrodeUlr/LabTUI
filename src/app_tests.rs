#[cfg(test)]
mod tests {
    use crate::app::{App, Screen};
    use crate::lability::config::{default_media_list, LabConfig, SwitchType};
    use crate::lability::deployment::DeploymentStatus;
    use crate::ui::vm_config::VmField;

    fn make_app() -> App {
        App::new()
    }

    // ── Initialisation ─────────────────────────────────────────────────────────

    #[test]
    fn new_app_has_correct_defaults() {
        let app = make_app();
        assert_eq!(app.current_screen, Screen::Dashboard);
        assert!(!app.should_quit);
        assert_eq!(app.menu_index, 0);
        assert!(app.lab_config.nodes.is_empty());
        assert!(app.lab_config.networks.is_empty());
        assert_eq!(app.deployment.status, DeploymentStatus::NotStarted);
        assert!(!app.is_editing);
        assert!(app.editing_value.is_empty());
        assert_eq!(app.lab_output_dir, ".");
    }

    // ── Menu navigation ────────────────────────────────────────────────────────

    #[test]
    fn menu_down_increments_index() {
        let mut app = make_app();
        app.menu_down();
        assert_eq!(app.menu_index, 1);
    }

    #[test]
    fn menu_up_does_not_wrap_below_zero() {
        let mut app = make_app();
        app.menu_up();
        assert_eq!(app.menu_index, 0);
    }

    #[test]
    fn menu_down_stops_at_last_item() {
        let mut app = make_app();
        for _ in 0..100 {
            app.menu_down();
        }
        assert_eq!(app.menu_index, app.menu_items_count - 1);
    }

    #[test]
    fn menu_select_vm_config() {
        let mut app = make_app();
        app.menu_select();
        assert_eq!(app.current_screen, Screen::VmConfig);
    }

    #[test]
    fn menu_select_deployment() {
        let mut app = make_app();
        app.menu_index = 3;
        app.menu_select();
        assert_eq!(app.current_screen, Screen::Deployment);
    }

    #[test]
    fn menu_select_quit_sets_should_quit() {
        let mut app = make_app();
        app.menu_index = 5; // "Quit" entry
        app.menu_select();
        assert!(app.should_quit);
    }

    // ── Node management ────────────────────────────────────────────────────────

    #[test]
    fn add_node_increments_node_list() {
        let mut app = make_app();
        app.add_node();
        assert_eq!(app.lab_config.nodes.len(), 1);
        assert_eq!(app.selected_node, Some(0));
    }

    #[test]
    fn add_multiple_nodes() {
        let mut app = make_app();
        app.add_node();
        app.add_node();
        app.add_node();
        assert_eq!(app.lab_config.nodes.len(), 3);
        assert_eq!(app.selected_node, Some(2));
    }

    #[test]
    fn delete_selected_node_removes_it() {
        let mut app = make_app();
        app.add_node();
        app.add_node();
        app.selected_node = Some(0);
        app.delete_selected_node();
        assert_eq!(app.lab_config.nodes.len(), 1);
    }

    #[test]
    fn delete_last_node_clears_selection() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.delete_selected_node();
        assert!(app.lab_config.nodes.is_empty());
        assert_eq!(app.selected_node, None);
    }

    #[test]
    fn node_down_up_navigation() {
        let mut app = make_app();
        app.add_node();
        app.add_node();
        app.selected_node = Some(0);
        app.node_down();
        assert_eq!(app.selected_node, Some(1));
        app.node_up();
        assert_eq!(app.selected_node, Some(0));
    }

    #[test]
    fn node_names_are_unique() {
        let mut app = make_app();
        app.add_node();
        app.add_node();
        let names: Vec<_> = app.lab_config.nodes.iter().map(|n| n.node_name.clone()).collect();
        assert_ne!(names[0], names[1]);
    }

    // ── VM field increment/decrement ───────────────────────────────────────────

    #[test]
    fn vm_cpu_increment() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.vm_focused_field = VmField::CpuCount;
        let before = app.lab_config.nodes[0].cpu_count;
        app.vm_field_increment();
        assert_eq!(app.lab_config.nodes[0].cpu_count, before + 1);
    }

    #[test]
    fn vm_cpu_decrement() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.vm_focused_field = VmField::CpuCount;
        app.vm_field_increment(); // 3
        app.vm_field_decrement(); // back to 2
        assert_eq!(app.lab_config.nodes[0].cpu_count, 2);
    }

    #[test]
    fn vm_cpu_does_not_go_below_one() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.lab_config.nodes[0].cpu_count = 1;
        app.vm_focused_field = VmField::CpuCount;
        app.vm_field_decrement();
        assert_eq!(app.lab_config.nodes[0].cpu_count, 1);
    }

    // ── Inline text editing ────────────────────────────────────────────────────

    #[test]
    fn start_editing_text_field_sets_editing_flag() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.current_screen = Screen::VmConfig;
        app.vm_focused_field = VmField::NodeName;
        app.start_editing();
        assert!(app.is_editing);
        assert_eq!(app.editing_value, app.lab_config.nodes[0].node_name.clone());
    }

    #[test]
    fn start_editing_numeric_field_does_nothing() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.current_screen = Screen::VmConfig;
        app.vm_focused_field = VmField::CpuCount;
        app.start_editing();
        assert!(!app.is_editing);
    }

    #[test]
    fn push_edit_char_appends() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.current_screen = Screen::VmConfig;
        app.vm_focused_field = VmField::NodeName;
        app.start_editing();
        app.editing_value.clear();
        app.push_edit_char('D');
        app.push_edit_char('C');
        app.push_edit_char('0');
        app.push_edit_char('1');
        assert_eq!(app.editing_value, "DC01");
    }

    #[test]
    fn backspace_edit_removes_last_char() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.current_screen = Screen::VmConfig;
        app.vm_focused_field = VmField::NodeName;
        app.start_editing();
        app.editing_value = "DC01".to_string();
        app.backspace_edit();
        assert_eq!(app.editing_value, "DC0");
    }

    #[test]
    fn confirm_edit_saves_node_name() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.current_screen = Screen::VmConfig;
        app.vm_focused_field = VmField::NodeName;
        app.is_editing = true;
        app.editing_value = "MyNewNode".to_string();
        app.confirm_edit();
        assert!(!app.is_editing);
        assert_eq!(app.lab_config.nodes[0].node_name, "MyNewNode");
    }

    #[test]
    fn confirm_edit_saves_role() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.current_screen = Screen::VmConfig;
        app.vm_focused_field = VmField::Role;
        app.is_editing = true;
        app.editing_value = "DomainController".to_string();
        app.confirm_edit();
        assert_eq!(app.lab_config.nodes[0].role, "DomainController");
    }

    #[test]
    fn confirm_edit_saves_ip_address() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.current_screen = Screen::VmConfig;
        app.vm_focused_field = VmField::IpAddress;
        app.is_editing = true;
        app.editing_value = "192.168.1.10".to_string();
        app.confirm_edit();
        assert_eq!(app.lab_config.nodes[0].ip_address, Some("192.168.1.10".to_string()));
    }

    #[test]
    fn confirm_edit_clears_ip_when_empty() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.lab_config.nodes[0].ip_address = Some("10.0.0.1".to_string());
        app.current_screen = Screen::VmConfig;
        app.vm_focused_field = VmField::IpAddress;
        app.is_editing = true;
        app.editing_value = String::new();
        app.confirm_edit();
        assert_eq!(app.lab_config.nodes[0].ip_address, None);
    }

    #[test]
    fn confirm_edit_saves_media_id() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.current_screen = Screen::VmConfig;
        app.vm_focused_field = VmField::MediaId;
        app.is_editing = true;
        app.editing_value = "2019_x64_Standard_EN_Eval".to_string();
        app.confirm_edit();
        assert_eq!(app.lab_config.nodes[0].media_id, "2019_x64_Standard_EN_Eval");
    }

    #[test]
    fn cancel_edit_leaves_field_unchanged() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.current_screen = Screen::VmConfig;
        app.vm_focused_field = VmField::NodeName;
        let original = app.lab_config.nodes[0].node_name.clone();
        app.is_editing = true;
        app.editing_value = "SomethingElse".to_string();
        app.cancel_edit();
        assert!(!app.is_editing);
        assert_eq!(app.lab_config.nodes[0].node_name, original);
    }

    #[test]
    fn confirm_edit_saves_lab_name_on_dashboard() {
        let mut app = make_app();
        app.current_screen = Screen::Dashboard;
        app.is_editing = true;
        app.editing_value = "CorpLab".to_string();
        app.confirm_edit();
        assert_eq!(app.lab_config.environment_name, "CorpLab");
        assert_eq!(app.deployment.environment_name, "CorpLab");
    }

    #[test]
    fn confirm_edit_saves_network_switch_name() {
        let mut app = make_app();
        app.add_network();
        app.current_screen = Screen::Network;
        app.is_editing = true;
        app.editing_value = "CORPNET".to_string();
        app.confirm_edit();
        assert_eq!(app.lab_config.networks[0].name, "CORPNET");
    }

    #[test]
    fn confirm_edit_saves_output_dir() {
        let mut app = make_app();
        app.current_screen = Screen::Deployment;
        app.is_editing = true;
        app.editing_value = "C:\\Labs\\MyLab".to_string();
        app.confirm_edit();
        assert_eq!(app.lab_output_dir, "C:\\Labs\\MyLab");
    }

    // ── Network management ─────────────────────────────────────────────────────

    #[test]
    fn add_network_increments_list() {
        let mut app = make_app();
        app.add_network();
        assert_eq!(app.lab_config.networks.len(), 1);
        assert_eq!(app.selected_network, Some(0));
    }

    #[test]
    fn delete_network_removes_it() {
        let mut app = make_app();
        app.add_network();
        app.delete_selected_network();
        assert!(app.lab_config.networks.is_empty());
        assert_eq!(app.selected_network, None);
    }

    #[test]
    fn toggle_switch_type_cycles_correctly() {
        let mut app = make_app();
        app.add_network();
        assert_eq!(app.lab_config.networks[0].switch_type, SwitchType::Internal);
        app.toggle_switch_type();
        assert_eq!(app.lab_config.networks[0].switch_type, SwitchType::External);
        app.toggle_switch_type();
        assert_eq!(app.lab_config.networks[0].switch_type, SwitchType::Private);
        app.toggle_switch_type();
        assert_eq!(app.lab_config.networks[0].switch_type, SwitchType::Internal);
    }

    // ── Media management ───────────────────────────────────────────────────────

    #[test]
    fn media_list_is_non_empty() {
        let list = default_media_list();
        assert!(!list.is_empty());
    }

    #[test]
    fn media_down_up_navigation() {
        let mut app = make_app();
        let start = app.selected_media.unwrap_or(0);
        app.media_down();
        assert_eq!(app.selected_media, Some(start + 1));
        app.media_up();
        assert_eq!(app.selected_media, Some(start));
    }

    #[test]
    fn assign_media_updates_node() {
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        // Pick the second media entry
        app.media_down();
        app.assign_media_to_node();
        let media = default_media_list();
        let expected_id = media[app.selected_media.unwrap()].id.clone();
        assert_eq!(app.lab_config.nodes[0].media_id, expected_id);
    }

    // ── Deployment action navigation ───────────────────────────────────────────

    #[test]
    fn action_down_up() {
        let mut app = make_app();
        assert_eq!(app.selected_action, 0);
        app.action_down();
        assert_eq!(app.selected_action, 1);
        app.action_up();
        assert_eq!(app.selected_action, 0);
    }

    #[test]
    fn action_does_not_exceed_bounds() {
        let mut app = make_app();
        for _ in 0..100 {
            app.action_down();
        }
        assert_eq!(app.selected_action, 4); // max index is now 4 (5 actions)

        for _ in 0..100 {
            app.action_up();
        }
        assert_eq!(app.selected_action, 0);
    }

    // ── go_home ────────────────────────────────────────────────────────────────

    #[test]
    fn go_home_resets_to_dashboard() {
        let mut app = make_app();
        app.current_screen = Screen::Help;
        app.go_home();
        assert_eq!(app.current_screen, Screen::Dashboard);
    }

    #[test]
    fn go_home_cancels_editing() {
        let mut app = make_app();
        app.is_editing = true;
        app.editing_value = "something".to_string();
        app.go_home();
        assert!(!app.is_editing);
        assert!(app.editing_value.is_empty());
    }

    // ── LabConfig model ────────────────────────────────────────────────────────

    #[test]
    fn lab_config_new() {
        let cfg = LabConfig::new("TestLab");
        assert_eq!(cfg.environment_name, "TestLab");
        assert!(cfg.nodes.is_empty());
    }

    // ── PowerShell command builders ────────────────────────────────────────────

    #[test]
    fn ps_start_command_contains_import_module() {
        use crate::lability::powershell::start_lab_command;
        let cmd = start_lab_command("MyLab", "C:\\Labs");
        assert!(cmd.contains("Import-Module Lability"));
        assert!(cmd.contains("Start-LabConfiguration"));
        assert!(cmd.contains("MyLab.psd1"));
        assert!(cmd.contains("MyLab.ps1"));
    }

    #[test]
    fn ps_stop_command_uses_env_name() {
        use crate::lability::powershell::stop_lab_command;
        let cmd = stop_lab_command("MyLab");
        assert!(cmd.contains("MyLab"));
        assert!(cmd.contains("Stop-VM"));
    }

    #[test]
    fn ps_remove_command_uses_env_name() {
        use crate::lability::powershell::remove_lab_command;
        let cmd = remove_lab_command("MyLab");
        assert!(cmd.contains("MyLab"));
        assert!(cmd.contains("Remove-LabConfiguration"));
    }

    // ── File generation integration ────────────────────────────────────────────

    #[test]
    fn generate_config_files_writes_to_output_dir() {
        let mut app = make_app();
        app.add_node();
        app.lab_config.nodes[0].node_name = "DC01".to_string();
        let dir = tempfile::tempdir().expect("tempdir");
        app.lab_output_dir = dir.path().to_str().unwrap().to_string();
        app.generate_config_files();

        let env = &app.lab_config.environment_name;
        assert!(dir.path().join(format!("{env}.psd1")).exists());
        assert!(dir.path().join(format!("{env}.ps1")).exists());
    }

    #[test]
    fn generate_config_files_logs_ok_message() {
        let mut app = make_app();
        app.add_node();
        let dir = tempfile::tempdir().expect("tempdir");
        app.lab_output_dir = dir.path().to_str().unwrap().to_string();
        app.generate_config_files();
        assert!(app.deployment.log_lines.iter().any(|l| l.contains("[OK]")));
    }
}

