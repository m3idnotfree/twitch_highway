use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CclLocale {
    #[serde(rename = "bg-BG")]
    bgBG,
    #[serde(rename = "cs-CZ")]
    csCZ,
    #[serde(rename = "da-DK")]
    daDK,
    #[serde(rename = "de-DE")]
    deDE,
    #[serde(rename = "el-GR")]
    elGR,
    #[serde(rename = "en-GB")]
    enGB,
    #[serde(rename = "en-US")]
    enUS,
    #[serde(rename = "es-ES")]
    esES,
    #[serde(rename = "es-MX")]
    esMX,
    #[serde(rename = "fi-FI")]
    fiFI,
    #[serde(rename = "fr-FR")]
    frFR,
    #[serde(rename = "hu-HU")]
    huHU,
    #[serde(rename = "it-IT")]
    itIT,
    #[serde(rename = "ja-JP")]
    jaJP,
    #[serde(rename = "ko-KR")]
    koKR,
    #[serde(rename = "nl-NL")]
    nlNL,
    #[serde(rename = "no-NO")]
    noNO,
    #[serde(rename = "pl-PL")]
    plPL,
    #[serde(rename = "pt-BT")]
    ptBT,
    #[serde(rename = "pt-PT")]
    ptPT,
    #[serde(rename = "ro-RO")]
    roRO,
    #[serde(rename = "ru-RU")]
    ruRU,
    #[serde(rename = "sk-SK")]
    skSK,
    #[serde(rename = "sv-SE")]
    svSE,
    #[serde(rename = "th-TH")]
    thTH,
    #[serde(rename = "tr-TR")]
    trTR,
    #[serde(rename = "vi-VN")]
    viVN,
    #[serde(rename = "zh-CN")]
    zkCN,
    #[serde(rename = "zh-TW")]
    zhTW,
}

impl CclLocale {
    pub fn as_str(&self) -> &str {
        match self {
            Self::bgBG => "bg-BG",
            Self::csCZ => "cs-CZ",
            Self::daDK => "da-DK",
            Self::deDE => "de-DE",
            Self::elGR => "el-GR",
            Self::enGB => "en-GB",
            Self::enUS => "en-US",
            Self::esES => "es-ES",
            Self::esMX => "es-MX",
            Self::fiFI => "fi-FI",
            Self::frFR => "fr-FR",
            Self::huHU => "hu-HU",
            Self::itIT => "it-IT",
            Self::jaJP => "ja-JP",
            Self::koKR => "ko-KR",
            Self::nlNL => "nl-NL",
            Self::noNO => "no-NO",
            Self::plPL => "pl-PL",
            Self::ptBT => "pt-BT",
            Self::ptPT => "pt-PT",
            Self::roRO => "ro-RO",
            Self::ruRU => "ru-RU",
            Self::skSK => "sk-SK",
            Self::svSE => "sv-SE",
            Self::thTH => "th-TH",
            Self::trTR => "tr-TR",
            Self::viVN => "vi-VN",
            Self::zkCN => "zh-CN",
            Self::zhTW => "zh-TW",
        }
    }
}

impl AsRef<str> for CclLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::ccls::request::CclLocale;

    #[test]
    fn ccls_locale_enum_all_variants() {
        let locales = vec![
            (CclLocale::bgBG, "bg-BG"),
            (CclLocale::csCZ, "cs-CZ"),
            (CclLocale::daDK, "da-DK"),
            (CclLocale::deDE, "de-DE"),
            (CclLocale::elGR, "el-GR"),
            (CclLocale::enGB, "en-GB"),
            (CclLocale::enUS, "en-US"),
            (CclLocale::esES, "es-ES"),
            (CclLocale::esMX, "es-MX"),
            (CclLocale::fiFI, "fi-FI"),
            (CclLocale::frFR, "fr-FR"),
            (CclLocale::huHU, "hu-HU"),
            (CclLocale::itIT, "it-IT"),
            (CclLocale::jaJP, "ja-JP"),
            (CclLocale::koKR, "ko-KR"),
            (CclLocale::nlNL, "nl-NL"),
            (CclLocale::noNO, "no-NO"),
            (CclLocale::plPL, "pl-PL"),
            (CclLocale::ptBT, "pt-BT"),
            (CclLocale::ptPT, "pt-PT"),
            (CclLocale::roRO, "ro-RO"),
            (CclLocale::ruRU, "ru-RU"),
            (CclLocale::skSK, "sk-SK"),
            (CclLocale::svSE, "sv-SE"),
            (CclLocale::thTH, "th-TH"),
            (CclLocale::trTR, "tr-TR"),
            (CclLocale::viVN, "vi-VN"),
            (CclLocale::zkCN, "zh-CN"),
            (CclLocale::zhTW, "zh-TW"),
        ];

        for (locale, expected_str) in locales {
            assert_eq!(locale.as_str(), expected_str);
            assert_eq!(locale.as_ref(), expected_str);

            let serialized = serde_json::to_string(&locale).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: CclLocale = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized.as_str(), expected_str);
        }
    }

    #[test]
    fn locale_serialization_roundtrip() {
        let test_locales = vec![
            CclLocale::bgBG,
            CclLocale::csCZ,
            CclLocale::daDK,
            CclLocale::deDE,
            CclLocale::elGR,
            CclLocale::enGB,
            CclLocale::enUS,
            CclLocale::esES,
            CclLocale::esMX,
            CclLocale::fiFI,
            CclLocale::frFR,
            CclLocale::huHU,
            CclLocale::itIT,
            CclLocale::jaJP,
            CclLocale::koKR,
            CclLocale::nlNL,
            CclLocale::noNO,
            CclLocale::plPL,
            CclLocale::ptBT,
            CclLocale::ptPT,
            CclLocale::roRO,
            CclLocale::ruRU,
            CclLocale::skSK,
            CclLocale::svSE,
            CclLocale::thTH,
            CclLocale::trTR,
            CclLocale::viVN,
            CclLocale::zkCN,
            CclLocale::zhTW,
        ];

        for locale in test_locales {
            let original_str = locale.as_str();
            let serialized = serde_json::to_string(&locale).unwrap();
            let deserialized: CclLocale = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized.as_str(), original_str);
        }
    }
}
