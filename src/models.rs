use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderKind {
    #[default]
    Claude,
    Codex,
}

impl ProviderKind {
    pub const ALL: [ProviderKind; 2] = [ProviderKind::Claude, ProviderKind::Codex];

    pub fn short_label(self) -> &'static str {
        match self {
            Self::Claude => "C",
            Self::Codex => "O",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryStorageMode {
    #[default]
    Local,
    RemoteOnly,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistorySyncSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(default = "default_true")]
    pub upload_history_store: bool,
    #[serde(default = "default_true")]
    pub upload_html_reports: bool,
    #[serde(default)]
    pub upload_conversations: bool,
    #[serde(default)]
    pub prefer_remote_reports: bool,
    #[serde(default)]
    pub workflow_file: Option<String>,
    #[serde(default)]
    pub storage_mode: HistoryStorageMode,
}

impl Default for HistorySyncSettings {
    fn default() -> Self {
        Self {
            repo_url: None,
            branch: None,
            upload_history_store: true,
            upload_html_reports: true,
            upload_conversations: false,
            prefer_remote_reports: false,
            workflow_file: None,
            storage_mode: HistoryStorageMode::Local,
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Default)]
pub struct UsageSection {
    pub percentage: f64,
    pub resets_at: Option<SystemTime>,
}

#[derive(Clone, Debug, Default)]
pub struct UsageData {
    pub session: UsageSection,
    pub weekly: UsageSection,
    pub review: Option<UsageSection>,
    pub credits_remaining: Option<f64>,
    pub has_credits: bool,
    pub unlimited_credits: bool,
    pub account_email: Option<String>,
    pub plan_name: Option<String>,
    pub source_label: Option<String>,
    pub updated_at: Option<SystemTime>,
}

#[derive(Clone, Debug, Default)]
pub struct ActivitySummary {
    pub prompts_last_24h: u32,
    pub prompts_last_7d: u32,
    pub sessions_last_24h: u32,
    pub sessions_last_7d: u32,
    pub last_prompt: Option<String>,
    pub last_prompt_at: Option<SystemTime>,
}

#[derive(Clone, Debug, Default)]
pub struct ProjectUsageEntry {
    pub project_path: String,
    pub sessions: u32,
    pub input_tokens: u64,
    pub cached_input_tokens: u64,
    pub output_tokens: u64,
    pub reasoning_output_tokens: u64,
    pub estimated_cost_usd: f64,
}

#[derive(Clone, Debug, Default)]
pub struct HistorySummary {
    pub total_projects: u32,
    pub total_sessions: u32,
    pub input_tokens: u64,
    pub cached_input_tokens: u64,
    pub output_tokens: u64,
    pub reasoning_output_tokens: u64,
    pub estimated_cost_usd: f64,
    pub unpriced_sessions: u32,
    pub top_projects: Vec<ProjectUsageEntry>,
}

#[derive(Clone, Debug, Default)]
pub struct HistorySnapshot {
    pub claude: HistorySummary,
    pub codex: HistorySummary,
}
