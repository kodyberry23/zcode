// src/neovim/client.rs - Neovim RPC client

use anyhow::{Context, Result};
use async_trait::async_trait;
use nvim_rs::{create::tokio as create, Handler, Neovim};
use parity_tokio_ipc::Connection as IpcConnection;

/// Handler for Neovim RPC events (minimal implementation)
#[derive(Clone)]
pub struct NeovimHandler;

#[async_trait]
impl Handler for NeovimHandler {
    type Writer = nvim_rs::compat::tokio::Compat<tokio::io::WriteHalf<IpcConnection>>;

    async fn handle_request(
        &self,
        _name: String,
        _args: Vec<nvim_rs::Value>,
        _neovim: Neovim<Self::Writer>,
    ) -> Result<nvim_rs::Value, nvim_rs::Value> {
        // Handle RPC requests if needed
        Err(nvim_rs::Value::Nil)
    }

    async fn handle_notify(
        &self,
        _name: String,
        _args: Vec<nvim_rs::Value>,
        _neovim: Neovim<Self::Writer>,
    ) {
        // Handle notifications from Neovim
    }
}

/// Public type alias for the Neovim Writer type
pub type NeovimWriter = <NeovimHandler as Handler>::Writer;

/// Neovim RPC client
pub struct NeovimClient {
    nvim: Option<Neovim<NeovimWriter>>,
    socket_path: Option<String>,
    _io_handle: Option<tokio::task::JoinHandle<Result<(), Box<nvim_rs::error::LoopError>>>>,
}

impl NeovimClient {
    /// Connect to a running Neovim instance via socket
    pub async fn connect(socket_path: &str) -> Result<Self> {
        let handler = NeovimHandler;

        // Use new_path to connect to existing Neovim instance via Unix socket
        let (nvim, io_handle) = create::new_path(socket_path, handler)
            .await
            .context("Failed to create Neovim client")?;

        Ok(Self {
            nvim: Some(nvim),
            socket_path: Some(socket_path.to_string()),
            _io_handle: Some(io_handle),
        })
    }

    /// Auto-detect Neovim socket from $NVIM environment variable
    pub async fn connect_auto() -> Result<Self> {
        let socket = std::env::var("NVIM")
            .context("No Neovim instance detected. Set $NVIM or run inside :terminal")?;
        Self::connect(&socket).await
    }

    /// Check if connected to Neovim
    pub fn is_connected(&self) -> bool {
        self.nvim.is_some()
    }

    /// Get the Neovim instance (if connected)
    pub fn nvim(&self) -> Option<&Neovim<NeovimWriter>> {
        self.nvim.as_ref()
    }

    /// Get mutable Neovim instance (if connected)
    pub fn nvim_mut(&mut self) -> Option<&mut Neovim<NeovimWriter>> {
        self.nvim.as_mut()
    }
}
