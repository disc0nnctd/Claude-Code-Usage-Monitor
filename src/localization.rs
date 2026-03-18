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
