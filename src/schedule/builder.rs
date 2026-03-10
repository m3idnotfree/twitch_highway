use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::{
    request::{NoContent, TwitchAPIRequest},
    schedule::ScheduleResponse,
    types::{
        constants::{
            AFTER, BROADCASTER_ID, FIRST, ID, IS_VACATION_ENABLED, SCHEDULE, SEGMENT, SETTINGS,
            START_TIME, TIMEZONE, VACATION_END_TIME, VACATION_START_TIME,
        },
        BroadcasterId, CategoryId, SegmentId,
    },
    Client,
};

define_request_builder! {
    #[derive(Debug)]
    GetChanelStreamScheduleBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]},
        opts: {
            start_time: &'a DateTime<Utc> [key = START_TIME, convert = rfc3339_opt],
            // Not supported
            // utc_offset: &'a str,
            ids: &'a [&'a SegmentId] [key = ID, convert = extend],
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER]

        }
    } -> ScheduleResponse;
        endpoint: GetChannelStreamSchedule,
        method: GET,
        path: [SCHEDULE],
}

define_request_builder! {
    #[derive(Debug)]
    UpdateChannelStreamScheduleBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId [key = BROADCASTER_ID]},
        opts: {
            vacation_start_time: &'a DateTime<Utc> [key = VACATION_START_TIME, convert = rfc3339_opt],
            vacation_end_time: &'a DateTime<Utc> [key = VACATION_END_TIME, convert = rfc3339_opt],
            timezone: Tz [key = TIMEZONE, convert = timezone],
            is_vacation_enabled: bool [key = IS_VACATION_ENABLED, convert = to_string]
        }
    } -> NoContent;
    endpoint: UpdateChannelStreamSchedule,
    method: PATCH,
    path: [SCHEDULE, SETTINGS],
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct CreateChannelStreamScheduleSegmentBuilder<'a> {
    #[serde(skip)]
    api: &'a Client,
    #[serde(skip)]
    broadcaster_id: &'a BroadcasterId,
    start_time: String,
    timezone: &'a str,
    #[serde_as(as = "DisplayFromStr")]
    duration: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    category_id: Option<&'a CategoryId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_recurring: Option<bool>,
}

impl<'a> CreateChannelStreamScheduleSegmentBuilder<'a> {
    pub fn new(
        api: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        start_time: &'a DateTime<Utc>,
        timezone: Tz,
        duration: u16,
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

    opt_method!(category_id, &'a CategoryId);
    opt_method!(title, &'a str);
    opt_method!(is_recurring, bool);

    pub fn build(self) -> TwitchAPIRequest<ScheduleResponse> {
        let mut url = self.api.base_url();

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
            self.api.http_client().clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<ScheduleResponse, crate::Error> {
        self.build().json().await
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct UpdateChannelStreamScheduleSegmentBulider<'a> {
    #[serde(skip)]
    api: &'a Client,
    #[serde(skip)]
    broadcaster_id: &'a BroadcasterId,
    #[serde(skip)]
    id: &'a SegmentId,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<&'a str>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    duration: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category_id: Option<&'a CategoryId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_canceled: Option<bool>,
}

impl<'a> UpdateChannelStreamScheduleSegmentBulider<'a> {
    pub fn new(api: &'a Client, broadcaster_id: &'a BroadcasterId, id: &'a SegmentId) -> Self {
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
    opt_method!(duration, u16);
    opt_method!(category_id, &'a CategoryId);
    opt_method!(title, &'a str);
    opt_method!(timezone, Tz[timezone]);
    opt_method!(is_canceled, bool);

    pub fn build(self) -> TwitchAPIRequest<ScheduleResponse> {
        let mut url = self.api.base_url();

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
            self.api.http_client().clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<ScheduleResponse, crate::Error> {
        self.build().json().await
    }
}
