use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::{
    schedule::ScheduleResponse,
    types::{
        constants::{
            AFTER, BROADCASTER_ID, FIRST, ID, IS_VACATION_ENABLED, SCHEDULE, SEGMENT, SETTINGS,
            START_TIME, TIMEZONE, VACATION_END_TIME, VACATION_START_TIME,
        },
        BroadcasterId, CategoryId, SegmentId,
    },
    Client, Error,
};

#[derive(Debug)]
pub struct GetChanelStreamScheduleBuilder<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    start_time: Option<&'a DateTime<Utc>>,
    ids: Option<&'a [&'a SegmentId]>,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetChanelStreamScheduleBuilder<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            start_time: None,
            ids: None,
            first: None,
            after: None,
        }
    }

    pub fn start_time(mut self, value: &'a DateTime<Utc>) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn ids(mut self, value: &'a [&'a SegmentId]) -> Self {
        self.ids = Some(value);
        self
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<ScheduleResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(SCHEDULE);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);
        if let Some(val) = self.start_time {
            url.query_pairs_mut().append_pair(
                START_TIME,
                &val.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            );
        }
        if let Some(ids) = self.ids {
            url.query_pairs_mut()
                .extend_pairs(ids.iter().map(|id| (ID, id)));
        }
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug)]
pub struct UpdateChannelStreamScheduleBuilder<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    vacation_start_time: Option<&'a DateTime<Utc>>,
    vacation_end_time: Option<&'a DateTime<Utc>>,
    timezone: Option<Tz>,
    is_vacation_enabled: Option<bool>,
}

impl<'a> UpdateChannelStreamScheduleBuilder<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            vacation_start_time: None,
            vacation_end_time: None,
            timezone: None,
            is_vacation_enabled: None,
        }
    }

    pub fn vacation_start_time(mut self, value: &'a DateTime<Utc>) -> Self {
        self.vacation_start_time = Some(value);
        self
    }

    pub fn vacation_end_time(mut self, value: &'a DateTime<Utc>) -> Self {
        self.vacation_end_time = Some(value);
        self
    }

    pub fn timezone(mut self, value: Tz) -> Self {
        self.timezone = Some(value);
        self
    }

    pub fn is_vacation_enabled(mut self, value: bool) -> Self {
        self.is_vacation_enabled = Some(value);
        self
    }

    pub async fn send(self) -> Result<(), Error> {
        let mut url = self.client.base_url();
        url.path_segments_mut()
            .unwrap()
            .extend([SCHEDULE, SETTINGS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);
        if let Some(val) = self.vacation_start_time {
            url.query_pairs_mut().append_pair(
                VACATION_START_TIME,
                &val.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            );
        }
        if let Some(val) = self.vacation_end_time {
            url.query_pairs_mut().append_pair(
                VACATION_END_TIME,
                &val.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            );
        }
        if let Some(val) = self.timezone {
            url.query_pairs_mut().append_pair(TIMEZONE, val.name());
        }
        if let Some(val) = self.is_vacation_enabled {
            url.query_pairs_mut()
                .append_pair(IS_VACATION_ENABLED, &val.to_string());
        }

        let req = self.client.http_client().patch(url);
        self.client.no_content(req).await
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct CreateChannelStreamScheduleSegmentBuilder<'a> {
    #[serde(skip)]
    client: &'a Client,
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
        client: &'a Client,
        broadcaster_id: &'a BroadcasterId,
        start_time: &'a DateTime<Utc>,
        timezone: Tz,
        duration: u16,
    ) -> Self {
        Self {
            client,
            broadcaster_id,
            start_time: start_time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            timezone: timezone.name(),
            duration,
            category_id: None,
            title: None,
            is_recurring: None,
        }
    }

    pub fn category_id(mut self, value: &'a CategoryId) -> Self {
        self.category_id = Some(value);
        self
    }

    pub fn title(mut self, value: &'a str) -> Self {
        self.title = Some(value);
        self
    }

    pub fn is_recurring(mut self, value: bool) -> Self {
        self.is_recurring = Some(value);
        self
    }

    pub async fn send(self) -> Result<ScheduleResponse, Error> {
        let mut url = self.client.base_url();
        url.path_segments_mut().unwrap().extend([SCHEDULE, SEGMENT]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);

        let req = self.client.http_client().post(url).json(&self);
        self.client.json(req).await
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct UpdateChannelStreamScheduleSegmentBulider<'a> {
    #[serde(skip)]
    client: &'a Client,
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
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId, id: &'a SegmentId) -> Self {
        Self {
            client,
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

    pub fn start_time(mut self, value: &'a str) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn duration(mut self, value: u16) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn category_id(mut self, value: &'a CategoryId) -> Self {
        self.category_id = Some(value);
        self
    }

    pub fn title(mut self, value: &'a str) -> Self {
        self.title = Some(value);
        self
    }

    pub fn timezone(mut self, value: Tz) -> Self {
        self.timezone = Some(value.name());
        self
    }

    pub fn is_canceled(mut self, value: bool) -> Self {
        self.is_canceled = Some(value);
        self
    }

    pub async fn send(self) -> Result<ScheduleResponse, Error> {
        let mut url = self.client.base_url();
        url.path_segments_mut().unwrap().extend([SCHEDULE, SEGMENT]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id)
            .append_pair(ID, self.id);

        let req = self.client.http_client().patch(url).json(&self);
        self.client.json(req).await
    }
}
