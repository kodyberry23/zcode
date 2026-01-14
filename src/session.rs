// src/session.rs - Session management

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionManager {
    pub current_session_id: Option<String>,
    pub sessions: HashMap<String, Session>,

    #[serde(skip)]
    pub dirty: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub provider: String,
    pub created_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
    pub description: String,
    pub prompt_count: u32,
    pub working_directory: PathBuf,
}

impl SessionManager {
    pub fn load() -> Result<Self> {
        let path = Self::sessions_path();

        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&mut self) -> Result<()> {
        if !self.dirty {
            return Ok(());
        }

        let path = Self::sessions_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)?;

        self.dirty = false;
        Ok(())
    }

    fn sessions_path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("zcode")
            .join("sessions.json")
    }

    pub fn start_session(&mut self, provider: &str, cwd: &std::path::Path) -> String {
        let id = format!(
            "{}_{}",
            Utc::now().format("%Y%m%d_%H%M%S"),
            uuid_v4_simple()
        );

        let session = Session {
            id: id.clone(),
            provider: provider.to_string(),
            created_at: Utc::now(),
            last_used: Utc::now(),
            description: String::new(),
            prompt_count: 0,
            working_directory: cwd.to_path_buf(),
        };

        self.sessions.insert(id.clone(), session);
        self.current_session_id = Some(id.clone());
        self.dirty = true;

        id
    }

    pub fn update_session(&mut self, description: Option<&str>) {
        if let Some(ref id) = self.current_session_id {
            if let Some(session) = self.sessions.get_mut(id) {
                session.last_used = Utc::now();
                session.prompt_count += 1;
                if let Some(desc) = description {
                    session.description = desc.to_string();
                }
                self.dirty = true;
            }
        }
    }

    pub fn recent_sessions(&self, limit: usize) -> Vec<&Session> {
        let mut sessions: Vec<_> = self.sessions.values().collect();
        sessions.sort_by(|a, b| b.last_used.cmp(&a.last_used));
        sessions.into_iter().take(limit).collect()
    }
}

fn uuid_v4_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:016x}", seed)
}
