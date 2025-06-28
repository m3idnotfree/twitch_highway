use asknothingx2_util::api::Method;
use request::{
    ChannelStreamScheduleRequest, CreateScheduleSegmentRequest, UpdateScheduleRequest,
    UpdateScheduleSegmentRequest,
};
use response::ScheduleResponse;

use crate::{
    request::{EmptyBody, EndpointType, RequestBody, TwitchAPIRequest},
    types::{
        constants::{BROADCASTER_ID, ID, SETTINGS},
        BroadcasterId, Id, PaginationQuery,
    },
    TwitchAPI,
};

pub mod request;
pub mod response;
pub mod types;

#[cfg_attr(docsrs, doc(cfg(feature = "schedule")))]
pub trait ScheduleAPI {
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-stream-schedule>
    fn get_channel_stream_schedule(
        &self,
        broadcaster_id: BroadcasterId,
        opts: Option<ChannelStreamScheduleRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ScheduleResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#get-channel-icalendar>
    fn get_channel_icalendar(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule>
    fn update_channel_stream_schedule(
        &self,
        broadcaster_id: BroadcasterId,
        opts: Option<UpdateScheduleRequest>,
    ) -> TwitchAPIRequest<EmptyBody>;
    /// <https://dev.twitch.tv/docs/api/reference/#create-channel-stream-schedule-segment>
    fn create_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        start_time: &str,
        timezone: &str,
        duration: &str,
        opts: Option<CreateScheduleSegmentRequest>,
    ) -> TwitchAPIRequest<ScheduleResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule-segment>
    fn update_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        opts: Option<UpdateScheduleSegmentRequest>,
    ) -> TwitchAPIRequest<ScheduleResponse>;
    /// <https://dev.twitch.tv/docs/api/reference/#delete-channel-stream-schedule-segment>
    fn delete_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
    ) -> TwitchAPIRequest<EmptyBody>;
}

impl ScheduleAPI for TwitchAPI {
    fn get_channel_stream_schedule(
        &self,
        broadcaster_id: BroadcasterId,
        opts: Option<ChannelStreamScheduleRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<ScheduleResponse> {
        let mut url = self.build_url();
        url.path(["schedule"]).query(BROADCASTER_ID, broadcaster_id);

        if let Some(opts) = opts {
            opts.apply_to_url(&mut url);
        }

        if let Some(pagination) = pagination {
            pagination.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::GetChannelStreamSchedule,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn get_channel_icalendar(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["schedule", "icalendar"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetChanneliCalendar,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            None,
        )
    }
    fn update_channel_stream_schedule(
        &self,
        broadcaster_id: BroadcasterId,
        opts: Option<UpdateScheduleRequest>,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["schedule", SETTINGS])
            .query(BROADCASTER_ID, broadcaster_id);

        if let Some(opts) = opts {
            opts.apply_to_url(&mut url);
        }

        TwitchAPIRequest::new(
            EndpointType::UpdateChannelStreamSchedule,
            url.build(),
            Method::PATCH,
            self.build_headers().build(),
            None,
        )
    }
    fn create_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        start_time: &str,
        timezone: &str,
        duration: &str,
        opts: Option<CreateScheduleSegmentRequest>,
    ) -> TwitchAPIRequest<ScheduleResponse> {
        let mut url = self.build_url();
        url.path(["schedule", "segment"])
            .query(BROADCASTER_ID, broadcaster_id);

        let required = serde_json::json!({
            "start_time": start_time,
            "timezone": timezone,
            "duration": duration,
        });

        let mut headers = self.build_headers();
        headers.json();

        let request_body = RequestBody::new(required, opts);

        TwitchAPIRequest::new(
            EndpointType::CreateChannelStreamScheduleSegment,
            url.build(),
            Method::POST,
            headers.build(),
            request_body.to_json(),
        )
    }
    fn update_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        request: Option<UpdateScheduleSegmentRequest>,
    ) -> TwitchAPIRequest<ScheduleResponse> {
        let mut url = self.build_url();
        url.path(["schedule", "segment"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(ID, id);

        let mut headers = self.build_headers();
        headers.json();

        let request = if let Some(request) = request {
            request.to_json()
        } else {
            None
        };

        TwitchAPIRequest::new(
            EndpointType::UpdateChannelStreamScheduleSegment,
            url.build(),
            Method::PATCH,
            headers.build(),
            request,
        )
    }
    fn delete_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["schedule", "segment"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(ID, id);

        TwitchAPIRequest::new(
            EndpointType::DeleteChannelStreamScheduleSegment,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            None,
        )
    }
}
