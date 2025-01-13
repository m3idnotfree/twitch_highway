use serde::{Deserialize, Serialize};

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{BroadcasterId, Id, AFTER, FIRST, ID},
    RequestBody,
};

use super::BROADCASTER_ID;

request_struct!(
    #[derive(Debug, Default, Serialize, Deserialize)]
    ChannelStreamScheduleRequest {
        ids: Vec<Id>,
        start_time: String,
        utc_offset: String,
        first: u64,
        after: String,
    }
);

impl IntoQueryPairs for ChannelStreamScheduleRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();

        params
            .extend_opt(self.ids.map(|id| id.into_iter().map(|i| (ID, i))))
            .push_opt("start_time", self.start_time)
            .push_opt("utc_offset", self.utc_offset)
            .push_opt(FIRST, self.first.map(|x| x.to_string()))
            .push_opt(AFTER, self.after);

        params.build()
    }
}

request_struct!(
#[derive(Debug, Serialize, Deserialize)]
UpdateScheduleRequest {
    required {
        broadcaster_id: BroadcasterId
    },
    optional {
        is_vacation_enabled: bool,
        vacation_start_time: String,
        vacation_end_time: String,
        timezone: String
    }
}
);
impl IntoQueryPairs for UpdateScheduleRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push(BROADCASTER_ID, self.broadcaster_id)
            .push_opt(
                "is_vacation_enabled",
                self.is_vacation_enabled.map(|x| x.to_string()),
            )
            .push_opt("vacation_start_time", self.vacation_start_time)
            .push_opt("vacation_end_time", self.vacation_end_time)
            .push_opt("timezone", self.timezone);

        params.build()
    }
}

request_struct!(
#[derive(Debug, Serialize, Deserialize)]
CreateScheduleSegmentRequest {
    required {
        start_time: String,
        timezonee: String,
        duration: String
    },
    optional {
        #[serde(skip_serializing_if = "Option::is_none")]
        is_recurring: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        category_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: String,
    }
}
);

impl RequestBody for CreateScheduleSegmentRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

request_struct!(
    #[derive(Debug, Default, Serialize, Deserialize)]
    UpdateScheduleSegmentRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        start_time: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        category_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_canceled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        timezonee: String,
    }
);

impl RequestBody for UpdateScheduleSegmentRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}
