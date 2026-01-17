#![cfg(feature = "ccls")]

#[macro_use]
mod common;

use twitch_highway::ccls::CclsAPI;

api_test!(get_content_classification_labels, [None]);
