fn_expected_request!(
    modules: [
        twitch_highway::schedule::ScheduleAPI,
        twitch_highway::types::BroadcasterId
    ],
    endpoint: get_channel_stream_schedule,
    token_type: Any,
    scopes: None,
    args: [BroadcasterId::new("141981764"), None, None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/schedule?broadcaster_id=141981764"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": {\n    \"segments\": [\n      {\n        \"id\": \"eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=\",\n        \"start_time\": \"2021-07-01T18:00:00Z\",\n        \"end_time\": \"2021-07-01T19:00:00Z\",\n        \"title\": \"TwitchDev Monthly Update // July 1, 2021\",\n        \"canceled_until\": null,\n        \"category\": {\n            \"id\": \"509670\",\n            \"name\": \"Science & Technology\"\n        },\n        \"is_recurring\": false\n      }\n    ],\n    \"broadcaster_id\": \"141981764\",\n    \"broadcaster_name\": \"TwitchDev\",\n    \"broadcaster_login\": \"twitchdev\",\n    \"vacation\": null\n  },\n  \"pagination\": {}\n}",
    module: twitch_highway::schedule::response::ScheduleResponse,
    de: ScheduleResponse
);

fn_expected_resopnse!(
    name: mock_server_response,
    payload: "{\n\t\"data\": {\n\t\t\"segments\": [\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDEtMjIgMjE6MDc6NDcuMTQzNTk3ICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-01-22T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-01-22T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDEtMjkgMjE6MDc6NDcuMTQ0MzU4ICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-01-29T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-01-29T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDItMDUgMjE6MDc6NDcuMTQ1MjU4ICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-02-05T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-02-05T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDItMTIgMjE6MDc6NDcuMTQ1OTY1ICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-02-12T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-02-12T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDItMTkgMjE6MDc6NDcuMTQ2NzE2ICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-02-19T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-02-19T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDItMjYgMjE6MDc6NDcuMTQ3NDUzICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-02-26T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-02-26T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDMtMDUgMjE6MDc6NDcuMTQ4MjE1ICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-03-05T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-03-05T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDMtMTIgMjE6MDc6NDcuMTQ4OTYzICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-03-12T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-03-12T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDMtMTkgMjE6MDc6NDcuMTQ5ODE0ICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-03-19T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-03-19T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDMtMjYgMjE6MDc6NDcuMTUwNTA4ICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-03-26T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-03-26T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDQtMDIgMjE6MDc6NDcuMTUxMTgxICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-04-02T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-04-02T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDQtMDkgMjE6MDc6NDcuMTUxODggKzAwMDAgVVRD\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-04-09T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-04-09T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDQtMTYgMjE6MDc6NDcuMTUyNjQgKzAwMDAgVVRD\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-04-16T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-04-16T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDQtMjMgMjE6MDc6NDcuMTUzNDE2ICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-04-23T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-04-23T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDQtMzAgMjE6MDc6NDcuMTU0MDQxICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-04-30T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-04-30T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDUtMDcgMjE6MDc6NDcuMTU0Njc1ICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-05-07T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-05-07T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDUtMTQgMjE6MDc6NDcuMTU1MzA3ICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-05-14T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-05-14T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDUtMjEgMjE6MDc6NDcuMTU1OTUzICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-05-21T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-05-21T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDUtMjggMjE6MDc6NDcuMTU2NzIxICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-05-28T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-05-28T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"id\": \"OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDYtMDQgMjE6MDc6NDcuMTU3MzQ1ICswMDAwIFVUQw\",\n\t\t\t\t\"title\": \"Test Title\",\n\t\t\t\t\"start_time\": \"2025-06-04T21:07:47Z\",\n\t\t\t\t\"end_time\": \"2025-06-04T21:07:47Z\",\n\t\t\t\t\"is_recurring\": true,\n\t\t\t\t\"category\": { \"id\": \"33689\", \"name\": \"Development Test\" },\n\t\t\t\t\"canceled_until\": null\n\t\t\t}\n\t\t],\n\t\t\"broadcaster_id\": \"35249427\",\n\t\t\"broadcaster_login\": \"banjomarston94\",\n\t\t\"broadcaster_name\": \"BanjoMarston94\",\n\t\t\"vacation\": { \"start_time\": \"\", \"end_time\": \"\" }\n\t},\n\t\"pagination\": { \"cursor\": \"eyJvIjowLCJsIjoyMH0\" }\n}",
    module: twitch_highway::schedule::response::ScheduleResponse,
    de: ScheduleResponse
);
