use windows::core::PWSTR;
use windows::Win32::Globalization::{
    GetUserDefaultLocaleName, GetUserDefaultUILanguage, GetUserPreferredUILanguages,
    LCIDToLocaleName, LOCALE_ALLOW_NEUTRAL_NAMES, MAX_LOCALE_NAME, MUI_LANGUAGE_NAME,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanguageId {
    English,
    Spanish,
    French,
    German,
    Japanese,
}

impl LanguageId {
    pub const ALL: [LanguageId; 5] = [
        LanguageId::English,
        LanguageId::Spanish,
        LanguageId::French,
        LanguageId::German,
        LanguageId::Japanese,
    ];

    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Spanish => "es",
            Self::French => "fr",
            Self::German => "de",
            Self::Japanese => "ja",
        }
    }

    pub fn native_name(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Spanish => "Español",
            Self::French => "Français",
            Self::German => "Deutsch",
            Self::Japanese => "日本語",
        }
    }

    pub fn strings(self) -> Strings {
        match self {
            Self::English => ENGLISH,
            Self::Spanish => SPANISH,
            Self::French => FRENCH,
            Self::German => GERMAN,
            Self::Japanese => JAPANESE,
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        let normalized = code.trim().replace('_', "-").to_ascii_lowercase();
        if normalized.is_empty() || normalized == "system" {
            return None;
        }

        match normalized.split('-').next().unwrap_or_default() {
            "en" => Some(Self::English),
            "es" => Some(Self::Spanish),
            "fr" => Some(Self::French),
            "de" => Some(Self::German),
            "ja" => Some(Self::Japanese),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Strings {
    pub window_title: &'static str,
    pub refresh: &'static str,
    pub update_frequency: &'static str,
    pub one_minute: &'static str,
    pub five_minutes: &'static str,
    pub fifteen_minutes: &'static str,
    pub one_hour: &'static str,
    pub settings: &'static str,
    pub start_with_windows: &'static str,
    pub reset_position: &'static str,
    pub language: &'static str,
    pub system_default: &'static str,
    pub check_for_updates: &'static str,
    pub checking_for_updates: &'static str,
    pub updates: &'static str,
    pub update_in_progress: &'static str,
    pub up_to_date: &'static str,
    pub up_to_date_short: &'static str,
    pub update_failed: &'static str,
    pub applying_update: &'static str,
    pub update_to: &'static str,
    pub exit: &'static str,
    pub session_window: &'static str,
    pub weekly_window: &'static str,
    pub now: &'static str,
    pub day_suffix: &'static str,
    pub hour_suffix: &'static str,
    pub minute_suffix: &'static str,
    pub second_suffix: &'static str,
}

const ENGLISH: Strings = Strings {
    window_title: "Claude Code Usage Monitor",
    refresh: "Refresh",
    update_frequency: "Update Frequency",
    one_minute: "1 Minute",
    five_minutes: "5 Minutes",
    fifteen_minutes: "15 Minutes",
    one_hour: "1 Hour",
    settings: "Settings",
    start_with_windows: "Start with Windows",
    reset_position: "Reset Position",
    language: "Language",
    system_default: "System Default",
    check_for_updates: "Check for Updates",
    checking_for_updates: "Checking for Updates...",
    updates: "Updates",
    update_in_progress: "An update check is already in progress.",
    up_to_date: "You already have the latest version.",
    up_to_date_short: "Up to date",
    update_failed: "Unable to update automatically",
    applying_update: "Applying update...",
    update_to: "Update to",
    exit: "Exit",
    session_window: "5h",
    weekly_window: "7d",
    now: "now",
    day_suffix: "d",
    hour_suffix: "h",
    minute_suffix: "m",
    second_suffix: "s",
};

const SPANISH: Strings = Strings {
    window_title: "Monitor de uso de Claude Code",
    refresh: "Actualizar",
    update_frequency: "Frecuencia de actualizacion",
    one_minute: "1 minuto",
    five_minutes: "5 minutos",
    fifteen_minutes: "15 minutos",
    one_hour: "1 hora",
    settings: "Configuracion",
    start_with_windows: "Iniciar con Windows",
    reset_position: "Restablecer posicion",
    language: "Idioma",
    system_default: "Predeterminado del sistema",
    check_for_updates: "Buscar actualizaciones",
    checking_for_updates: "Buscando actualizaciones...",
    updates: "Actualizaciones",
    update_in_progress: "Ya hay una comprobacion de actualizacion en curso.",
    up_to_date: "Ya tienes la version mas reciente.",
    up_to_date_short: "Actualizado",
    update_failed: "No se pudo actualizar automaticamente",
    applying_update: "Aplicando actualizacion...",
    update_to: "Actualizar a",
    exit: "Salir",
    session_window: "5h",
    weekly_window: "7d",
    now: "ahora",
    day_suffix: "d",
    hour_suffix: "h",
    minute_suffix: "m",
    second_suffix: "s",
};

const FRENCH: Strings = Strings {
    window_title: "Moniteur d'utilisation Claude Code",
    refresh: "Actualiser",
    update_frequency: "Frequence de mise a jour",
    one_minute: "1 minute",
    five_minutes: "5 minutes",
    fifteen_minutes: "15 minutes",
    one_hour: "1 heure",
    settings: "Parametres",
    start_with_windows: "Demarrer avec Windows",
    reset_position: "Reinitialiser la position",
    language: "Langue",
    system_default: "Par defaut du systeme",
    check_for_updates: "Verifier les mises a jour",
    checking_for_updates: "Verification des mises a jour...",
    updates: "Mises a jour",
    update_in_progress: "Une verification de mise a jour est deja en cours.",
    up_to_date: "Vous utilisez deja la version la plus recente.",
    up_to_date_short: "A jour",
    update_failed: "Impossible d'effectuer la mise a jour automatiquement",
    applying_update: "Application de la mise a jour...",
    update_to: "Mettre a jour vers",
    exit: "Quitter",
    session_window: "5h",
    weekly_window: "7d",
    now: "maintenant",
    day_suffix: "j",
    hour_suffix: "h",
    minute_suffix: "m",
    second_suffix: "s",
};

const GERMAN: Strings = Strings {
    window_title: "Claude Code Nutzungsmonitor",
    refresh: "Aktualisieren",
    update_frequency: "Aktualisierungsintervall",
    one_minute: "1 Minute",
    five_minutes: "5 Minuten",
    fifteen_minutes: "15 Minuten",
    one_hour: "1 Stunde",
    settings: "Einstellungen",
    start_with_windows: "Mit Windows starten",
    reset_position: "Position zurucksetzen",
    language: "Sprache",
    system_default: "Systemstandard",
    check_for_updates: "Nach Updates suchen",
    checking_for_updates: "Suche nach Updates...",
    updates: "Updates",
    update_in_progress: "Eine Update-Prufung lauft bereits.",
    up_to_date: "Sie verwenden bereits die neueste Version.",
    up_to_date_short: "Aktuell",
    update_failed: "Automatisches Update war nicht moglich",
    applying_update: "Update wird installiert...",
    update_to: "Aktualisieren auf",
    exit: "Beenden",
    session_window: "5h",
    weekly_window: "7d",
    now: "jetzt",
    day_suffix: "T",
    hour_suffix: "h",
    minute_suffix: "m",
    second_suffix: "s",
};

const JAPANESE: Strings = Strings {
    window_title: "Claude Code 使用量モニター",
    refresh: "更新",
    update_frequency: "更新間隔",
    one_minute: "1分",
    five_minutes: "5分",
    fifteen_minutes: "15分",
    one_hour: "1時間",
    settings: "設定",
    start_with_windows: "Windows と同時に開始",
    reset_position: "位置をリセット",
    language: "言語",
    system_default: "システム既定",
    check_for_updates: "更新を確認",
    checking_for_updates: "更新を確認しています...",
    updates: "更新",
    update_in_progress: "更新確認は既に実行中です。",
    up_to_date: "既に最新バージョンです。",
    up_to_date_short: "最新です",
    update_failed: "自動更新を完了できませんでした",
    applying_update: "更新を適用しています...",
    update_to: "更新先",
    exit: "終了",
    session_window: "5h",
    weekly_window: "7d",
    now: "今",
    day_suffix: "日",
    hour_suffix: "時間",
    minute_suffix: "分",
    second_suffix: "秒",
};

pub fn resolve_language(language_override: Option<LanguageId>) -> LanguageId {
    language_override.unwrap_or_else(detect_system_language)
}

pub fn detect_system_language() -> LanguageId {
    preferred_ui_languages()
        .into_iter()
        .find_map(|locale| LanguageId::from_code(&locale))
        .or_else(default_ui_locale)
        .or_else(default_locale_name)
        .unwrap_or(LanguageId::English)
}

fn preferred_ui_languages() -> Vec<String> {
    unsafe {
        let mut num_languages = 0u32;
        let mut buffer_len = 0u32;
        if GetUserPreferredUILanguages(
            MUI_LANGUAGE_NAME,
            &mut num_languages,
            PWSTR::null(),
            &mut buffer_len,
        )
        .is_err()
            || buffer_len == 0
        {
            return Vec::new();
        }

        let mut buffer = vec![0u16; buffer_len as usize];
        if GetUserPreferredUILanguages(
            MUI_LANGUAGE_NAME,
            &mut num_languages,
            PWSTR(buffer.as_mut_ptr()),
            &mut buffer_len,
        )
        .is_err()
        {
            return Vec::new();
        }

        buffer
            .split(|unit| *unit == 0)
            .filter(|part| !part.is_empty())
            .map(String::from_utf16_lossy)
            .collect()
    }
}

fn default_ui_locale() -> Option<LanguageId> {
    unsafe {
        let lang_id = GetUserDefaultUILanguage();
        let mut buffer = [0u16; MAX_LOCALE_NAME as usize];
        let len = LCIDToLocaleName(
            lang_id as u32,
            Some(&mut buffer),
            LOCALE_ALLOW_NEUTRAL_NAMES,
        );
        if len <= 1 {
            return None;
        }
        let locale = String::from_utf16_lossy(&buffer[..(len as usize - 1)]);
        LanguageId::from_code(&locale)
    }
}

fn default_locale_name() -> Option<LanguageId> {
    unsafe {
        let mut buffer = [0u16; MAX_LOCALE_NAME as usize];
        let len = GetUserDefaultLocaleName(&mut buffer);
        if len <= 1 {
            return None;
        }
        let locale = String::from_utf16_lossy(&buffer[..(len as usize - 1)]);
        LanguageId::from_code(&locale)
    }
}
