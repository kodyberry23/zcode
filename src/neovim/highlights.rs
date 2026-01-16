// src/neovim/highlights.rs - Neovim highlight group setup

use crate::neovim::client::NeovimWriter;
use anyhow::{Context, Result};
use nvim_rs::Neovim;

/// Set up highlight groups in Neovim for ZCode decorations
pub async fn setup_highlights(nvim: &Neovim<NeovimWriter>) -> Result<()> {
    // Deletion styling: strikethrough, dimmed
    nvim.command("highlight ZCodeDeletion gui=strikethrough guifg=#666666")
        .await
        .context("Failed to set ZCodeDeletion highlight")?;
    nvim.command("highlight ZCodeDeletionText guifg=#aa5555")
        .await
        .context("Failed to set ZCodeDeletionText highlight")?;

    // Addition styling: green background
    nvim.command("highlight ZCodeAddition guibg=#1a3320 guifg=#88cc88")
        .await
        .context("Failed to set ZCodeAddition highlight")?;

    // Pending marker
    nvim.command("highlight ZCodePending guifg=#cccc00")
        .await
        .context("Failed to set ZCodePending highlight")?;

    // Accepted marker
    nvim.command("highlight ZCodeAccepted guifg=#00cc00")
        .await
        .context("Failed to set ZCodeAccepted highlight")?;

    // Rejected marker
    nvim.command("highlight ZCodeRejected guifg=#cc0000")
        .await
        .context("Failed to set ZCodeRejected highlight")?;

    Ok(())
}
