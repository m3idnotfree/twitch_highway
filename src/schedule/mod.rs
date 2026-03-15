mod builder;
mod response;
mod types;

pub use builder::{
    CreateChannelStreamScheduleSegment, GetChannelStreamSchedule, UpdateChannelStreamSchedule,
    UpdateChannelStreamScheduleSegment,
};
pub use response::{Schedule, ScheduleResponse};
pub use types::{Segment, Vacation};

use std::future::Future;

use chrono::{DateTime, Utc};
use chrono_tz::Tz;

use crate::{
    Client, Error,
    types::{
        BroadcasterId, SegmentId,
        constants::{BROADCASTER_ID, ICALENDAR, ID, SCHEDULE, SEGMENT},
    },
};

pub trait ScheduleAPI {
    /// See <https://dev.twitch.tv/docs/api/reference/#get-channel-stream-schedule>
    fn get_channel_stream_schedule<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetChannelStreamSchedule<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#get-channel-icalendar>
    fn get_channel_icalendar(
        &self,
        broadcaster_id: &BroadcasterId,
    ) -> impl Future<Output = Result<String, Error>> + Send;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule>
    fn update_channel_stream_schedule<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> UpdateChannelStreamSchedule<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#create-channel-stream-schedule-segment>
    fn create_channel_stream_schedule_segment<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        start_time: &'a DateTime<Utc>,
        timezone: Tz,
        //  minutes, must be range 30, max 1380 (23 hours)
        duration: u16,
    ) -> CreateChannelStreamScheduleSegment<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule-segment>
    fn update_channel_stream_schedule_segment<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        id: &'a SegmentId,
    ) -> UpdateChannelStreamScheduleSegment<'a>;

    /// See <https://dev.twitch.tv/docs/api/reference/#delete-channel-stream-schedule-segment>
    fn delete_channel_stream_schedule_segment(
        &self,
        broadcaster_id: &BroadcasterId,
        id: &SegmentId,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl ScheduleAPI for Client {
    fn get_channel_stream_schedule<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetChannelStreamSchedule<'a> {
        GetChannelStreamSchedule::new(self, broadcaster_id)
    }

    async fn get_channel_icalendar(&self, broadcaster_id: &BroadcasterId) -> Result<String, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SCHEDULE, ICALENDAR]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id);

        self.text(self.http_client().get(url)).await
    }

    fn update_channel_stream_schedule<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> UpdateChannelStreamSchedule<'a> {
        UpdateChannelStreamSchedule::new(self, broadcaster_id)
    }

    fn create_channel_stream_schedule_segment<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        start_time: &'a DateTime<Utc>,
        timezone: Tz,
        duration: u16,
    ) -> CreateChannelStreamScheduleSegment<'a> {
        CreateChannelStreamScheduleSegment::new(
            self,
            broadcaster_id,
            start_time,
            timezone,
            duration,
        )
    }

    fn update_channel_stream_schedule_segment<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        id: &'a SegmentId,
    ) -> UpdateChannelStreamScheduleSegment<'a> {
        UpdateChannelStreamScheduleSegment::new(self, broadcaster_id, id)
    }

    async fn delete_channel_stream_schedule_segment(
        &self,
        broadcaster_id: &BroadcasterId,
        id: &SegmentId,
    ) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().extend([SCHEDULE, SEGMENT]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, broadcaster_id)
            .append_pair(ID, id);

        self.no_content(self.http_client().delete(url)).await
    }
}
