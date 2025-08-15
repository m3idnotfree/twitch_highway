use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum CclsLocale {
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

impl CclsLocale {
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

impl AsRef<str> for CclsLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::ccls::request::CclsLocale;

    #[test]
    fn ccls_locale_enum_all_variants() {
        let locales = vec![
            (CclsLocale::bgBG, "bg-BG"),
            (CclsLocale::csCZ, "cs-CZ"),
            (CclsLocale::daDK, "da-DK"),
            (CclsLocale::deDE, "de-DE"),
            (CclsLocale::elGR, "el-GR"),
            (CclsLocale::enGB, "en-GB"),
            (CclsLocale::enUS, "en-US"),
            (CclsLocale::esES, "es-ES"),
            (CclsLocale::esMX, "es-MX"),
            (CclsLocale::fiFI, "fi-FI"),
            (CclsLocale::frFR, "fr-FR"),
            (CclsLocale::huHU, "hu-HU"),
            (CclsLocale::itIT, "it-IT"),
            (CclsLocale::jaJP, "ja-JP"),
            (CclsLocale::koKR, "ko-KR"),
            (CclsLocale::nlNL, "nl-NL"),
            (CclsLocale::noNO, "no-NO"),
            (CclsLocale::plPL, "pl-PL"),
            (CclsLocale::ptBT, "pt-BT"),
            (CclsLocale::ptPT, "pt-PT"),
            (CclsLocale::roRO, "ro-RO"),
            (CclsLocale::ruRU, "ru-RU"),
            (CclsLocale::skSK, "sk-SK"),
            (CclsLocale::svSE, "sv-SE"),
            (CclsLocale::thTH, "th-TH"),
            (CclsLocale::trTR, "tr-TR"),
            (CclsLocale::viVN, "vi-VN"),
            (CclsLocale::zkCN, "zh-CN"),
            (CclsLocale::zhTW, "zh-TW"),
        ];

        for (locale, expected_str) in locales {
            assert_eq!(locale.as_str(), expected_str);
            assert_eq!(locale.as_ref(), expected_str);

            let serialized = serde_json::to_string(&locale).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: CclsLocale = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized.as_str(), expected_str);
        }
    }

    #[test]
    fn locale_serialization_roundtrip() {
        let test_locales = vec![
            CclsLocale::bgBG,
            CclsLocale::csCZ,
            CclsLocale::daDK,
            CclsLocale::deDE,
            CclsLocale::elGR,
            CclsLocale::enGB,
            CclsLocale::enUS,
            CclsLocale::esES,
            CclsLocale::esMX,
            CclsLocale::fiFI,
            CclsLocale::frFR,
            CclsLocale::huHU,
            CclsLocale::itIT,
            CclsLocale::jaJP,
            CclsLocale::koKR,
            CclsLocale::nlNL,
            CclsLocale::noNO,
            CclsLocale::plPL,
            CclsLocale::ptBT,
            CclsLocale::ptPT,
            CclsLocale::roRO,
            CclsLocale::ruRU,
            CclsLocale::skSK,
            CclsLocale::svSE,
            CclsLocale::thTH,
            CclsLocale::trTR,
            CclsLocale::viVN,
            CclsLocale::zkCN,
            CclsLocale::zhTW,
        ];

        for locale in test_locales {
            let original_str = locale.as_str();
            let serialized = serde_json::to_string(&locale).unwrap();
            let deserialized: CclsLocale = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized.as_str(), original_str);
        }
    }
}
