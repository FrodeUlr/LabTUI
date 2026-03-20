#[cfg(test)]
mod tests {
    use crate::app::{App, Screen};
    use crate::lability::config::{default_media_list, LabConfig, SwitchType};
    use crate::lability::deployment::DeploymentStatus;

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
        use crate::ui::vm_config::VmField;
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
        use crate::ui::vm_config::VmField;
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
        use crate::ui::vm_config::VmField;
        let mut app = make_app();
        app.add_node();
        app.selected_node = Some(0);
        app.lab_config.nodes[0].cpu_count = 1;
        app.vm_focused_field = VmField::CpuCount;
        app.vm_field_decrement();
        assert_eq!(app.lab_config.nodes[0].cpu_count, 1);
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
        assert_eq!(app.selected_action, 3); // max index

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
        let cmd = start_lab_command("/path/config", "/path/data");
        assert!(cmd.contains("Import-Module Lability"));
        assert!(cmd.contains("Start-LabConfiguration"));
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
}
