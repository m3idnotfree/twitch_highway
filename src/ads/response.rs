use serde::{Deserialize, Serialize};

use super::types::{AdSchedule, SnoozeNextAd, StartCommercial};

#[derive(Debug, Serialize, Deserialize)]
pub struct StartCommercialResponse {
    pub data: Vec<StartCommercial>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdScheduleResponse {
    pub data: Vec<AdSchedule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnoozeNextAdResponse {
    pub data: Vec<SnoozeNextAd>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::ads::response::{AdScheduleResponse, SnoozeNextAdResponse, StartCommercialResponse};

    #[test]
    fn start_commercial_response_deserialization() {
        let json_data = json!({
            "data": [{
                "length": 30,
                "message": "Commercial started successfully",
                "retry_after": 480
            }]
        });

        let response: StartCommercialResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].length, 30);
        assert_eq!(response.data[0].message, "Commercial started successfully");
        assert_eq!(response.data[0].retry_after, 480);
    }

    #[test]
    fn ad_schedule_response_deserialization() {
        let json_data = json!({
            "data": [{
                "snooze_count": "1",
                "snooze_refresh_at": "2023-12-01T15:30:00Z",
                "next_ad_at": "2023-12-01T16:00:00Z",
                "duration": "60",
                "last_ad_at": "2023-12-01T15:00:00Z",
                "preroll_free_time": "300"
            }]
        });

        let response: AdScheduleResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 1);
        let ad_schedule = &response.data[0];
        assert_eq!(ad_schedule.snooze_count, "1");
        assert_eq!(ad_schedule.duration, "60");
        assert_eq!(ad_schedule.preroll_free_time, "300");
        assert!(ad_schedule.snooze_refresh_at.is_some());
        assert!(ad_schedule.next_ad_at.is_some());
        assert!(ad_schedule.last_ad_at.is_some());
    }

    #[test]
    fn ad_schedule_with_null_values() {
        let json_data = json!({
            "data": [{
                "snooze_count": "0",
                "snooze_refresh_at": null,
                "next_ad_at": null,
                "duration": "0",
                "last_ad_at": null,
                "preroll_free_time": "0"
            }]
        });

        let response: AdScheduleResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 1);
        let ad_schedule = &response.data[0];
        assert_eq!(ad_schedule.snooze_count, "0");
        assert_eq!(ad_schedule.duration, "0");
        assert_eq!(ad_schedule.preroll_free_time, "0");
        assert!(ad_schedule.snooze_refresh_at.is_none());
        assert!(ad_schedule.next_ad_at.is_none());
        assert!(ad_schedule.last_ad_at.is_none());
    }

    #[test]
    fn snooze_next_ad_response_deserialization() {
        let json_data = json!({
            "data": [{
                "snooze_count": "2",
                "snooze_refresh_at": "2023-12-01T16:30:00Z",
                "next_ad_at": "2023-12-01T17:00:00Z"
            }]
        });

        let response: SnoozeNextAdResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.data.len(), 1);
        let snooze_data = &response.data[0];
        assert_eq!(snooze_data.snooze_count, "2");
    }
}
