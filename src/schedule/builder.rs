use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::Serialize;

use crate::{
    request::{NoContent, TwitchAPIRequest},
    schedule::ScheduleResponse,
    types::{
        constants::{
            AFTER, BROADCASTER_ID, FIRST, ID, IS_VACATION_ENABLED, SCHEDULE, SEGMENT, SETTINGS,
            START_TIME, TIMEZONE, VACATION_END_TIME, VACATION_START_TIME,
        },
        BroadcasterId, Id,
    },
    TwitchAPI,
};

define_request_builder! {
    #[derive(Debug)]
    GetChanelStreamScheduleBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]},
        opts: {
            start_time: &'a DateTime<Utc> [key = START_TIME, convert = rfc3339],
            // Not supported
            // utc_offset: &'a str,
            ids: &'a [Id] [key = ID, convert = extend],
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER]

        }
    } -> ScheduleResponse;
            endpoint_type: GetChannelStreamSchedule,
            method: GET,
            path: [SCHEDULE],

}

define_request_builder! {
    #[derive(Debug)]
    UpdateChannelStreamScheduleBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]},
        opts: {
            vacation_start_time: &'a str [key = VACATION_START_TIME],
            vacation_end_time: &'a str [key = VACATION_END_TIME],
            timezone: &'a str [key = TIMEZONE],
            is_vacation_enabled: bool [key = IS_VACATION_ENABLED, convert = to_string]
        }
    } -> NoContent;
    endpoint_type: UpdateChannelStreamSchedule,
    method: PATCH,
    path: [SCHEDULE, SETTINGS],
}

#[derive(Debug, Serialize)]
pub struct CreateChannelStreamScheduleSegmentBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
    #[serde(skip)]
    broadcaster_id: &'a BroadcasterId,
    start_time: String,
    timezone: &'a str,
    duration: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    category_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_recurring: Option<bool>,
}

impl<'a> CreateChannelStreamScheduleSegmentBuilder<'a> {
    pub fn new(
        api: &'a TwitchAPI,
        broadcaster_id: &'a BroadcasterId,
        start_time: &'a DateTime<Utc>,
        timezone: Tz,
        duration: &'a str,
    ) -> Self {
        Self {
            api,
            broadcaster_id,
            start_time: start_time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            timezone: timezone.name(),
            duration,
            category_id: None,
            title: None,
            is_recurring: None,
        }
    }

    opt_method!(category_id, &'a str);
    opt_method!(title, &'a str);
    opt_method!(is_recurring, bool);

    pub fn build(self) -> TwitchAPIRequest<ScheduleResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[SCHEDULE, SEGMENT]);

        let mut query = url.query_pairs_mut();

        query.append_pair(BROADCASTER_ID, self.broadcaster_id);

        let body = serde_json::to_string(&self).ok();

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::CreateChannelStreamScheduleSegment,
            url,
            reqwest::Method::POST,
            self.api.header_json(),
            body,
            self.api.client.clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<ScheduleResponse, crate::Error> {
        self.build().json().await
    }
}

#[derive(Debug, Serialize)]
pub struct UpdateChannelStreamScheduleSegmentBulider<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
    #[serde(skip)]
    broadcaster_id: &'a BroadcasterId,
    #[serde(skip)]
    id: &'a Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_canceled: Option<bool>,
}

impl<'a> UpdateChannelStreamScheduleSegmentBulider<'a> {
    pub fn new(api: &'a TwitchAPI, broadcaster_id: &'a BroadcasterId, id: &'a Id) -> Self {
        Self {
            api,
            broadcaster_id,
            id,
            start_time: None,
            duration: None,
            category_id: None,
            title: None,
            timezone: None,
            is_canceled: None,
        }
    }

    opt_method!(start_time, &'a str);
    opt_method!(duration, &'a str);
    opt_method!(category_id, &'a str);
    opt_method!(title, &'a str);
    opt_method!(timezone, &'a str);
    opt_method!(is_canceled, bool);

    pub fn build(self) -> TwitchAPIRequest<ScheduleResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[SCHEDULE, SEGMENT]);

        let mut query = url.query_pairs_mut();

        query.append_pair(BROADCASTER_ID, self.broadcaster_id);
        query.append_pair(ID, self.id);

        let body = serde_json::to_string(&self).ok();

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::UpdateChannelStreamScheduleSegment,
            url,
            reqwest::Method::PATCH,
            self.api.header_json(),
            body,
            self.api.client.clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<ScheduleResponse, crate::Error> {
        self.build().json().await
    }
}
