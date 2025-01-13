use asknothingx2_util::api::Method;
use request::{
    ChannelStreamScheduleRequest, CreateScheduleSegmentRequest, UpdateScheduleRequest,
    UpdateScheduleSegmentRequest,
};

use crate::{
    base::TwitchAPIBase,
    types::{BroadcasterId, Id, BROADCASTER_ID, ID},
    EmptyBody, EndpointType, TwitchAPI, TwitchAPIRequest,
};

pub mod request;
pub mod response;
pub mod types;

pub trait ScheduleAPI: TwitchAPIBase {
    fn get_channel_stream_schedule(
        &self,
        broadcaster_id: BroadcasterId,
        request: ChannelStreamScheduleRequest,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn get_channel_icalendar(&self, broadcaster_id: BroadcasterId) -> TwitchAPIRequest<EmptyBody>;
    fn update_channel_stream_schedule(
        &self,
        request: UpdateScheduleRequest,
    ) -> TwitchAPIRequest<EmptyBody>;
    fn create_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        request: CreateScheduleSegmentRequest,
    ) -> TwitchAPIRequest<CreateScheduleSegmentRequest>;
    fn update_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        request: UpdateScheduleSegmentRequest,
    ) -> TwitchAPIRequest<UpdateScheduleSegmentRequest>;
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
        request: ChannelStreamScheduleRequest,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["schedule"])
            .query(BROADCASTER_ID, broadcaster_id)
            .query_pairs(request);

        TwitchAPIRequest::new(
            EndpointType::GetChannelStreamSchedule,
            url.build(),
            Method::GET,
            self.build_headers().build(),
            EmptyBody,
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
            EmptyBody,
        )
    }
    fn update_channel_stream_schedule(
        &self,
        request: UpdateScheduleRequest,
    ) -> TwitchAPIRequest<EmptyBody> {
        let mut url = self.build_url();
        url.path(["schedule", "settings"]).query_pairs(request);

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
        request: CreateScheduleSegmentRequest,
    ) -> TwitchAPIRequest<CreateScheduleSegmentRequest> {
        let mut url = self.build_url();
        url.path(["schedule", "segment"])
            .query(BROADCASTER_ID, broadcaster_id);

        let mut headers = self.build_headers();
        headers.json();

        TwitchAPIRequest::new(
            EndpointType::CreateChannelStreamScheduleSegment,
            url.build(),
            Method::POST,
            headers.build(),
            request,
        )
    }
    fn update_channel_stream_schedule_segment(
        &self,
        broadcaster_id: BroadcasterId,
        id: Id,
        request: UpdateScheduleSegmentRequest,
    ) -> TwitchAPIRequest<UpdateScheduleSegmentRequest> {
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
            EmptyBody,
        )
    }
}
