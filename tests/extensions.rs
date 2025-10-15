#![cfg(feature = "extensions")]

#[macro_use]
mod common;

use twitch_highway::{
    extensions::{ExtensionsAPI, Segment},
    types::{BroadcasterId, Cost, CostType, ExtensionId, JWTToken},
};

api_test!(build
    get_extension_configuration_segment |api| {
        api.get_extension_configuration_segment(
            JWTToken::from("test_jwt_token"),
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            &[Segment::Global],
        )
    }
);
api_test!(build
    set_extension_configuration_segment |api| {
        api.set_extension_configuration_segment(
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            Segment::Global,
        )
        .version("0.0.1")
        .content("hello config!")
    }
);
api_test!(
    set_extension_required_configuration,
    [
        &BroadcasterId::from("274637212"),
        &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
        "0.0.1",
        "RCS",
    ]
);
api_test!(
    send_extension_pubsub_message,
    [
        &["broadcast"],
        "hello world!",
        &BroadcasterId::from("141981764"),
        None
    ]
);
api_test!(build
    get_extension_live_channels |api| {
        api.get_extension_live_channels(&ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"))
    }
);
api_test!(
    get_extension_secrets,
    [&ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2")]
);
api_test!(
    create_extension_secret,
    [
        &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
        Some(600)
    ]
);
api_test!(
    send_extension_chat_message,
    [
        &BroadcasterId::from("237757755"),
        "Hello",
        &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
        "0.0.9",
    ]
);
api_test!(
    get_extensions,
    [
        &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
        Some("0.0.9")
    ]
);
api_test!(
    get_released_extensions,
    [
        &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
        Some("0.0.9")
    ]
);
api_test!(get_extension_bits_products, [Some(true)]);
api_test!(build
    update_extension_bits_product |api| {
        api.update_extension_bits_product("1010", Cost::new(990, CostType::Bits), "Rusty Crate 2")
            .in_development(true)
            .is_broadcast(true)
            .expiration("2021-05-18T09:10:13.397Z")
    }
);

api_test!(build_extra
    get_extension_configuration_segment,
    get_extension_configuration_segment2 |api| {
        api.get_extension_configuration_segment(
            JWTToken::from("test_jwt_token"),
            &ExtensionId::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            &[Segment::Global],
        )
    }
);
