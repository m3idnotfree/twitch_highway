mod request;
mod response;
mod types;

pub use request::{
    ChannelStreamScheduleRequest, CreateScheduleSegmentRequest, UpdateScheduleRequest,
    UpdateScheduleSegmentRequest,
};
pub use response::{Schedule, ScheduleResponse};
pub use types::{Segment, Vacation};

use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;

use crate::{
    request::{NoContent, RequestBody},
    types::{
        constants::{BROADCASTER_ID, ID, SETTINGS},
        BroadcasterId, Id, PaginationQuery,
    },
};

endpoints! {
    ScheduleAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-stream-schedule>
        fn get_channel_stream_schedule(
            &self,
            broadcaster_id: &BroadcasterId,
            opts: Option<ChannelStreamScheduleRequest>,
            pagination: Option<PaginationQuery>,
        ) -> ScheduleResponse {
            endpoint_type: GetChannelStreamSchedule,
            method: GET,
            path: ["schedule"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt_into_query(opts),
                pagination(pagination)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-channel-icalendar>
        fn get_channel_icalendar(
            &self,
            broadcaster_id: &BroadcasterId,
        ) -> String {
            endpoint_type: GetChanneliCalendar,
            method: GET,
            path: ["schedule", "icalendar"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule>
        fn update_channel_stream_schedule(
            &self,
            broadcaster_id: &BroadcasterId,
            opts: Option<UpdateScheduleRequest>,
        ) -> NoContent {
            endpoint_type: UpdateChannelStreamSchedule,
            method: PATCH,
            path: ["schedule", SETTINGS],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                opt_into_query(opts)
            }
        }
        /// <https://dev.twitch.tv/docs/api/reference/#create-channel-stream-schedule-segment>
        fn create_channel_stream_schedule_segment<T: TimeZone>(
            &self,
            broadcaster_id: &BroadcasterId,
            start_time: &DateTime<T>,
            timezone: Tz,
            duration: &str,
            opts: Option<CreateScheduleSegmentRequest>,
        ) -> ScheduleResponse {
            endpoint_type: CreateChannelStreamScheduleSegment,
            method: POST,
            path: ["schedule", "segment"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
            },
            headers: [json],
            body: {
                RequestBody::new(serde_json::json!({
                    "start_time": &start_time.to_rfc3339(),
                    "timezone": timezone.name(),
                    "duration": duration,
                }), opts).into_json()
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#update-channel-stream-schedule-segment>
        fn update_channel_stream_schedule_segment(
            &self,
            broadcaster_id: &BroadcasterId,
            id: &Id,
            opts: Option<UpdateScheduleSegmentRequest>,
        ) -> ScheduleResponse {
            endpoint_type: UpdateChannelStreamScheduleSegment,
            method: PATCH,
            path: ["schedule", "segment"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(ID, id)
            },
            headers: [json],
            body: opts.and_then(|o|o.into_json())
        }

        /// <https://dev.twitch.tv/docs/api/reference/#delete-channel-stream-schedule-segment>
        fn delete_channel_stream_schedule_segment(
            &self,
            broadcaster_id: &BroadcasterId,
            id: &Id,
        ) -> NoContent {
            endpoint_type: DeleteChannelStreamScheduleSegment,
            method: DELETE,
            path: ["schedule", "segment"],
            query_params: {
                query(BROADCASTER_ID, broadcaster_id),
                query(ID, id)
            }
        }
    }
}
