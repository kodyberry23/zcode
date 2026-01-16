// src/neovim/mod.rs - Neovim RPC integration module

pub mod client;
pub mod extmarks;
pub mod highlights;

pub use client::NeovimClient;
pub use extmarks::ExtmarkManager;
