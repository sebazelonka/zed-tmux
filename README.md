# Zed Tmux Integration

Automatically open tmux sessions based on your current Zed project.

## Features

- Opens tmux session corresponding to your current Zed project
- Session name is derived from project folder name
- Automatically attaches to existing session or creates a new one
- Sanitizes special characters for valid tmux session names

## Installation

### As a Dev Extension

1. Clone this repository:
   ```bash
   git clone https://github.com/sebazelonka/zed-tmux.git
   cd zed-tmux
   ```

2. Install Rust via rustup (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Build the extension:
   ```bash
   cargo build --release
   ```

4. In Zed, open the Extensions panel (Cmd+Shift+X)
5. Click "Install Dev Extension"
6. Select the directory containing this extension

## Usage

Once installed, open the command palette (Cmd+Shift+P) and type:

```
/tmux
```

This will attach to or create a tmux session named after your current project folder.

### Example

If your project is at `/home/user/projects/my-app`, this extension will:

1. Extract the project name: `my-app`
2. Try to attach to tmux session: `my-app`
3. If it doesn't exist, create a new session: `my-app`
4. Attach to that session

## Requirements

- [Zed](https://zed.dev/)
- [tmux](https://github.com/tmux/tmux) installed and available in PATH
- [Rust](https://www.rust-lang.org/) (for building)

## Session Name Rules

The session name is derived from your project folder:

- **Path:** `/home/user/projects/my-app` → **Session:** `my-app`
- **Path:** `/home/user/projects/work/cool-project` → **Session:** `cool-project`
- **Path:** `/home/user/Projects/2024/portfolio` → **Session:** `2024_portfolio`

Special characters (`.` ` ` `:` `/` `\`) are replaced with `_` for valid tmux session names.

## Limitations

- Does not detect project changes automatically (Zed doesn't expose project change events to extensions)
- Requires manual execution of the `/tmux` command
- Requires tmux to be installed on your system

## Future Improvements

- [ ] Auto-detect project changes (requires Zed API enhancement)
- [ ] Configuration for custom tmux commands
- [ ] Support for multiple session profiles per project
- [ ] Add command to list available tmux sessions

## License

MIT License - see LICENSE file for details
