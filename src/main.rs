//! ZCode - A Zellij plugin for AI code assistant integration
//!
//! ZCode enables seamless integration of AI tools (Claude, Aider, and more) directly
//! into your Zellij workspace. It provides an interactive diff viewer for reviewing
//! AI-generated code changes, with safe file operations and automatic backups.
//!
//! # Features
//!
//! - **Multi-provider support**: Works with Claude, Aider, and custom AI tools
//! - **Interactive diff viewer**: Navigate and selectively apply code changes
//! - **Safe file operations**: Atomic writes with automatic backups and rollback
//! - **Configurable**: Custom keybindings, providers, and display options
//!
//! # Architecture
//!
//! The plugin is organized into several key modules:
//!
//! - `state`: Central application state machine
//! - `ui`: User interface components and rendering
//! - `input`: Keyboard input handling and key bindings
//! - `file_ops`: File operations with atomic writes and backups
//! - `providers`: AI provider implementations
//! - `diff`: Diff generation and hunk extraction
//! - `parsers`: Parsing AI tool outputs
//! - `config`: Configuration management

// Allow certain lints for stub code that will be implemented later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::manual_clamp)]
#![allow(clippy::default_constructed_unit_structs)]

mod config;
mod diff;
mod error;
mod input;
mod parsers;
mod providers;
mod session;
mod state;
mod ui;

// file_ops is now a module (directory)
mod file_ops;

use std::collections::BTreeMap;
use zellij_tile::prelude::*;

use crate::state::State;

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
            PermissionType::OpenFiles,
            PermissionType::RunCommands,
        ]);

        subscribe(&[EventType::Key, EventType::PermissionRequestResult]);

        let _ = self.initialize(&_configuration);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => self.handle_key(&key),
            Event::PermissionRequestResult(result) => {
                self.permissions_granted =
                    matches!(result, zellij_tile::prelude::PermissionStatus::Granted);
                true
            }
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        use crate::ui::{RenderContext, Renderer};

        self.viewport_rows = rows;
        self.viewport_cols = cols;

        if !self.permissions_granted {
            println!("{}Requesting permissions...", crate::ui::RESET);
            return;
        }

        // Create render context
        let colors = if self.config.display.color_scheme == "light" {
            crate::ui::Colors::light()
        } else {
            crate::ui::Colors::dark()
        };
        let ctx = RenderContext::new(rows, cols, colors);

        // Dispatch to appropriate renderer
        if self.last_error.is_some() {
            let renderer = crate::ui::renderer::ErrorRenderer;
            renderer.render(self, &ctx);
        } else {
            let renderer = crate::ui::renderer::get_renderer_for_mode(&self.mode);
            renderer.render(self, &ctx);
        }
    }
}
