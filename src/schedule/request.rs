use serde::{Deserialize, Serialize};

use crate::types::Id;

define_request!(
    #[derive(Serialize)]
    ChannelStreamScheduleRequest<'a> {
        opts: {
            start_time: &'a str,
            utc_offset: &'a str,
            ids: &'a [Id] => ID ; vec,
        };
        into_query
    }
);

define_request!(
    #[derive(Serialize, Deserialize)]
    UpdateScheduleRequest<'a> {
        opts: {
            vacation_start_time: &'a str,
            vacation_end_time: &'a str,
            timezone: &'a str,
            is_vacation_enabled: bool ; bool,
        };
        into_query
    }
);

define_request!(
    #[derive(Serialize, Deserialize)]
    CreateScheduleSegmentRequest<'a> {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            category_id: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_recurring: bool
        };
        into_json
    }
);

define_request!(
    #[derive(Default, Serialize, Deserialize)]
    UpdateScheduleSegmentRequest<'a> {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            start_time: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            duration: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            category_id: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            timezone: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_canceled: bool,
        };
        into_json
    }
);
