use zed_extension_api as zed;
use serde::{Deserialize, Serialize};
use std::path::Path;

struct TmuxExtension {
    // State could go here if needed
}

impl zed::Extension for TmuxExtension {
    fn new() -> Self {
        Self {}
    }

    fn run_slash_command(
        &mut self,
        command: zed::SlashCommand,
        args: Vec<String>,
        worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        if command != "tmux" {
            return Err("Unknown command. Use: /tmux".to_string());
        }

        let worktree = worktree.ok_or("No worktree active")?;

        let project_path = worktree.root_path()?;
        let session_name = extract_session_name(&project_path)?;

        // Execute tmux command
        let tmux_command = zed::Command {
            command: "tmux".to_string(),
            args: vec![
                "attach-session".to_string(),
                "-t".to_string(),
                session_name.clone(),
            ],
            env: vec![],
        };

        let output = tmux_command.output()?;

        if output.status_code == 0 {
            Ok(zed::SlashCommandOutput {
                sections: vec![],
                text: format!("Attached to tmux session: {}", session_name),
            })
        } else {
            // Try creating a new session if attach failed
            let new_session_command = zed::Command {
                command: "tmux".to_string(),
                args: vec![
                    "new-session".to_string(),
                    "-s".to_string(),
                    "-A".to_string(),
                    session_name.clone(),
                ],
                env: vec![],
            };

            let create_output = new_session_command.output()?;
            if create_output.status_code == 0 {
                Ok(zed::SlashCommandOutput {
                    sections: vec![],
                    text: format!("Created and attached to tmux session: {}", session_name),
                })
            } else {
                Err(format!(
                    "Failed to create tmux session. stdout: {}, stderr: {}",
                    create_output.stdout,
                    create_output.stderr
                ))
            }
        }
    }
}

/// Extract a session name from the project path
/// Example: /home/user/projects/my-app -> my-app
/// Example: /home/user/projects/work/my-project -> my-project
fn extract_session_name(path: &str) -> Result<String, String> {
    let path = Path::new(path);

    // Get the folder name (last component of the path)
    let session_name = path
        .file_name()
        .ok_or("Invalid path")?
        .to_str()
        .ok_or("Invalid UTF-8")?
        .to_string();

    // Sanitize session name (tmux doesn't like special chars)
    let sanitized = session_name
        .chars()
        .map(|c| {
            match c {
                '.' | ' ' | ':' | '/' | '\\' => '_',
                _ => c,
            }
        })
        .collect::<String>();

    Ok(sanitized)
}

zed::register_extension!(TmuxExtension);
