//! Keyboard input handling and key bindings
//!
//! This module provides a trait-based system for handling keyboard input across
//! different application modes. Each mode has its own input handler that interprets
//! key presses and generates appropriate actions.
//!
//! # Modes
//!
//! - **DiffReview**: Navigate and select hunks
//! - **PromptEntry**: Enter and edit AI prompts
//! - **ProviderSelect**: Choose an AI provider
//! - **Confirmation**: Confirm or cancel operations
//!
//! # Submodules
//!
//! - [`handler`]: The `InputHandler` trait and `Action` enum
//! - [`keybindings`]: Key binding definitions and management
//! - [`modes`]: Mode-specific input handlers

pub mod handler;
pub mod keybindings;
pub mod modes;

pub use handler::{Action, InputHandler, InputResult};
pub use keybindings::{KeyCombo, KeybindingMap};
pub use modes::{
    ConfirmationHandler, DiffReviewHandler, PromptEntryHandler, ProviderSelectHandler,
};
