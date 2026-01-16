// src/neovim/extmarks.rs - Neovim extmark API integration

use crate::neovim::client::NeovimWriter;
use anyhow::{Context, Result};
use nvim_rs::{Neovim, Value};

/// Namespace for ZCode extmarks (keeps our marks separate from other plugins)
const ZCODE_NS: &str = "zcode_diff";

/// Manager for Neovim extmarks
pub struct ExtmarkManager {
    namespace_id: i64,
}

impl ExtmarkManager {
    /// Create namespace for ZCode marks
    pub async fn init(nvim: &Neovim<NeovimWriter>) -> Result<Self> {
        let ns_id_value = nvim
            .call("nvim_create_namespace", vec![Value::from(ZCODE_NS)])
            .await;

        let ns_id = match ns_id_value {
            Ok(Ok(Value::Integer(i))) => i.as_i64().context("Invalid namespace ID")?,
            Ok(Ok(_)) => {
                return Err(anyhow::anyhow!(
                    "Invalid namespace ID type returned from Neovim"
                ))
            }
            Ok(Err(e)) => return Err(anyhow::anyhow!("Neovim RPC error: {:?}", e)),
            Err(e) => return Err(anyhow::anyhow!("Failed to call Neovim: {:?}", e)),
        };

        Ok(Self {
            namespace_id: ns_id,
        })
    }

    /// Add a deletion decoration (strikethrough + dimmed)
    pub async fn mark_deletion(
        &self,
        nvim: &Neovim<NeovimWriter>,
        buf: i64,
        line: usize,
        text: &str,
    ) -> Result<i64> {
        // Build options as a Vec of (Value, Value) pairs
        // virt_text needs to be Value::Array containing an array of [text, hl_group]
        let virt_text = Value::Array(vec![Value::Array(vec![
            Value::from(text),
            Value::from("ZCodeDeletionText"),
        ])]);

        let opts = vec![
            (Value::from("hl_group"), Value::from("ZCodeDeletion")),
            (Value::from("virt_text"), virt_text),
            (Value::from("virt_text_pos"), Value::from("overlay")),
            (Value::from("priority"), Value::from(100)),
        ];

        let result = nvim
            .call(
                "nvim_buf_set_extmark",
                vec![
                    Value::from(buf),
                    Value::from(self.namespace_id),
                    Value::from(line as i64),
                    Value::from(0),
                    Value::Map(opts.into_iter().collect()),
                ],
            )
            .await;

        match result {
            Ok(Ok(Value::Integer(i))) => i.as_i64().context("Invalid extmark ID"),
            Ok(Ok(_)) => Err(anyhow::anyhow!("Invalid extmark ID type returned")),
            Ok(Err(e)) => Err(anyhow::anyhow!("Neovim RPC error: {:?}", e)),
            Err(e) => Err(anyhow::anyhow!("Failed to set deletion extmark: {:?}", e)),
        }
    }

    /// Add an addition decoration (virtual text on new line)
    pub async fn mark_addition(
        &self,
        nvim: &Neovim<NeovimWriter>,
        buf: i64,
        line: usize,
        text: &str,
    ) -> Result<i64> {
        // Build options as a Vec of (Value, Value) pairs
        // virt_lines needs to be Value::Array containing an array of lines
        let virt_lines = Value::Array(vec![Value::Array(vec![Value::Array(vec![
            Value::from(text),
            Value::from("ZCodeAddition"),
        ])])]);

        let opts = vec![
            (Value::from("virt_lines"), virt_lines),
            (Value::from("virt_lines_above"), Value::from(false)),
            (Value::from("priority"), Value::from(100)),
        ];

        let result = nvim
            .call(
                "nvim_buf_set_extmark",
                vec![
                    Value::from(buf),
                    Value::from(self.namespace_id),
                    Value::from(line as i64),
                    Value::from(0),
                    Value::Map(opts.into_iter().collect()),
                ],
            )
            .await;

        match result {
            Ok(Ok(Value::Integer(i))) => i.as_i64().context("Invalid extmark ID"),
            Ok(Ok(_)) => Err(anyhow::anyhow!("Invalid extmark ID type returned")),
            Ok(Err(e)) => Err(anyhow::anyhow!("Neovim RPC error: {:?}", e)),
            Err(e) => Err(anyhow::anyhow!("Failed to set addition extmark: {:?}", e)),
        }
    }

    /// Clear all ZCode extmarks from a buffer
    pub async fn clear_buffer(&self, nvim: &Neovim<NeovimWriter>, buf: i64) -> Result<()> {
        let result = nvim
            .call(
                "nvim_buf_clear_namespace",
                vec![
                    Value::from(buf),
                    Value::from(self.namespace_id),
                    Value::from(0),
                    Value::from(-1),
                ],
            )
            .await;

        match result {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(anyhow::anyhow!("Neovim RPC error clearing buffer: {:?}", e)),
            Err(e) => Err(anyhow::anyhow!("Failed to clear buffer extmarks: {:?}", e)),
        }
    }

    /// Clear all ZCode extmarks from all buffers
    pub async fn clear_all(&self, nvim: &Neovim<NeovimWriter>) -> Result<()> {
        let bufs_value = nvim.call("nvim_list_bufs", vec![]).await;

        let bufs = match bufs_value {
            Ok(Ok(Value::Array(arr))) => arr,
            Ok(Ok(_)) => {
                return Err(anyhow::anyhow!(
                    "Invalid buffer list type returned from Neovim"
                ))
            }
            Ok(Err(e)) => return Err(anyhow::anyhow!("Neovim RPC error: {:?}", e)),
            Err(e) => return Err(anyhow::anyhow!("Failed to list Neovim buffers: {:?}", e)),
        };

        for buf_value in bufs {
            let buf_num = match buf_value {
                Value::Integer(i) => i.as_i64().context("Invalid buffer number")?,
                _ => continue, // Skip invalid buffer entries
            };
            self.clear_buffer(nvim, buf_num).await?;
        }
        Ok(())
    }

    /// Get namespace ID
    pub fn namespace_id(&self) -> i64 {
        self.namespace_id
    }
}
