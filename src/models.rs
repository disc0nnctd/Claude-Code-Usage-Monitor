use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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
            Self::Claude => "CLD",
            Self::Codex => "CDX",
        }
    }
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
