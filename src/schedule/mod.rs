mod builder;
mod response;
mod types;

pub use builder::{
    CreateChannelStreamScheduleSegmentBuilder, GetChanelStreamScheduleBuilder,
    UpdateChannelStreamScheduleBuilder, UpdateChannelStreamScheduleSegmentBulider,
};
pub use response::{Schedule, ScheduleResponse};
pub use types::{Segment, Vacation};

use chrono::{DateTime, Utc};
use chrono_tz::Tz;

use crate::{
    request::{NoContent, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, ICALENDAR, ID, SCHEDULE, SEGMENT},
        BroadcasterId, SegmentId,
    },
    Client,
};

pub trait ScheduleAPI {
    /// Gets the broadcaster’s streaming schedule
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that owns the streaming schedule you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`GetChanelStreamScheduleBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     schedule::ScheduleAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_channel_stream_schedule(&BroadcasterId::from("1234"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-stream-schedule>
    fn get_channel_stream_schedule<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetChanelStreamScheduleBuilder<'a>;

    /// Gets the broadcaster’s streaming schedule as an iCalendar.
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that owns the streaming schedule you want to get.
    ///
    /// # Returns
    ///
    /// Returns a [`String`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     schedule::ScheduleAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_channel_icalendar(&BroadcasterId::from("1234"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-icalendar>
    fn get_channel_icalendar(&self, broadcaster_id: &BroadcasterId) -> TwitchAPIRequest<String>;

    /// Updates the broadcaster’s schedule settings, such as scheduling a vacation.
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster whose schedule settings you want to update.
    ///
    /// # Returns
    ///
    /// Returns a [`UpdateChannelStreamScheduleBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     schedule::ScheduleAPI,
    ///     types::BroadcasterId,
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .update_channel_stream_schedule(&BroadcasterId::from("1234"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule>
    fn update_channel_stream_schedule<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> UpdateChannelStreamScheduleBuilder<'a>;

    /// Adds a single or recurring broadcast to the broadcaster’s streaming schedule.
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that owns the schedule to add the broadcast segment to.
    /// * `start_time` - The date and time that the broadcast segment starts.
    /// * `timezone` - The date and time that the broadcast segment starts.
    /// * `duration` - The length of time. The duration must be in the range 30 through 1380 (23 hours).
    ///
    /// # Returns
    ///
    /// Returns a [`CreateChannelStreamScheduleSegmentBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     schedule::ScheduleAPI,
    ///     types::BroadcasterId,
    /// };
    /// use chrono::{DateTime, Utc};
    /// use chrono_tz::America::New_York;
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .create_channel_stream_schedule_segment(
    ///         &BroadcasterId::from("1234"),
    ///         &"2021-07-01T18:00:00Z".parse::<DateTime<Utc>>().unwrap(),
    ///         New_York,
    ///         60
    ///     )
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#create-channel-stream-schedule-segment>
    fn create_channel_stream_schedule_segment<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        start_time: &'a DateTime<Utc>,
        timezone: Tz,
        //  minutes, must be range 30, max 1380 (23 hours)
        duration: u16,
    ) -> CreateChannelStreamScheduleSegmentBuilder<'a>;

    /// Updates a scheduled broadcast segment
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster who owns the broadcast segment to update.
    /// * `id` - The ID of the broadcast segment to update.
    ///
    /// # Returns
    ///
    /// Returns a [`UpdateChannelStreamScheduleSegmentBulider`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     schedule::ScheduleAPI,
    ///     types::{BroadcasterId, SegmentId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .update_channel_stream_schedule_segment(
    ///         &BroadcasterId::from("1234"),
    ///         &SegmentId::from("5678")
    ///     )
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule-segment>
    fn update_channel_stream_schedule_segment<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        id: &'a SegmentId,
    ) -> UpdateChannelStreamScheduleSegmentBulider<'a>;

    /// Removes a broadcast segment from the broadcaster’s streaming schedule
    ///
    /// # Arguments
    ///
    /// * `broadcaster_id` - The ID of the broadcaster that owns the streaming schedule.
    /// * `id` - The ID of the broadcast segment to remove.
    ///
    /// # Returns
    ///
    /// Returns a [`NoContent`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     schedule::ScheduleAPI,
    ///     types::{BroadcasterId, SegmentId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .delete_channel_stream_schedule_segment(
    ///         &BroadcasterId::from("1234"),
    ///         &SegmentId::from("5678")
    ///     )
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#delete-channel-stream-schedule-segment>
    fn delete_channel_stream_schedule_segment(
        &self,
        broadcaster_id: &BroadcasterId,
        id: &SegmentId,
    ) -> TwitchAPIRequest<NoContent>;
}

impl ScheduleAPI for Client {
    fn get_channel_stream_schedule<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> GetChanelStreamScheduleBuilder<'a> {
        GetChanelStreamScheduleBuilder::new(self, broadcaster_id)
    }
    simple_endpoint!(
        fn get_channel_icalendar(
            broadcaster_id: &BroadcasterId [key = BROADCASTER_ID]
        ) -> String;
            endpoint: GetChanneliCalendar,
            method: GET,
            path: [SCHEDULE, ICALENDAR],
    );
    fn update_channel_stream_schedule<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
    ) -> UpdateChannelStreamScheduleBuilder<'a> {
        UpdateChannelStreamScheduleBuilder::new(self, broadcaster_id)
    }
    fn create_channel_stream_schedule_segment<'a>(
        &'a self,
        broadcaster_id: &'a BroadcasterId,
        start_time: &'a DateTime<Utc>,
        timezone: Tz,
        duration: u16,
    ) -> CreateChannelStreamScheduleSegmentBuilder<'a> {
        CreateChannelStreamScheduleSegmentBuilder::new(
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
    ) -> UpdateChannelStreamScheduleSegmentBulider<'a> {
        UpdateChannelStreamScheduleSegmentBulider::new(self, broadcaster_id, id)
    }
    simple_endpoint!(
        fn delete_channel_stream_schedule_segment(
            broadcaster_id: &BroadcasterId [key = BROADCASTER_ID],
            id: &SegmentId [key = ID]
        ) -> NoContent;
            endpoint: DeleteChannelStreamScheduleSegment,
            method: DELETE,
            path: [SCHEDULE, SEGMENT],
    );
}
