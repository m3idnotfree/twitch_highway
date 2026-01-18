#![cfg(feature = "videos")]

#[macro_use]
mod common;

use twitch_highway::{types::VideoId, videos::VideosAPI};

api_test!(get_videos | api | { api.get_videos_by_ids(&[VideoId::from("335921245")]) });

api_test!(delete_videos[&[VideoId::from("1234"), VideoId::from("9876")],]);
