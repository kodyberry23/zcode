// src/input/command_mode.rs - Command mode parser and executor

use crate::state::{MessageFilter, State};
use anyhow::{Context, Result};
use std::path::PathBuf;

/// Command types that can be executed
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Config(ConfigSubcommand),
    Model(String),
    Provider(String),
    Jump(usize),
    Filter(MessageFilter),
    Pin(PathBuf),
    Search(String),
    Neovim(NeovimSubcommand),
    Help,
    Quit,
    Save,
    Load(String),
    Clear,
    Export,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigSubcommand {
    Show,
    Set { key: String, value: String },
    Edit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NeovimSubcommand {
    Connect,
    Push,
    Clear,
    Status,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandError {
    UnknownCommand,
    InvalidArguments,
    MissingArgument,
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::UnknownCommand => write!(f, "Unknown command"),
            CommandError::InvalidArguments => write!(f, "Invalid arguments"),
            CommandError::MissingArgument => write!(f, "Missing required argument"),
        }
    }
}

impl std::error::Error for CommandError {}

/// Parse a command string into a Command enum
pub fn parse_command(input: &str) -> Result<Command, CommandError> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.is_empty() {
        return Err(CommandError::UnknownCommand);
    }

    match parts[0] {
        "config" => parse_config_command(&parts[1..]),
        "model" => {
            let model = parts.get(1).ok_or(CommandError::MissingArgument)?;
            Ok(Command::Model(model.to_string()))
        }
        "provider" => {
            let provider = parts.get(1).ok_or(CommandError::MissingArgument)?;
            Ok(Command::Provider(provider.to_string()))
        }
        "jump" => {
            let num_str = parts.get(1).ok_or(CommandError::MissingArgument)?;
            let num = num_str
                .parse()
                .map_err(|_| CommandError::InvalidArguments)?;
            Ok(Command::Jump(num))
        }
        "filter" => {
            let filter_type = parts.get(1).ok_or(CommandError::MissingArgument)?;
            let filter = match *filter_type {
                "error" => MessageFilter::Error,
                "success" => MessageFilter::Success,
                "all" => MessageFilter::All,
                _ => return Err(CommandError::InvalidArguments),
            };
            Ok(Command::Filter(filter))
        }
        "pin" => {
            let file_str = parts.get(1).ok_or(CommandError::MissingArgument)?;
            Ok(Command::Pin(PathBuf::from(file_str)))
        }
        "search" => {
            let query = parts[1..].join(" ");
            Ok(Command::Search(query))
        }
        "neovim" => parse_neovim_command(&parts[1..]),
        "help" | "h" => Ok(Command::Help),
        "quit" | "q" => Ok(Command::Quit),
        "save" => Ok(Command::Save),
        "load" => {
            let session = parts.get(1).ok_or(CommandError::MissingArgument)?;
            Ok(Command::Load(session.to_string()))
        }
        "clear" => Ok(Command::Clear),
        "export" => Ok(Command::Export),
        _ => Err(CommandError::UnknownCommand),
    }
}

fn parse_config_command(parts: &[&str]) -> Result<Command, CommandError> {
    if parts.is_empty() {
        return Ok(Command::Config(ConfigSubcommand::Show));
    }

    match parts[0] {
        "set" => {
            if parts.len() < 3 {
                return Err(CommandError::MissingArgument);
            }
            Ok(Command::Config(ConfigSubcommand::Set {
                key: parts[1].to_string(),
                value: parts[2..].join(" "),
            }))
        }
        "edit" => Ok(Command::Config(ConfigSubcommand::Edit)),
        _ => Err(CommandError::InvalidArguments),
    }
}

fn parse_neovim_command(parts: &[&str]) -> Result<Command, CommandError> {
    let subcmd = parts.get(0).ok_or(CommandError::MissingArgument)?;
    match *subcmd {
        "connect" => Ok(Command::Neovim(NeovimSubcommand::Connect)),
        "push" => Ok(Command::Neovim(NeovimSubcommand::Push)),
        "clear" => Ok(Command::Neovim(NeovimSubcommand::Clear)),
        "status" => Ok(Command::Neovim(NeovimSubcommand::Status)),
        _ => Err(CommandError::InvalidArguments),
    }
}

/// Execute a command on the state
pub fn execute_command(command: &Command, state: &mut State) -> Result<String> {
    match command {
        Command::Config(ConfigSubcommand::Show) => Ok(format!("Config: {:?}", state.config)),
        Command::Config(ConfigSubcommand::Set { key, value }) => {
            // TODO: Implement config setting
            Ok(format!("Setting {} = {}", key, value))
        }
        Command::Config(ConfigSubcommand::Edit) => Ok("Opening config editor...".to_string()),
        Command::Model(model) => {
            state.status_info.model = model.clone();
            Ok(format!("Switched to model: {}", model))
        }
        Command::Provider(provider) => {
            // TODO: Switch provider
            Ok(format!("Switching to provider: {}", provider))
        }
        Command::Jump(message_id) => {
            use crate::ui::search::jump_to_message;
            if jump_to_message(&mut state.chat_history, *message_id) {
                Ok(format!("Jumped to message {}", message_id))
            } else {
                Ok(format!("Message {} not found", message_id))
            }
        }
        Command::Filter(filter) => {
            use crate::ui::search::apply_filter;
            apply_filter(&mut state.chat_history, filter.clone());
            Ok(format!("Filter applied: {:?}", filter))
        }
        Command::Pin(file_path) => {
            use crate::ui::sidebar::pin_file;
            pin_file(&mut state.sidebar_state, file_path.clone());
            Ok(format!("Pinned file: {}", file_path.display()))
        }
        Command::Search(query) => {
            state.chat_history.search_query = Some(query.clone());
            Ok(format!("Searching for: {}", query))
        }
        Command::Neovim(subcmd) => match subcmd {
            NeovimSubcommand::Connect => Ok("Connecting to Neovim...".to_string()),
            NeovimSubcommand::Push => Ok("Pushing overlays to Neovim...".to_string()),
            NeovimSubcommand::Clear => Ok("Clearing Neovim overlays...".to_string()),
            NeovimSubcommand::Status => Ok("Neovim status: Not connected".to_string()),
        },
        Command::Help => Ok("Help: Press ? for help screen".to_string()),
        Command::Quit => {
            state.mode = crate::state::Mode::ProviderSelect;
            Ok("Quitting...".to_string())
        }
        Command::Save => {
            // TODO: Save session
            Ok("Session saved".to_string())
        }
        Command::Load(session_id) => {
            // TODO: Load session
            Ok(format!("Loading session: {}", session_id))
        }
        Command::Clear => {
            state.chat_history.messages.clear();
            Ok("Chat history cleared".to_string())
        }
        Command::Export => {
            // TODO: Export config
            Ok("Config exported".to_string())
        }
    }
}
