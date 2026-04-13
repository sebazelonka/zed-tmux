# Zed Tmux Integration

Automatically open tmux sessions based on your current Zed project.

## Features

- ✅ Detects current Zed project using `$ZED_WORKTREE_ROOT` variable
- ✅ Opens tmux session matching the project folder name
- ✅ Creates new session if it doesn't exist
- ✅ Attaches to existing session if available
- ✅ Keybindings for quick access (Alt+T, Alt+S, Alt+Cmd+T)
- ✅ Shows terminal in center area
- ✅ Hides terminal after success
- ✅ Supports concurrent runs (can switch between projects)

## Installation

### Option 1: Copy files to Zed config

1. Create directory: `~/.config/zed/tmux-integration/`
2. Copy all files from this repo:
   - `tasks.json` → `~/.config/zed/tmux-integration/tasks.json`
   - `keymap.json` → `~/.config/zed/tmux-integration/keymap.json`

### Option 2: Clone repository

```bash
cd ~/.config/zed
git clone https://github.com/sebazelonka/zed-tmux.git tmux-integration
```

Then open Zed settings and add the `tmux-integration` directory to your workspace.

## Usage

### With Keybindings (Recommended)

Press one of these keys:
- **Alt+T** - Quick toggle (any layout)
- **Alt+S** - Quick toggle (any layout)
- **Alt+Cmd+T** - Quick toggle (any layout)

### From Command Palette

1. Press `Cmd-Shift-P` (or `Ctrl-Shift-P`)
2. Type "tmux"
3. Press Enter

The task will automatically:
1. Get the current project path from `$ZED_WORKTREE_ROOT`
2. Extract the folder name (e.g., `my-project` from `/path/to/my-project`)
3. Sanitize it for tmux (replace special chars with `_`)
4. Attach to existing session if available
5. Create new session with `-s -A` flag if it doesn't exist

## How It Works

### Session Naming

The tmux session name is derived from your project folder:

| Project Path | Tmux Session |
|-------------|---------------|
| `/home/user/projects/my-app` | `my_app` |
| `/home/user/projects/work/project` | `project` |
| `/home/user/projects/site-2024` | `site_2024` |

**Sanitization:** Special characters (`.`, ` `, `:`, `/`, `\`) are replaced with `_` to avoid tmux issues.

### Behavior

**If session exists:**
```
tmux attach -t session_name
```

**If session doesn't exist:**
```
tmux new-session -s -A session_name
```

The `-s -A` flag (session-name-after) ensures the session is detached before creating it.

## Configuration Options

### Terminal Focus

The task uses `reveal: "always"` which means:
- Terminal tab is always shown
- Focus moves to the terminal after task completes
- Works with both new and existing terminals

### Concurrent Runs

`allow_concurrent_runs: true` allows you to:
- Switch between tmux sessions without waiting
- Cancel previous tasks
- Have multiple sessions running

### Hide Behavior

`hide: "on_success"` means:
- Terminal tab hides after tmux command completes successfully
- Keeps your workspace clean
- Reappears when you run the task again

## Customization

### Change Keybindings

Edit `~/.config/zed/tmux-integration/keymap.json`:

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "ctrl-t": "task::Spawn"  // Change to your preferred key
    }
  }
]
```

### Modify Tmux Command

Edit `~/.config/zed/tmux-integration/tasks.json`:

```json
{
  "command": "tmux attach -t my_custom_session",
  "args": [],
  "env": {},
  "cwd": "$ZED_WORKTREE_ROOT",
  ...
}
```

### Use Different Shell

If you need a specific shell, add to `tasks.json`:

```json
{
  "command": "...",
  "shell": {
    "program": "/bin/zsh"
  },
  ...
}
```

## Troubleshooting

### Tmux command not found

If you get "tmux: command not found":
1. Install tmux: `brew install tmux`
2. Or add tmux to PATH
3. Restart Zed

### Session name issues

If tmux session names look weird, check:
1. Project folder name contains special characters
2. Manual session exists with that name
3. Try `tmux list-sessions` to see active sessions

### Task not appearing in task picker

1. Make sure files are in `~/.config/zed/tmux-integration/`
2. Restart Zed
3. Try running `zed: open tasks` to verify it's loaded

## Alternative: Manual Tmux Management

If you prefer managing tmux manually, consider:

- **[tmux-resurrect](https://github.com/tmux-plugins/tmux-resurrect)** - Save/restore tmux sessions
- **[tmux-continuum](https://github.com/tmux-plugins/tmux-continuum)** - Continuous saving of tmux environment

These plugins can automatically save and restore your tmux sessions, making manual switching unnecessary.

## License

MIT
