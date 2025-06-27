use serde::{Deserialize, Serialize};

use crate::types::Id;

define_request!(
    #[derive(Serialize, Deserialize)]
    ChannelStreamScheduleRequest<'a> {
        opts: {
            start_time: &'a str,
            utc_offset: &'a str,
            ids: Vec<Id> => ID ; vec,
        };
        apply_to_url
    }
);

define_request!(
    #[derive( Serialize, Deserialize)]
    UpdateScheduleRequest<'a> {
        opts: {
            vacation_start_time: &'a str,
            vacation_end_time: &'a str,
            timezone: &'a str,
            is_vacation_enabled: bool ; bool,
        };
        apply_to_url
    }
);

define_request!(
    #[derive(Serialize, Deserialize)]
    CreateScheduleSegmentRequest {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            category_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_recurring: bool
        };
        into_request_body
    }
);

define_request!(
    #[derive(Default, Serialize, Deserialize)]
    UpdateScheduleSegmentRequest {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            start_time: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            duration: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            category_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            timezone: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_canceled: bool,
        };
        into_request_body
    }
);
