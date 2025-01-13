use asknothingx2_util::api::Method;
use serde::{Deserialize, Serialize};

use crate::{base::TwitchAPIBase, EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest};

pub mod response;
pub mod types;

pub trait CclsAPI: TwitchAPIBase {
    fn get_content_classification_labels(
        &self,
        locale: Option<CclsLocale>,
    ) -> TwitchAPIRequest<EmptyBody>;
}

impl CclsAPI for TwitchAPI {
    fn get_content_classification_labels(
        &self,
        locale: Option<CclsLocale>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["content_classification_labels"])
            .query_opt("locale", locale);

        TwitchAPIRequest::new(
            EndpointType::GetContentClassificationLabels,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}

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
