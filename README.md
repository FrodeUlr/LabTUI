# LabTUI

A terminal user interface (TUI) written in Rust for managing
[Lability](https://github.com/VirtualEngine/Lability) – the PowerShell module
that automates building Hyper-V lab environments.

## Features

| Screen | What you can do |
|---|---|
| **Dashboard** | Lab overview – name, node count, deployment status |
| **VM Configuration** | Add / remove nodes; set CPU, memory, media, IP, DSC resources |
| **Network Setup** | Add virtual switches; cycle type (Internal / External / Private) |
| **Media Selection** | Browse built-in Windows Server / Desktop evaluation images; assign to nodes |
| **Deployment** | Start / Stop / Reset / Delete a lab via PowerShell + Lability |
| **Help** | Full key-binding reference |

## Requirements

- Windows 10 / 11 or Windows Server 2016+ with **Hyper-V** enabled
- PowerShell 5.1+ or PowerShell 7+ (`pwsh`)
- [Lability](https://github.com/VirtualEngine/Lability) module:
  ```powershell
  Install-Module -Name Lability -Force
  ```

## Building

```bash
cargo build --release
```

The resulting binary is `./target/release/labtui`.

## Usage

```bash
./target/release/labtui
```

### Key bindings

| Key | Action |
|---|---|
| `↑` / `↓` | Move selection |
| `Enter` | Confirm / activate |
| `Esc` | Return to main menu |
| `q` / `Q` | Quit |
| `?` | Open help screen |
| `a` | Add node / switch (VM Config / Network screens) |
| `d` | Delete selected node / switch |
| `Tab` / `Shift-Tab` | Cycle between editable fields (VM Config) |
| `+` / `-` | Increment / decrement numeric fields (CPU, memory) |
| `t` | Toggle switch type (Network screen) |

## Screen overview

### Dashboard

```
┌──────────────────────────────────┐┌ Lab Overview ──────────────────────────────────┐
│ _          _   _______  _   _    ││                                                 │
│| |   __ _ | |_|_   _| || || |_ _|││   Lab Name     : MyLab                          │
│| |_ / _` || '_ \| || || || | | | ││   Nodes        : 2                               │
│|___|\__,_||_.__/|_| \___/|_|___ ││   DSC Modules  : 0                               │
│ Lability Lab Manager v0.1.0      ││   Networks     : 1                               │
└──────────────────────────────────┘│                                                 │
┌ Menu ────────────────────────────┐│   Deployment   : Not Started                    │
│ ▶   VM Configuration             │└─────────────────────────────────────────────────┘
│     Network Setup                │
│     Media Selection              │
│     Deployment                   │
│     Help                         │
│     Quit                         │
└──────────────────────────────────┘
```

### VM Configuration

```
┌ Nodes ────────────────────────┐┌ Node: Node01 ─────────────────────────────────────┐
│   Node01   2 CPU  2GB         ││   Node Name             : Node01▌                  │
│ ▶ Node02   2 CPU  2GB         ││   Role                  : Default                  │
│                               ││   CPU Count             : 2                        │
│                               ││   Startup Memory (GB)   : 2.0                      │
│                               ││   Media ID              : 2022_x64_Standard_EN_Eval│
│                               ││   IP Address            :                           │
│                               ││                                                    │
│                               ││   DSC Resources: (none)                            │
│                               ││   Network Adapters:                                │
│                               ││     • Switch: Default Switch                       │
└───────────────────────────────┘└────────────────────────────────────────────────────┘
```

## Project structure

```
src/
├── main.rs              # Terminal setup, event loop, key dispatch
├── app.rs               # Central application state & business logic
├── lability/
│   ├── mod.rs
│   ├── config.rs        # Data models: NodeConfig, LabConfig, DscModule, VirtualSwitch
│   ├── deployment.rs    # DeploymentStatus, Deployment, DeploymentAction
│   ├── media.rs         # MediaRepository (built-in + custom entries)
│   └── powershell.rs    # PowerShell command builders & runner
└── ui/
    ├── mod.rs
    ├── dashboard.rs     # Main menu + lab overview panel
    ├── vm_config.rs     # Node list + field editor
    ├── network.rs       # Virtual switch list + detail
    ├── media.rs         # Media browser + detail panel
    ├── deployment.rs    # Action panel + live log view
    └── help.rs          # Key-binding reference
```

## License

MIT
