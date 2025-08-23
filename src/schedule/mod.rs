pub mod request;
pub mod response;
pub mod types;

use request::{
    ChannelStreamScheduleRequest, CreateScheduleSegmentRequest, UpdateScheduleRequest,
    UpdateScheduleSegmentRequest,
};
use response::ScheduleResponse;

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
        fn create_channel_stream_schedule_segment(
            &self,
            broadcaster_id: &BroadcasterId,
            start_time: &str,
            timezone: &str,
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
                    "start_time": start_time,
                    "timezone": timezone,
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

#[cfg(test)]
mod tests {
    use crate::{
        schedule::{
            request::{
                CreateScheduleSegmentRequest, UpdateScheduleRequest, UpdateScheduleSegmentRequest,
            },
            ScheduleAPI,
        },
        types::{BroadcasterId, Id},
    };

    api_test!(
        get_channel_stream_schedule,
        [&BroadcasterId::from("141981764"), None, None]
    );
    api_test!(send get_channel_icalendar, [&BroadcasterId::from("141981764")]);
    api_test!(
        update_channel_stream_schedule,
        [
            &BroadcasterId::from("141981764"),
            Some(
                UpdateScheduleRequest::new()
                    .is_vacation_enabled(true)
                    .vacation_start_time("2021-05-16T00:00:00Z")
                    .vacation_end_time("2021-05-23T00:00:00Z")
                    .timezone("America/New_York")
            )
        ]
    );
    api_test!(
        create_channel_stream_schedule_segment,
        [
            &BroadcasterId::from("141981764"),
            "2021-07-01T18:00:00Z",
            "America/New_York",
            "60",
            Some(
                CreateScheduleSegmentRequest::new()
                    .is_recurring(false)
                    .category_id("509670")
                    .title("TwitchDev Monthly Update // July 1, 2021")
            )
        ]
    );
    api_test!(
        update_channel_stream_schedule_segment,
        [
            &BroadcasterId::from("141981764"),
            &Id::from("eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0="),
            Some(
                UpdateScheduleSegmentRequest::new()
                    .duration("120")
            )
        ]
    );
    api_test!(
        delete_channel_stream_schedule_segment,
        [&BroadcasterId::from("141981764"), &Id::from("eyJzZWdtZW50SUQiOiI4Y2EwN2E2NC0xYTZkLTRjYWItYWE5Ni0xNjIyYzNjYWUzZDkiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyMX0=")]
    );
}
