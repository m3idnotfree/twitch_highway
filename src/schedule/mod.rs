use asknothingx2_util::api::Method;
use request::{
    ChannelStreamScheduleRequest, CreateScheduleSegmentRequest, UpdateScheduleRequest,
    UpdateScheduleSegmentRequest,
};
use response::ScheduleResponse;
use serde_json::Value;

use crate::{
    base::TwitchAPIBase,
    request::RequestBody,
    types::{
        constants::{BROADCASTER_ID, ID, SETTINGS},
        BroadcasterId, Id, PaginationQuery,
    },
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

pub trait ScheduleAPI: TwitchAPIBase {
    fn get_channel_stream_schedule(
        &self,
        broadcaster_id: BroadcasterId,
        opts: Option<ChannelStreamScheduleRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ScheduleResponse>;
    //
    fn get_channel_icalendar(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    fn update_channel_stream_schedule(
        &self,
        broadcaster_id: BroadcasterId,
        opts: Option<UpdateScheduleRequest>,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
    fn create_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        start_time: &str,
        timezone: &str,
        duration: &str,
        opts: Option<CreateScheduleSegmentRequest>,
    ) -> TwitchAPIRequest<RequestBody<Value, CreateScheduleSegmentRequest>, ScheduleResponse>;
    fn update_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        opts: Option<UpdateScheduleSegmentRequest>,
    ) -> TwitchAPIRequest<UpdateScheduleSegmentRequest, ScheduleResponse>;
    fn delete_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody>;
}

impl ScheduleAPI for TwitchAPI {
    fn get_channel_stream_schedule(
        &self,
        broadcaster_id: BroadcasterId,
        opts: Option<ChannelStreamScheduleRequest>,
        pagination: Option<PaginationQuery>,
    ) -> TwitchAPIRequest<EmptyBody, ScheduleResponse> {
        let mut url = self.build_url();
        url.path(["schedule"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_pairs(opts)
            .query_opt_pairs(pagination);

        TwitchAPIRequest::new(
            EndpointType::GetChannelStreamSchedule,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn get_channel_icalendar(
        &self,
        broadcaster_id: BroadcasterId,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path(["schedule", "icalendar"])
            .query(BROADCASTER_ID, broadcaster_id);

        TwitchAPIRequest::new(
            EndpointType::GetChanneliCalendar,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn update_channel_stream_schedule(
        &self,
        broadcaster_id: BroadcasterId,
        opts: Option<UpdateScheduleRequest>,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path(["schedule", SETTINGS])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_opt_pairs(opts);

        TwitchAPIRequest::new(
            EndpointType::UpdateChannelStreamSchedule,
            url.build(),
            Method::PATCH,
            self.build_headers().build(),
            EmptyBody,
        )
    }
    fn create_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        start_time: &str,
        timezone: &str,
        duration: &str,
        opts: Option<CreateScheduleSegmentRequest>,
    ) -> TwitchAPIRequest<RequestBody<Value, CreateScheduleSegmentRequest>, ScheduleResponse> {
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
            request_body,
        )
    }
    fn update_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        request: Option<UpdateScheduleSegmentRequest>,
    ) -> TwitchAPIRequest<UpdateScheduleSegmentRequest, ScheduleResponse> {
        let mut url = self.build_url();
        url.path(["schedule", "segment"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(ID, id);

        let mut headers = self.build_headers();
        headers.json();
        TwitchAPIRequest::new(
            EndpointType::UpdateChannelStreamScheduleSegment,
            url.build(),
            Method::PATCH,
            headers.build(),
            request.unwrap_or_default(),
        )
    }
    fn delete_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
    ) -> TwitchAPIRequest<EmptyBody, EmptyBody> {
        let mut url = self.build_url();
        url.path(["schedule", "segment"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query(ID, id);

        TwitchAPIRequest::new(
            EndpointType::DeleteChannelStreamScheduleSegment,
            url.build(),
            Method::DELETE,
            self.build_headers().build(),
            EmptyBody,
        )
    }
}
