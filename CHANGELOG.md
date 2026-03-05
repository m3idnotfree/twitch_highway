## [0.4.0](https://github.com/m3idnotfree/twitch_highway/compare/v0.3.3..v0.4.0) - 2026-03-05

### Features

- **(clips)** Add create clip from vod endpoint([56ca299](https://github.com/m3idnotfree/twitch_highway/commit/56ca29972aa6b8fe1b7a6339e6251ad2ae32b628))
- **(clips)** [**breaking**] Update Create Clip to match Twitch API 2025-12-19([26f9610](https://github.com/m3idnotfree/twitch_highway/commit/26f96101543a95a4e1b5a6ea0ae657ae028f1c1d))
- **(moderation)** Add `add suspicious status to chat user` endpoint([d61111c](https://github.com/m3idnotfree/twitch_highway/commit/d61111c73f9e2ca2428fab59380fbb5af7aac585))
- **(moderation)** Add `remove suspicious status from chat user` endpoint([fd26bec](https://github.com/m3idnotfree/twitch_highway/commit/fd26bec18ee90465b6dd29f3f38062af60878703))

### Refactor

- [**breaking**] Remove per-endpoint feature flags([6fd1b41](https://github.com/m3idnotfree/twitch_highway/commit/6fd1b4171636410ef7a0eb30c8a07071c5bf4f91))
- [**breaking**] Simplify webhook and websocket features([4100207](https://github.com/m3idnotfree/twitch_highway/commit/410020786e5df2bcbe85fad39e1a3cc97feb53f8))

### Documentation

- Add twitch api coverage section([23595c2](https://github.com/m3idnotfree/twitch_highway/commit/23595c25b83548de866eb065c05fb743a6dedb48))
- Update twitch api coverage to 2026-02-05([6561d5f](https://github.com/m3idnotfree/twitch_highway/commit/6561d5fd57e17477b2948876694948c9440f8cf4))

### Miscellaneous

- Add CHANGELOG.md and cliff.toml([e5b78e8](https://github.com/m3idnotfree/twitch_highway/commit/e5b78e86f77e7f00701c8cd7f4c12d99a4894554))
- Update dependencies and migrate([5e9c8c2](https://github.com/m3idnotfree/twitch_highway/commit/5e9c8c2819433dc53c0e4021ff2b55701a9d2311))

## [0.3.3](https://github.com/m3idnotfree/twitch_highway/compare/v0.3.2..v0.3.3) - 2026-01-19

### Features

- **(examples)** Add chatbot example([8ff5d87](https://github.com/m3idnotfree/twitch_highway/commit/8ff5d8753bed008d081318cf24c1200aed7901f2))

### Refactor

- **(websocket)** Improve graceful shutdown handling([9a61633](https://github.com/m3idnotfree/twitch_highway/commit/9a616338cea0ce7e970a34a143150931fd8ef801))
- **(tests)** Consolidate twitch-cli tests into twitch_cli.rs([1ae1774](https://github.com/m3idnotfree/twitch_highway/commit/1ae1774a7fd0be1916ab06ee7ceda92618b2ae52))
- **(tests)** Simplify api_test macro and migrate test files([c16b3ed](https://github.com/m3idnotfree/twitch_highway/commit/c16b3ed340ece4255c7222f3654691de688d7a7f))

### Miscellaneous

- Migrate from taplo to tombi for TOML formatting([88c049f](https://github.com/m3idnotfree/twitch_highway/commit/88c049fecf774986dd02f05194f98ca89c351100))
- **(lint)** Suppress unused warning for TwitchAPI::build_url() method([5dad891](https://github.com/m3idnotfree/twitch_highway/commit/5dad8919704d695d84a8fa3b38072a58c757b079))

## [0.3.2](https://github.com/m3idnotfree/twitch_highway/compare/v0.3.1..v0.3.2) - 2025-12-10

### Bug Fixes

- **(types)** Add conduits to pagination feature gates([99cb2a7](https://github.com/m3idnotfree/twitch_highway/commit/99cb2a76c8dd7ac13db0169e28933812e3f30775))
- **(types)** Add hype-train to BROADCASTER_ID feature gates([82cde0d](https://github.com/m3idnotfree/twitch_highway/commit/82cde0ddfa271c80b505e11cfe1c3ddaad73a5d4))
- **(types)** Add whisper to UserId feature gates([6ee8013](https://github.com/m3idnotfree/twitch_highway/commit/6ee8013ef4d1954f7bd2178d6bc54b02514abc96))

### Miscellaneous

- **(dev-deps)** Replace dotenv with dotenvy([23f64bf](https://github.com/m3idnotfree/twitch_highway/commit/23f64bfc7629f18a7013fef34ea44c64a01f78d8))
- **(lint)** Allow unused warnings([d99e939](https://github.com/m3idnotfree/twitch_highway/commit/d99e939b5ad348bb0e7e55066f4c0a95652b6321))
- **(dev-deps)** Remove unused dev-dependencies([8760af1](https://github.com/m3idnotfree/twitch_highway/commit/8760af17f80010ec324c674ced9927ecf7891596))

## [0.3.1](https://github.com/m3idnotfree/twitch_highway/compare/v0.3.0..v0.3.1) - 2025-10-26

### Features

- **(clips)** Add Get Clips Download endpoint([d413626](https://github.com/m3idnotfree/twitch_highway/commit/d413626805c93141cbe819e23cfc0a31a5983c44))
- **(users)** Add Get Authorization By User endpoint([c9bdad9](https://github.com/m3idnotfree/twitch_highway/commit/c9bdad95ae716894fbd0ce996760dec3ad38d008))

### Refactor

- **(macros)** Rename endpoint_type to endpoint for consistency([6be6fd1](https://github.com/m3idnotfree/twitch_highway/commit/6be6fd1feaf8f0d813d70c5993660b103163af1b))

## [0.3.0](https://github.com/m3idnotfree/twitch_highway/compare/v0.2.1..v0.3.0) - 2025-10-16

### Features

- **(test)** Add test feature flag([5d156ac](https://github.com/m3idnotfree/twitch_highway/commit/5d156ac5f9fdb024e03662342073c26cdc10bafb))
- **(serde_helpers)** Add deserialize_optional_datetime helper([16f4afe](https://github.com/m3idnotfree/twitch_highway/commit/16f4afe73e76b8b7c605ad9394fad44046a07942))
- **(eventsub)** Add webhook handler([e2906a7](https://github.com/m3idnotfree/twitch_highway/commit/e2906a71187bb5b8669d4ab8beef0799313c9d83))
- **(eventsub)** Add resolve_subscription_type macro for deserialization([d599c7d](https://github.com/m3idnotfree/twitch_highway/commit/d599c7d1e84d9625bd50ae0e2acf3b7f6e46ea9c))
- **(eventsub)** Add websocket message types([3b2b71b](https://github.com/m3idnotfree/twitch_highway/commit/3b2b71b8787f8cea4e90121b408d5315c222785e))
- **(eventsub)** Add JSON Scanner for websocket message parsing([b3fe079](https://github.com/m3idnotfree/twitch_highway/commit/b3fe079f1c0694905a51df132d37a2762f741937))
- **(eventsub)** Add Display trait to websocket MetaData([e879d8a](https://github.com/m3idnotfree/twitch_highway/commit/e879d8a57ef084ed1d3a4614db1df1d52b5933be))
- **(eventsub)** Add Request, Response and Error for websocket([e644ced](https://github.com/m3idnotfree/twitch_highway/commit/e644ced29885365d7a188a1a66dd57fa50c0a5cd))
- **(eventsub)** Add websocket router([d0ef896](https://github.com/m3idnotfree/twitch_highway/commit/d0ef896237966c87d723ebb1b29ee6ef80dd4567))
- **(eventsub)** Implement Service<()> for Router to enable MakeService pattern([a62c2b0](https://github.com/m3idnotfree/twitch_highway/commit/a62c2b01fce32c6dab287d901e07e48e1bb37b96))
- **(eventsub)** Add Event<T> extractor for websocket handlers([8113877](https://github.com/m3idnotfree/twitch_highway/commit/8113877ed4ea0659341265cad2f024be0453a6b9))
- **(eventsub)** Add reconnect URL extraction to Scanner([56a5169](https://github.com/m3idnotfree/twitch_highway/commit/56a51691be011d8df49470498faa63a6e5b4ee30))
- **(eventsub)** Add utility methods to Request([08b0c3b](https://github.com/m3idnotfree/twitch_highway/commit/08b0c3bc39d1987bf078cc6cbb71e515301102de))
- **(eventsub)** Implement Display for Response and make Status public([9298b55](https://github.com/m3idnotfree/twitch_highway/commit/9298b55ca727495b8bfe1e4822ebb20a59630e89))
- **(eventsub)** Add websocket client([202bb17](https://github.com/m3idnotfree/twitch_highway/commit/202bb17e1fa013e432606414738fb12a69de4099))
- **(eventsub)** Implement Extract trait for String([8030318](https://github.com/m3idnotfree/twitch_highway/commit/8030318e5f61f2fe84f1e484bc9bd9f55724f39a))
- **(eventsub)** Add TraceLayer for websocket router([1ffd4d4](https://github.com/m3idnotfree/twitch_highway/commit/1ffd4d498bed39924b782b22465e865d857eefa4))
- **(polls)** Add ChoiceId type for poll choices([2588ea1](https://github.com/m3idnotfree/twitch_highway/commit/2588ea18e8ca7c9d5cb85b75b7ec7694c0b8c598))
- **(eventsub)** Add whisper event type([2c872b3](https://github.com/m3idnotfree/twitch_highway/commit/2c872b33cb83d744054ce920032df59e5ed246f1))
- **(eventsub)** Add user event types([9f4b498](https://github.com/m3idnotfree/twitch_highway/commit/9f4b49807673eae5d9e87029ee1fd4d933ad9a7b))
- **(eventsub)** Add stream event types([564e7a8](https://github.com/m3idnotfree/twitch_highway/commit/564e7a8ad6df2289b5f8b793d988a03a0fc1ec42))
- **(eventsub)** Add hype train event types([52b8774](https://github.com/m3idnotfree/twitch_highway/commit/52b8774e857a2f5dc0f713e4ca7ce014b51a58c5))
- **(eventsub)** Add goals event type([fd87e3e](https://github.com/m3idnotfree/twitch_highway/commit/fd87e3e7cf92c384542c097f33e27d1caba7e95c))
- **(eventsub)** Add extension event type([f63a0f8](https://github.com/m3idnotfree/twitch_highway/commit/f63a0f8b0aa22151e0f75d17a0978b9ea956b32a))
- **(eventsub)** Add drop entitlement grant event type([727ac6a](https://github.com/m3idnotfree/twitch_highway/commit/727ac6aaf41ede0ed1ad01e88d7b39ee6f14488d))
- **(eventsub)** Add conduit event type([b9307e9](https://github.com/m3idnotfree/twitch_highway/commit/b9307e9a16274a2e9fefc46486daf87b97567b2b))
- **(eventsub)** Add charity event types([879c0a1](https://github.com/m3idnotfree/twitch_highway/commit/879c0a11b347f258bf040f0d212af78e3cdeaea6))
- **(eventsub)** Add polls event types([66c488b](https://github.com/m3idnotfree/twitch_highway/commit/66c488becbadc872f5c0435a22af7623616d82e9))
- **(eventsub)** Add guest star event types([de5ebf3](https://github.com/m3idnotfree/twitch_highway/commit/de5ebf3059d00c83453d70c8f3de23bc382b75a5))
- **(eventsub)** Add channel moderator event types([4631b6f](https://github.com/m3idnotfree/twitch_highway/commit/4631b6f6e29fd2a9f965939b741c1e69a66b299b))
- **(eventsub)** Add channel moderate event types([db9e919](https://github.com/m3idnotfree/twitch_highway/commit/db9e919f4a90eb8f4b7450d648f801c68c2095b2))
- **(eventsub)** Add raid event type([7fd5c73](https://github.com/m3idnotfree/twitch_highway/commit/7fd5c7378f5521c56431540669440450e71560dc))
- **(eventsub)** Add channel follow event type([00d7e56](https://github.com/m3idnotfree/twitch_highway/commit/00d7e56f8ff65f657e34e253e3f2f3292c299275))
- **(eventsub)** Add channel unban event types([27511e8](https://github.com/m3idnotfree/twitch_highway/commit/27511e8605120a119a1a77bd66c38f7a4f3daf1e))
- **(eventsub)** Add channel subscribe/cheer/update event types([ab84370](https://github.com/m3idnotfree/twitch_highway/commit/ab843705a09b53c9081f4122b0b6e40758e5f0c6))
- **(eventsub)** Add channel chat event types([ecf06e9](https://github.com/m3idnotfree/twitch_highway/commit/ecf06e961ad074c05338ee9c49840c1b1c74b2e4))
- **(eventsub)** Add channel event types([f75b0a2](https://github.com/m3idnotfree/twitch_highway/commit/f75b0a2e321c6978f2650389af8243082ddcedce))
- **(eventsub)** Add automod event types([c2df24a](https://github.com/m3idnotfree/twitch_highway/commit/c2df24a5267befd59bc3edae44866d343e1adcf6))
- **(macros)** Add simple_endpoint and opt_method([bc6d816](https://github.com/m3idnotfree/twitch_highway/commit/bc6d816773e43cdb449cfa11460e211b01ddff98))
- **(macros)** Add `into` flag to opt_method! macro([84aca68](https://github.com/m3idnotfree/twitch_highway/commit/84aca686710ea93ac59442507a510c9c0b6e0290))
- **(new_types)** Implement Borrow trait and refactor ID types([0f3b87c](https://github.com/m3idnotfree/twitch_highway/commit/0f3b87c9b46c222b7175d67bb31ec847e6f4ecf6))
- **(webhook)** Include verification in framework features([a92e70a](https://github.com/m3idnotfree/twitch_highway/commit/a92e70a2c2f2f80b69f194c07377d0a0301fa570))

### Bug Fixes

- **(moderation)** Handle empty arrays and null dates in response([9aff9b1](https://github.com/m3idnotfree/twitch_highway/commit/9aff9b1735c4685ad4e6cb759ccdb745f704f81b))
- **(streams)** Handle empty arrays and null dates in response([0cc3c4b](https://github.com/m3idnotfree/twitch_highway/commit/0cc3c4b0b537eeb449d9416e2897a6b27dca8df0))
- **(subscriptions)** Handle empty arrays in response([4e45209](https://github.com/m3idnotfree/twitch_highway/commit/4e45209fe9c29432f15895a0d52abd4559c0de3e))
- **(teams)** Handle empty arrays in response([4a29be4](https://github.com/m3idnotfree/twitch_highway/commit/4a29be420a24d376c06f4dda42478bc9cd751e66))
- **(ads)** Deserialize numeric strings as u64([1983a37](https://github.com/m3idnotfree/twitch_highway/commit/1983a373f6fc50a9f536da70d5dfcf46e7994d89))
- **(eventsub)** Correct serde attribute for StreamType enum([f1e903b](https://github.com/m3idnotfree/twitch_highway/commit/f1e903b27640e353e9f26c517976873493360a02))
- **(hype-train)** Correct serde attribute for HypeTrainType enum([3a5e666](https://github.com/m3idnotfree/twitch_highway/commit/3a5e6660a18f4a6b63927b14bf48679135e91559))
- **(eventsub)** Correct serde attribute for GoalType enum([e01052f](https://github.com/m3idnotfree/twitch_highway/commit/e01052fcd4d8c2e7a5c3e21eeb7908e8661dbfab))
- **(moderation)** Wrap ban user body in data field([739afad](https://github.com/m3idnotfree/twitch_highway/commit/739afad2d863f66406c58d7c17dd7458e93a8a0a))

### Refactor

- **(twitchapi)** Move client from TwitchAPIRequest to TwitchAPI([17bfa2d](https://github.com/m3idnotfree/twitch_highway/commit/17bfa2d771df8fc2a80444778a015fba743ccb61))
- [**breaking**] Reorganize module structure and add Clone derives([484395d](https://github.com/m3idnotfree/twitch_highway/commit/484395d154f07861ad07365b94402d30f0e4bd68))
- **(channel_points)** [**breaking**] Reorganize API parameters and types([91f3e5d](https://github.com/m3idnotfree/twitch_highway/commit/91f3e5d75ec552c687fcb612bcb87da33dbd45f5))
- **(channels)** [**breaking**] Make ModifyChannelRequest required in modify_channel_info([fcf04a4](https://github.com/m3idnotfree/twitch_highway/commit/fcf04a4b564bf735cf47184423509f9f55a97410))
- **(chat)** [**breaking**] Make UpdateChatSettingsRequest required in update_chat_settings([eb012e4](https://github.com/m3idnotfree/twitch_highway/commit/eb012e4096124c1b1248f71cdf9e085f5bbba96d))
- [**breaking**] Rename whispers to whisper throughout codebase([b8c5581](https://github.com/m3idnotfree/twitch_highway/commit/b8c55816f56a384570176d449034542e004d476f))
- **(users)** Make description parameter required in update_user([068a8b5](https://github.com/m3idnotfree/twitch_highway/commit/068a8b51b1768f5973de5f9cf1c8f75e779ce882))
- **(new_types)** Simplify serde implementation and add Deref([ac500d2](https://github.com/m3idnotfree/twitch_highway/commit/ac500d24a70e123c976850ca11ce18988fba5d5d))
- **(eventsub)** Replace manual impls with macro for SubscriptionType([0a4032d](https://github.com/m3idnotfree/twitch_highway/commit/0a4032db8dce7bb8b7b89fcefae66e734908785c))
- **(eventsub)** Remove duplicate Subscription from webhook module([b21969f](https://github.com/m3idnotfree/twitch_highway/commit/b21969ff189e7df49eaab8cc8f6b6d9af446b670))
- **(websocket)** Remove message_type detect([4f1e60d](https://github.com/m3idnotfree/twitch_highway/commit/4f1e60d3a86023e446245f594c71f56803645791))
- **(request)** Add Clone/Copy/Hash traits to EndpointType and TokenType and remove required_scopes method([0395f4e](https://github.com/m3idnotfree/twitch_highway/commit/0395f4eb77c3501a57986908a91ea778568059de))
- **(eventsub)** Add RevocationPayload and NotificationPayload types([5a62865](https://github.com/m3idnotfree/twitch_highway/commit/5a628651d168bf06df777ab58a7faeff6ed3d433))
- **(eventsub)** Remove EventSubData and TransportMethod from public API([fc24b8c](https://github.com/m3idnotfree/twitch_highway/commit/fc24b8c69b3727614be3325078a6e922a4d08d28))
- **(Cargo.toml)** Group eventsub features by webhook and websocket([c1ea53c](https://github.com/m3idnotfree/twitch_highway/commit/c1ea53c96452add099c879a343c96b3a3d146db5))
- **(eventsub)** Replace From with FromStr to improve error handling([9c7f082](https://github.com/m3idnotfree/twitch_highway/commit/9c7f082e4058810f3bceae24cdf1ebf81dc465dd))
- **(eventsub)** Extract route builders into routes module([b43b8f1](https://github.com/m3idnotfree/twitch_highway/commit/b43b8f11c8e6959bacefb4ecb70ff9a412388c35))
- **(macros)** Remove compile-time test enforcement from endpoints macro([7e167c6](https://github.com/m3idnotfree/twitch_highway/commit/7e167c6c99cc929f4029e5b506ec7e019a273c4a))
- **(schedule)** Use typed datetime and timezone parameters([23ec5b8](https://github.com/m3idnotfree/twitch_highway/commit/23ec5b80d43dc7df7879e416555f5320e4a574da))
- **(serde_helpers)** Gate entire module with moderation feature([e927e5e](https://github.com/m3idnotfree/twitch_highway/commit/e927e5e0ebffeb3f14f21173d1476a96b3ff21d7))
- **(hype-train)** Extract repeated path segment to constant([7d97770](https://github.com/m3idnotfree/twitch_highway/commit/7d977701037007e9b3690cd67186622caea582d0))
- **(moderation)** Extract repeated path segment to constant([ff9e392](https://github.com/m3idnotfree/twitch_highway/commit/ff9e392f87c7a7f2b1aa436026c69700ce3f52d4))
- **(eventsub)** [**breaking**] Use minimal dependencies and reorganize features([873570d](https://github.com/m3idnotfree/twitch_highway/commit/873570d9ede8e2d42fc761d1b662f7e798365033))
- **(eventsub)** Rename HeaderProvider to HeaderAccess([4b9aa41](https://github.com/m3idnotfree/twitch_highway/commit/4b9aa41e038f90953acc89df84e7ba0d5457ed56))
- **(eventsub)** Rename WebhookError to VerificationError([9c374e9](https://github.com/m3idnotfree/twitch_highway/commit/9c374e99b7da8a87bb011a7c24bfc543fc7f121b))
- **(eventsub)** Reorganize webhook module structure([65a9473](https://github.com/m3idnotfree/twitch_highway/commit/65a94733c6609b9e75b6a97ed4b437f18bb50d24))
- **(eventsub)** Remove Condition generic parameter([b8820a9](https://github.com/m3idnotfree/twitch_highway/commit/b8820a9c09ab5b8412d1738a5ec6bc7d32c2cbee))
- **(predictions)** Move TopPredictor to shared types module([b50f484](https://github.com/m3idnotfree/twitch_highway/commit/b50f484df43b3aca35e4af90d139e3f2ea086358))
- **(hype-train)** Move TopContribution/ContributionType to shared types module([2aba257](https://github.com/m3idnotfree/twitch_highway/commit/2aba257291683c3aa0bcc97d3e666cacc1fa9d43))
- **(predictions)** Move Outcome/OutcomeColor to shared types module([107cc66](https://github.com/m3idnotfree/twitch_highway/commit/107cc669bd8e402bd24ccf90202ca992fdf06c03))
- **(channel-points)** Move Reward to shared types module([5587ba6](https://github.com/m3idnotfree/twitch_highway/commit/5587ba69f26681e14fc7fd54267626159072405d))
- **(polls)** Move Choice to shared types module([a44fdc3](https://github.com/m3idnotfree/twitch_highway/commit/a44fdc39c5a0e13b85ad95133e2b6a4233e890f7))
- **(charity)** Move Amount to shared types module([e336864](https://github.com/m3idnotfree/twitch_highway/commit/e3368645766eb9163f2fa46bb58976c0b9958f49))
- **(eventsub)** Convert channels to directory module([c601a53](https://github.com/m3idnotfree/twitch_highway/commit/c601a534b27e390f13ad88a5453d9101fb79aac3))
- **(eventsub)** Consolidate all channel events under channels/ directory([e7382f0](https://github.com/m3idnotfree/twitch_highway/commit/e7382f08cf8c14f36b4894d80df79b029becc2d2))
- **(whisper)** [**breaking**] Make SendWhisperBody private([0fc1ae0](https://github.com/m3idnotfree/twitch_highway/commit/0fc1ae03eed5444475d1585491c9f0111f567505))
- **(analytics)** [**breaking**] Migrate to builder pattern([ace4cf2](https://github.com/m3idnotfree/twitch_highway/commit/ace4cf2e4bbca5f3bf2933bf0a5c93f507aa8610))
- **(bits)** [**breaking**] Migrate to builder pattern([c391007](https://github.com/m3idnotfree/twitch_highway/commit/c391007713a401567eb998d279e278b8b26fac00))
- **(channel-points)** [**breaking**] Migrate to builder pattern([bc72051](https://github.com/m3idnotfree/twitch_highway/commit/bc720516c4bdcb0a93b214c672c242381b5f8b0f))
- **(channels)** [**breaking**] Migrate to builder pattern([2fd2161](https://github.com/m3idnotfree/twitch_highway/commit/2fd21615badfb62c2b7a00de79328c3500ce3089))
- **(entitlements)** [**breaking**] Migrate to builder pattern([384da1e](https://github.com/m3idnotfree/twitch_highway/commit/384da1eec39d2d7376dd98cd88cfe1856707a399))
- **(chat)** [**breaking**] Migrate to builder pattern([048275c](https://github.com/m3idnotfree/twitch_highway/commit/048275c6258587fc6e6d5a77b95163f62c842839))
- **(clips)** [**breaking**] Migrate to builder pattern([b631ded](https://github.com/m3idnotfree/twitch_highway/commit/b631dedc9a15f647cc2a07029670d03942ec0291))
- **(conduits)** [**breaking**] Migrate to builder pattern([20ad7b0](https://github.com/m3idnotfree/twitch_highway/commit/20ad7b04c160c39db9bb56dd0c5c0c4d80d8e1dc))
- **(eventsub)** [**breaking**] Migrate to builder pattern([3203bea](https://github.com/m3idnotfree/twitch_highway/commit/3203beafd41f6c89717086368400ca55c4bed149))
- **(games)** [**breaking**] Migrate to builder pattern([5f5261f](https://github.com/m3idnotfree/twitch_highway/commit/5f5261f40453af1fed6d73f2f6968c9e89bfc371))
- **(videos)** [**breaking**] Migrate to builder pattern([2ec6db0](https://github.com/m3idnotfree/twitch_highway/commit/2ec6db064ce62c1e50d0ea04e0be609c1ccb2cce))
- **(users)** [**breaking**] Migrate to builder pattern([627e5c6](https://github.com/m3idnotfree/twitch_highway/commit/627e5c68de97b310df1897c2153a8e8c42ab3664))
- **(streams)** [**breaking**] Migrate to builder pattern([0735597](https://github.com/m3idnotfree/twitch_highway/commit/0735597a453dcabaf437b0c0128662080027bbe0))
- **(polls)** [**breaking**] Migrate to builder pattern([822b0b7](https://github.com/m3idnotfree/twitch_highway/commit/822b0b7e2bf1545707e011e7cb7cbf4f5966794d))
- **(teams)** [**breaking**] Migrate to builder pattern([82046ff](https://github.com/m3idnotfree/twitch_highway/commit/82046ffc905482cf8b37399626b9763a4c524250))
- **(subscriptions)** [**breaking**] Migrate to builder pattern([1e8bfce](https://github.com/m3idnotfree/twitch_highway/commit/1e8bfcebf09ef65894524b0d6f7ff026a34dfab4))
- **(search)** [**breaking**] Migrate to builder pattern([8aaa709](https://github.com/m3idnotfree/twitch_highway/commit/8aaa7093007696f61762a2d111a3eda7fe055f08))
- **(schedule)** [**breaking**] Migrate to builder pattern([a0de3a1](https://github.com/m3idnotfree/twitch_highway/commit/a0de3a190faa5892d62e27e499f459a82d0daca6))
- **(predictions)** [**breaking**] Migrate to builder pattern([6006cd9](https://github.com/m3idnotfree/twitch_highway/commit/6006cd9c490276cc4da95c56073f0c0f04c05470))
- **(moderation)** [**breaking**] Migrate to builder pattern([260b74e](https://github.com/m3idnotfree/twitch_highway/commit/260b74ec0f31c06d7dfd5849d5d1e896ed57c772))
- **(hype-train)** [**breaking**] Migrate to builder pattern([d597f51](https://github.com/m3idnotfree/twitch_highway/commit/d597f512ab55efcb4978e50003a971dabcd4624f))
- **(guest-star)** [**breaking**] Migrate to builder pattern([d0bff8b](https://github.com/m3idnotfree/twitch_highway/commit/d0bff8bf16d0443868c926fb6cd6886cec2109c1))
- **(extensions)** [**breaking**] Migrate to builder pattern([1a3ac66](https://github.com/m3idnotfree/twitch_highway/commit/1a3ac661c8ebceacb8f9f8703dbb712ce01c7269))
- **(charity)** [**breaking**] Migrate to builder pattern([9382d59](https://github.com/m3idnotfree/twitch_highway/commit/9382d59b74ac37f4a373b75e245c8014e49685ff))
- **(ads)** Replace endpoints with simple_endpoint macro([30138b2](https://github.com/m3idnotfree/twitch_highway/commit/30138b23090a9d6b5e6c85fd121d04d85eff8f17))
- **(ccls)** Replace endpoints with simple_endpoint macro([c64b636](https://github.com/m3idnotfree/twitch_highway/commit/c64b6367ef4369517ab8482f3979d7f3e2132713))
- **(goals)** Replace endpoints with simple_endpoint macro([9cd8bb6](https://github.com/m3idnotfree/twitch_highway/commit/9cd8bb6dd8c1b354c3e3a360a5e43e2d2dc1b5d6))
- **(raid)** Replace endpoints with simple_endpoint macro([230d650](https://github.com/m3idnotfree/twitch_highway/commit/230d65068a95dd9644cdefd81d46e9f7fa58bbf6))
- **(whisper)** Replace endpoints with simple_endpoint macro([2afa114](https://github.com/m3idnotfree/twitch_highway/commit/2afa114013aa248ceccfdfb30d3e50af7de25d07))
- Replace define_request! macro with inline JSON([9338e1b](https://github.com/m3idnotfree/twitch_highway/commit/9338e1b38c8cd85841bb635bd3802d94fdf4f983))
- **(extensions)** Remove RequestBody wrapper and add SendExtensionPubSubMessageBody([bc0660c](https://github.com/m3idnotfree/twitch_highway/commit/bc0660c810369bc0940ac513dea5c3e2aea6b2c4))
- **(channel-points)** Use simple_endpoint! macro([a772214](https://github.com/m3idnotfree/twitch_highway/commit/a77221444ccad83f01b59b89f8a9495c68daf204))
- **(types)** Remove PaginationQuery([9f79494](https://github.com/m3idnotfree/twitch_highway/commit/9f79494a79035726114667319f5a5cd605b5dfa1))
- **(test)** Add webhook, websocket([fcd3a41](https://github.com/m3idnotfree/twitch_highway/commit/fcd3a41dec7a44424a600450d7d87f0f6b61afc9))
- Replace inline JSON with typed request body structs([e8c3e31](https://github.com/m3idnotfree/twitch_highway/commit/e8c3e31252995cdab84754e2a6d116bff165de65))
- **(ads)** Use u8 for commercial length([26afc23](https://github.com/m3idnotfree/twitch_highway/commit/26afc23e1674ad9a6fbc0ccb1bfbea205ce88e9b))
- **(analytics)** Improve type safety and code organization([f2c7943](https://github.com/m3idnotfree/twitch_highway/commit/f2c7943026d8bc18069950dc0e1a17410b5a9cd3))
- **(bits)** Replace Id with TransactionId([c835b0d](https://github.com/m3idnotfree/twitch_highway/commit/c835b0db40f6ed80af014f8e6c1d6e84b4783338))
- **(ccls)** Replace Id with String([afc1a47](https://github.com/m3idnotfree/twitch_highway/commit/afc1a4729ed0f37f532da6c624685e6a21aeaa04))
- **(charity)** Replace Id with CampaignId([09d1f47](https://github.com/m3idnotfree/twitch_highway/commit/09d1f473be6b69601aacef9a971b213f634bc607))
- **(chat)** Replace Id with EmoteId([dab64ae](https://github.com/m3idnotfree/twitch_highway/commit/dab64aee43b50d8be5007b6fe928e07057151f19))
- **(clips)** Improve type safety([5fe0808](https://github.com/m3idnotfree/twitch_highway/commit/5fe08082c9562773e8a63bcd812678c2ff231dfd))
- **(types)** Validate cursor is non-empty on deserialize([a70755a](https://github.com/m3idnotfree/twitch_highway/commit/a70755a8fedcab2ad9690a82897ab931995eb239))
- **(conduits)** Use shared Pagination type([964d71d](https://github.com/m3idnotfree/twitch_highway/commit/964d71d6ca3f7245d59891b07de4a0d476398cd1))
- **(extensions)** Improve type safety([ddf0a0d](https://github.com/m3idnotfree/twitch_highway/commit/ddf0a0deeaf6f0b07e2056276a9c14ee713edc43))
- **(games)** Replace Id with GameId([6037d3d](https://github.com/m3idnotfree/twitch_highway/commit/6037d3d4692cf2db2216a75545aac7352560fb6e))
- **(types)** Improve Category type safety([3e053df](https://github.com/m3idnotfree/twitch_highway/commit/3e053df3a49424356be8902a433e58dc9dac9e4c))
- **(goals)** Replace Id with GoalId([4de39de](https://github.com/m3idnotfree/twitch_highway/commit/4de39de3e2b06f974517719633dc1426b4852976))
- **(guest-star)** Replace Id with SessionId([2630aa0](https://github.com/m3idnotfree/twitch_highway/commit/2630aa064a8a2fa01428c80ea0425b33720e3e05))
- **(hype-train)** Replace Id with HypeTrainId([e548ff1](https://github.com/m3idnotfree/twitch_highway/commit/e548ff1b288cfc99f1214967ac99561c80f023bd))
- **(moderation)** Improve type safety([b9da611](https://github.com/m3idnotfree/twitch_highway/commit/b9da611619a3f94b26fa7f1b2b67651792f23da7))
- **(polls)** Replace Id with PollId([94d89c8](https://github.com/m3idnotfree/twitch_highway/commit/94d89c8a72a841518f6841eedca9e9d9492a9521))
- **(schedule)** Improve type safety([523691c](https://github.com/m3idnotfree/twitch_highway/commit/523691c4d5ed0d9f562b1149c22822d1b44841d0))
- **(search)** Improve type safety([ec2d501](https://github.com/m3idnotfree/twitch_highway/commit/ec2d5016d97e5c0e7edcc68225dbeba2ffd0625c))
- **(streams)** Improve type safety([4aefda9](https://github.com/m3idnotfree/twitch_highway/commit/4aefda9c347a65fe9d03abb90a16b354d04e9424))
- **(teams)** Replace Id with TeamId([2f76354](https://github.com/m3idnotfree/twitch_highway/commit/2f76354596034aecd15fd08d23269bf4d8766b33))
- **(macros)** Remove over-engineered endpoint macros([e8cbf8f](https://github.com/m3idnotfree/twitch_highway/commit/e8cbf8fb9b2561ba75bfa399e1ce6bcc5b6ef69c))

### Documentation

- **(ads)** Document API endpoints([338604f](https://github.com/m3idnotfree/twitch_highway/commit/338604f734645e8a66e421605579bc28bac224ae))
- **(ads)** Document API endpoints([2940981](https://github.com/m3idnotfree/twitch_highway/commit/2940981f4b1f169d51696a08d3e7d656bb5a1d94))
- **(ccls)** Document API endpoints([dd31cf4](https://github.com/m3idnotfree/twitch_highway/commit/dd31cf46db111c04c3b8660a3fce9fab72027376))
- **(whisper)** Document API endpoints([a2346bb](https://github.com/m3idnotfree/twitch_highway/commit/a2346bbb6d0a730ae8e4b936f94ee972af4e8e0d))
- **(raid)** Document API endpoints([91fefb1](https://github.com/m3idnotfree/twitch_highway/commit/91fefb1dd7c1250359ae368973d58914273547af))
- **(goals)** Document API endpoints([70c5f59](https://github.com/m3idnotfree/twitch_highway/commit/70c5f5932ebbf1a410e67f229cbef7e0071f52a7))
- **(hype-train)** Mark get_hype_train_events as deprecated([555e8df](https://github.com/m3idnotfree/twitch_highway/commit/555e8df59b6d2a7f8ac593c54b398e0c2dec1c9f))
- Restructure and improve documentation([c26cb95](https://github.com/m3idnotfree/twitch_highway/commit/c26cb9509d1b118ac2c4aa8704536c41628ce669))
- **(schedule)** Fix typo in doc comment([3cfbc10](https://github.com/m3idnotfree/twitch_highway/commit/3cfbc1016875a01e445e75219f248dce6e80b792))

### Performance

- Add conditional compilation to macros and constants([d7d20ac](https://github.com/m3idnotfree/twitch_highway/commit/d7d20ac462e257b40b29db15cf0fb597987e82c4))
- **(types)** Add conditional compilation with feature gates([2c793df](https://github.com/m3idnotfree/twitch_highway/commit/2c793dfa42075cc9e412980f2cc796d079bd77a8))

### Testing

- Reorganize tests([b4b6545](https://github.com/m3idnotfree/twitch_highway/commit/b4b6545f65eadcebaad7793b27de7f1e455b2ed6))
- **(eventsub)** Add webhook message verifictaion test([1da836b](https://github.com/m3idnotfree/twitch_highway/commit/1da836b02f4b592db104ef9513601e6031b6c346))
- **(http_mocks)** Rename get_channel_editors -> get_channel_editor([d6f19c3](https://github.com/m3idnotfree/twitch_highway/commit/d6f19c3d91afcf74df321e95393373b0cb9f7c8f))
- Add version parameter to event triggers([3c3db87](https://github.com/m3idnotfree/twitch_highway/commit/3c3db87e6382a4760dce0f7f3275283427e68162))
- Migrate to builder pattern API([977a91b](https://github.com/m3idnotfree/twitch_highway/commit/977a91bf468af9da51b0b7f4106ec591432bfb6c))
- Update tests for new type system([5056262](https://github.com/m3idnotfree/twitch_highway/commit/5056262a3c5c0298add920025e876c49a9a1a580))

### Miscellaneous

- Adopt dual license (MIT OR Apache-2.0)([312b59c](https://github.com/m3idnotfree/twitch_highway/commit/312b59c56e7ca0f6acbfc0a7029cff9ff94bf540))
- Bump twitch_oauth_token to 2.0.5([53de426](https://github.com/m3idnotfree/twitch_highway/commit/53de4262b9800bf3065e4a485e106f69b6fb8c53))
- **(test_utils)** Adapt to asknothingx2-util([6a40257](https://github.com/m3idnotfree/twitch_highway/commit/6a402576dd0c29385cd623c13ac21256fa2ee98f))
- Remove unused imports and fix json!([aaed3c2](https://github.com/m3idnotfree/twitch_highway/commit/aaed3c21bbb0bd6c9b81cea8621aa99ac4f87654))
- Fix docs typos and remove unused imports([1441525](https://github.com/m3idnotfree/twitch_highway/commit/14415258061a94600824c5d018cd8075d40997b7))
- Fix docs typos([5234646](https://github.com/m3idnotfree/twitch_highway/commit/5234646dec2b4d0e058562ec3b6c7b626650801f))

## [0.2.1](https://github.com/m3idnotfree/twitch_highway/compare/v0.2.0..v0.2.1) - 2025-08-23

### Features

- **(eventsub)** Implement EventSub API([ca4527e](https://github.com/m3idnotfree/twitch_highway/commit/ca4527e277c92c906dd9a32538113283ce16b1f4))
- **(conduits)** Implement Conduits API([29ba638](https://github.com/m3idnotfree/twitch_highway/commit/29ba6382682d9b8c2bdaa7e24d92cf8a1ed6804c))

### Refactor

- **(new-type)** Replace new() method with From trait([64fcd21](https://github.com/m3idnotfree/twitch_highway/commit/64fcd21a52d9c9b279c6a3424585ebc04cd248aa))
- Enhance endpoints macro and fix compiler warnings([426a0c0](https://github.com/m3idnotfree/twitch_highway/commit/426a0c09eb96fd36b155110b6a186a4d04f3d4ed))
- **(types)** Extract Status enum to shared types for eventsub/conduits([ed186c9](https://github.com/m3idnotfree/twitch_highway/commit/ed186c963c3f512d37218dfc26b2d6f05cceecbf))
- **(macros)** Simplify endpoints macro syntax and migrate all modules([dcbc2de](https://github.com/m3idnotfree/twitch_highway/commit/dcbc2de858580112ea21b919c2ce76750aa89c80))
- **(types)** Extract duplicate EVENTSUB constant from modules([201c874](https://github.com/m3idnotfree/twitch_highway/commit/201c8749c356b71353db81e55eb1b97d05f572ba))

## [0.2.0](https://github.com/m3idnotfree/twitch_highway/compare/v0.1.5..v0.2.0) - 2025-08-22

### Features

- **(macros)** Add request query generation macro([a60c1eb](https://github.com/m3idnotfree/twitch_highway/commit/a60c1eb67ede1a97785797c239523a01071add32))
- **(macros)** Add flags for conditional trait impls([5be6595](https://github.com/m3idnotfree/twitch_highway/commit/5be65953c420739369a86801678383c6652d0639))
- **(macros)** Add new type generation macro([3e4a7a2](https://github.com/m3idnotfree/twitch_highway/commit/3e4a7a2615d4f63b1f014bfba64f13f5fe6a1267))
- **(macros)** Add twitch_api_trait macro([b019a9b](https://github.com/m3idnotfree/twitch_highway/commit/b019a9b819e1ad83c4194cd7421a5f0dd77013dd))
- **(test)** Add TwitchApiTest utilities with ads endpoint mocking([0e9f02d](https://github.com/m3idnotfree/twitch_highway/commit/0e9f02d268cedd8ed11717f77a2a75397ae03484))
- **(request)** Make send method public([da6226b](https://github.com/m3idnotfree/twitch_highway/commit/da6226b9ee8bac204d1a91121bb8c3976f3f55ad))
- **(tests)** Add macro-enforced test coverage and standardize test naming([055cb14](https://github.com/m3idnotfree/twitch_highway/commit/055cb14544cab588cab91f5684a9393030c53998))
- **(macros)** Auto-generate basic endpoint tests([55e5ecb](https://github.com/m3idnotfree/twitch_highway/commit/55e5ecbd6a170faa7fa46384fed6df5b48970a6d))
- **(request)** Handle 204 No Content responses in json() method([e512976](https://github.com/m3idnotfree/twitch_highway/commit/e5129769073a497e79534e26b4e42c2b009d0fe8))
- **(twitchapi)** Add set_access_token and set_client_id methods([58c5d3f](https://github.com/m3idnotfree/twitch_highway/commit/58c5d3f59daa833631414fca1c92746d18e3bf69))

### Refactor

- **(base)** Remove TwitchAPIBase trait([e3cbe70](https://github.com/m3idnotfree/twitch_highway/commit/e3cbe70e37755fe81dc6cc147ed73b53317233fb))
- **(videos)** Remove unnecessary allocation of VideosRequest([a8bfb2c](https://github.com/m3idnotfree/twitch_highway/commit/a8bfb2ca7d89c4223750c5ad5f5f9acbf3b2ce64))
- **(types)** Remove unnecessary allocation of PaginationQuery([7c0d556](https://github.com/m3idnotfree/twitch_highway/commit/7c0d556ecc32224a2d5846f108b6c5e7699e2c82))
- **(macros)** Reorganized([b841b67](https://github.com/m3idnotfree/twitch_highway/commit/b841b67b37c1aae30d1edf95b4064a6eb09d47ab))
- **(base)** Merged QueryParams into UrlBuilder, removed IntoQueryPairs trait([bb65cc5](https://github.com/m3idnotfree/twitch_highway/commit/bb65cc522e91231369d67003f03576c0188a2367))
- Migrate to new macro([970b570](https://github.com/m3idnotfree/twitch_highway/commit/970b57038ba7b3a1f6c1561aa1f202d1ead7224f))
- **(request)** Simplify request body handling([051ff11](https://github.com/m3idnotfree/twitch_highway/commit/051ff11073a960afb2f73c8677375d373b852538))
- **(request)** Upgrade asknothing-util and replace EmptyBody with NoContent([05ca47e](https://github.com/m3idnotfree/twitch_highway/commit/05ca47e72f65bb14a312cb61c26eacca1b274352))
- **(lib)** Simplify API structure by removing helper functions([b87be29](https://github.com/m3idnotfree/twitch_highway/commit/b87be29ea3e48aaa5b2357c38fc2d2d4300de663))
- **(error)** Replace asknothingx2-util errors with direct reqwest errors([982a7d2](https://github.com/m3idnotfree/twitch_highway/commit/982a7d2877d1fb5068a5c48cc8e6b85f505c4d7f))
- **(new_types)** Improve conversion methods and add tests([eecea2d](https://github.com/m3idnotfree/twitch_highway/commit/eecea2d91b8f71c63e5fecc85b80419312cbae9a))
- **(pagination)** Add Copy trait and update method name([ebdcd66](https://github.com/m3idnotfree/twitch_highway/commit/ebdcd667ac0475640546f10366a46d5457e814db))
- **(error)** Replace thiserror enum with custom boxed error type([3be30d3](https://github.com/m3idnotfree/twitch_highway/commit/3be30d3d03fd4be59718a94a90e06f12f92c1448))
- **(request)** Improve response handling and enhance error handling([e448320](https://github.com/m3idnotfree/twitch_highway/commit/e448320cce6504f490b4fbb667aa79b57d461a4f))
- **(test_utils)** Extract assert_datetime from TwitchApiTest([bc9559c](https://github.com/m3idnotfree/twitch_highway/commit/bc9559cf312f169528b7470e14c51d78141119d3))
- **(ads)** Migrate to twitch_api_trait macro and add full test coverage([fe6aa7f](https://github.com/m3idnotfree/twitch_highway/commit/fe6aa7fb89b131834460f2c621b4f6d469ba7df3))
- **(types)** Replace String with DateTime<FixedOffset> in DateRange([90779f0](https://github.com/m3idnotfree/twitch_highway/commit/90779f0a03178964ab4de8c7cd7405f724535490))
- **(analytics)** Migrate to twitch_api_trait macro and add full test coverage([f06322c](https://github.com/m3idnotfree/twitch_highway/commit/f06322cf709a1ddd484022de2ba0dd30e2bb5410))
- **(bits)** Migrate to twitch_api_trait and add full test coverage([2370be7](https://github.com/m3idnotfree/twitch_highway/commit/2370be7d9f8db769c08fa83250429899be214abb))
- **(ccls)** Migrate to twitch_api_trait and add full test coverage([88a2aea](https://github.com/m3idnotfree/twitch_highway/commit/88a2aea180fe9d892e3c7806c1e889f8059d12e6))
- **(request)** Move response status validation to send method([c24a110](https://github.com/m3idnotfree/twitch_highway/commit/c24a110d75868544dba26902d37816c393aab89d))
- **(channel-points)** Migrate to twitch_api_trait and add full test coverage([260e976](https://github.com/m3idnotfree/twitch_highway/commit/260e97612e696d12bfdf28e77843eea2f48be639))
- **(channels-points)** Migrate to twitch_api_trait and add full test coverage([6c295de](https://github.com/m3idnotfree/twitch_highway/commit/6c295def1467502a9e61165249b01f8cda4c0408))
- **(charity)** Migrate to twitch_api_trait and add full test coverage([14aee7c](https://github.com/m3idnotfree/twitch_highway/commit/14aee7c0a5ab4aba172406ddc41e747d6c148878))
- **(chat)** Migrate to twitch_api_trait and add full test coverage([e9f1db7](https://github.com/m3idnotfree/twitch_highway/commit/e9f1db74fb6881f08f5dec5b0a37b05fb50a56df))
- **(clips)** Migrate to twitch_api_trait and add full test coverage([19efa1e](https://github.com/m3idnotfree/twitch_highway/commit/19efa1e24cf7501e463b804aa2024cc1860b4b72))
- **(entitlements)** Migrate to twitch_api_trait and add full test coverage([fc68353](https://github.com/m3idnotfree/twitch_highway/commit/fc68353b04d6ca89742c721276720c8034fb86fb))
- **(extensions)** Migrate to twitch_api_trait and add full test coverage([59f20a3](https://github.com/m3idnotfree/twitch_highway/commit/59f20a30b82098936c360ebfdf356b915d3f0122))
- **(games)** Migrate to twitch_api_trait and add full test coverage([777c419](https://github.com/m3idnotfree/twitch_highway/commit/777c419b8ebeba1d115f5bebca6035d23fa57ad5))
- **(goals)** Migrate to twitch_api_trait and add full test coverage([d6d4e8b](https://github.com/m3idnotfree/twitch_highway/commit/d6d4e8be7554eac76dc5b56cf2e1f3b2015440cc))
- **(guest-star)** Migrate to twitch_api_trait and add full test coverage([e626c2c](https://github.com/m3idnotfree/twitch_highway/commit/e626c2c33fdab58e1a70c3b50bb2c5e575d001d2))
- **(hype-train)** Migrate to twitch_api_trait and add full test coverage([b60fcbc](https://github.com/m3idnotfree/twitch_highway/commit/b60fcbc94bb0ee49cf60bb032b0b747f1b7116bd))
- **(moderation)** Migrate to twitch_api_trait([bc6efe5](https://github.com/m3idnotfree/twitch_highway/commit/bc6efe511585dbfb7116cdebf82bb16a0dde0f08))
- **(macros)** Rename twitch_api_trait to endpoints and simplify syntax([30c99ab](https://github.com/m3idnotfree/twitch_highway/commit/30c99ab794770a7c4fee7f6b709f04e0ee1d4efc))
- **(test_utils)** Simplify mocks for auto-generated base tests([6bb06f1](https://github.com/m3idnotfree/twitch_highway/commit/6bb06f1f9328f9fba82b739958b5b7aee724e9bd))
- **(ads)** Migrate to auto-generated base tests([0d95e48](https://github.com/m3idnotfree/twitch_highway/commit/0d95e48dd637a6e46376e66d9a8fcf55d2c36cf0))
- **(analytics)** Migrate to auto-generated base tests([f0a767c](https://github.com/m3idnotfree/twitch_highway/commit/f0a767c0f2606c4e6a05a2edb9546751f7be4450))
- **(macros)** Improve parameter convenience([22100ad](https://github.com/m3idnotfree/twitch_highway/commit/22100ad59c30b4e07c37e1570250e2c98dd09cdf))
- Migrate endpoints macros([9c9bfa3](https://github.com/m3idnotfree/twitch_highway/commit/9c9bfa3041da254558db22f5a40cf7aec0506f3c))

### Documentation

- **(lib)** Add error handling documentation([97c43a0](https://github.com/m3idnotfree/twitch_highway/commit/97c43a050c11f9bd64fbc8390e66ed6cf5fd30bc))
- Improve documentation([bd13811](https://github.com/m3idnotfree/twitch_highway/commit/bd138116e947565a8dc6390268113396e26f8e11))

### Performance

- **(urlbuilder)** Create url once([3033fcf](https://github.com/m3idnotfree/twitch_highway/commit/3033fcf4ac039d20856af5d8b7a018cbed00f321))
- **(urlbuilder)** Add zero-alloc query_u64 methods([1d2b3b0](https://github.com/m3idnotfree/twitch_highway/commit/1d2b3b08a9a207036d5328a18352ad5f09f5e178))

### Styling

- **(types)** Use impl Trait instead of generic parameter([d5d0d74](https://github.com/m3idnotfree/twitch_highway/commit/d5d0d74043f9af1f580883017179fd9b0d10d524))

### Testing

- Remove tests using deprecated asknothingx2-util API([367e20f](https://github.com/m3idnotfree/twitch_highway/commit/367e20f7a335f6fafaa38f3167a76522a0132d07))
- **(cost)** Add serde roundtrip test([6960083](https://github.com/m3idnotfree/twitch_highway/commit/6960083731322741b51949faa614e6bc82e33def))

### Miscellaneous

- Update gitignore([9f1bf75](https://github.com/m3idnotfree/twitch_highway/commit/9f1bf753c8f82f81df087a539aaaad0ff24e0749))
- **(request)** Update imports and improve API consistency([6a168d5](https://github.com/m3idnotfree/twitch_highway/commit/6a168d55f08459248f021376b8553e69e5a9d633))

## [0.1.5](https://github.com/m3idnotfree/twitch_highway/compare/v0.1.4..v0.1.5) - 2025-01-22

### Features

- **(test_url)** Add parameter use_prefix([2b5e773](https://github.com/m3idnotfree/twitch_highway/commit/2b5e773ad396436ede120f701df8a44e4d43dc8e))

### Refactor

- **(core)** Reorganize base, request module([478c67c](https://github.com/m3idnotfree/twitch_highway/commit/478c67ca0c9768078bb3f8a304f02dd6e458dca2))

## [0.1.4](https://github.com/m3idnotfree/twitch_highway/compare/v0.1.3..v0.1.4) - 2025-01-21

### Refactor

- **(channel-points)** Improve type safety with dedicated ID types([5a43879](https://github.com/m3idnotfree/twitch_highway/commit/5a4387938ab31a7de35635e598c3fe6d8d26311f))

## [0.1.3](https://github.com/m3idnotfree/twitch_highway/compare/v0.1.2..v0.1.3) - 2025-01-21

### Documentation

- Add docsrs feature([87fb798](https://github.com/m3idnotfree/twitch_highway/commit/87fb798fafb1e3aa85260339c018bb0fad89ff8f))

## [0.1.2](https://github.com/m3idnotfree/twitch_highway/compare/v0.1.1..v0.1.2) - 2025-01-20

### Documentation

- Update([d39c16c](https://github.com/m3idnotfree/twitch_highway/commit/d39c16cfbfb5a6e7fae91f6798d64db7cf56f091))

## [0.1.0](https://github.com/m3idnotfree/twitch_highway/compare/v0.0.10..v0.1.0) - 2025-01-20

### Features

- **(test_url)** Add mock server url endpoint([927fa84](https://github.com/m3idnotfree/twitch_highway/commit/927fa84b7bf5cc3be4e0bb10a922d185a74c92ce))
- **(request)** Implement API request([4d3bf4d](https://github.com/m3idnotfree/twitch_highway/commit/4d3bf4d596361c5f87f04d970c65bade28f1bf94))
- **(error)** Update([16a0b39](https://github.com/m3idnotfree/twitch_highway/commit/16a0b3924f9d95585f21ee0e12f09864c2bc6253))
- **(base)** Add IntoQueryPairs trait([2839ac2](https://github.com/m3idnotfree/twitch_highway/commit/2839ac22fddd2befa0f4c465b622c47f1d854850))
- **(macros)** Add helper macros([e2bc3e3](https://github.com/m3idnotfree/twitch_highway/commit/e2bc3e3d7bc24b1cd6878b7f2f35a979357494e5))
- **(base)** Add query help method([c4c132d](https://github.com/m3idnotfree/twitch_highway/commit/c4c132db203365cb723855a447afd038ff1eebd9))
- **(request)** Update EndpointType([ba9d1db](https://github.com/m3idnotfree/twitch_highway/commit/ba9d1dba74260061db5e1a967e39757426567c2d))
- **(types)** Add date_range, image, new_types, pagination, title([ca1c4f7](https://github.com/m3idnotfree/twitch_highway/commit/ca1c4f7600a24871a2d8fe3da0607adf1fe3fd70))
- **(ads)** Implement Twitch Ads API endpoints([e75fbe3](https://github.com/m3idnotfree/twitch_highway/commit/e75fbe35b3d5ad17283b13db07013354b2430f33))
- **(analytics)** Implement Twitch Analytics API endpoints([ee31eb2](https://github.com/m3idnotfree/twitch_highway/commit/ee31eb2aea325066fc6452e7700958ed6ef30f39))
- **(bits)** Implement Twitch Bits API endpoints([c95c5e9](https://github.com/m3idnotfree/twitch_highway/commit/c95c5e98702e0b43be70bcdef86334b56b4aeac2))
- **(types)** Add Id, implement From trait([114089c](https://github.com/m3idnotfree/twitch_highway/commit/114089ca629b613a1231d9810b994e7501a8a14a))
- **(channel_points)** Implement Twitch Channel Points API endpoints([934af6d](https://github.com/m3idnotfree/twitch_highway/commit/934af6de926aaad218f3fd32b79ece69f653cd32))
- **(channels)** Implement Twitch Chanels API endpoints([939dc10](https://github.com/m3idnotfree/twitch_highway/commit/939dc1093b2f8880792a776846ab0bf171f85775))
- **(charity)** Implement Twitch Charity API endpoints([e5d0f49](https://github.com/m3idnotfree/twitch_highway/commit/e5d0f494eda894914ecfa9a5757aae793b656b4b))
- **(clips)** Implement Twitch Clips API endpoints([11eb7a9](https://github.com/m3idnotfree/twitch_highway/commit/11eb7a90a579e1366304b69c99dc457c46cf73ac))
- **(moderation)** Implement Twitch Moderation API endpoints([f06e237](https://github.com/m3idnotfree/twitch_highway/commit/f06e2374ca35f687ebc43af4e7a3f76727c45dcb))
- **(whispers)** Implement Twitch Whispers API endpoints([eba6028](https://github.com/m3idnotfree/twitch_highway/commit/eba6028b19ba614ec5251d19c00fbdb63973e08d))
- **(videos)** Implement Twitch Videos API endpoints([8d77d36](https://github.com/m3idnotfree/twitch_highway/commit/8d77d361f0b61506db65f37cce371cbb710ada20))
- **(subscriptions)** Implement Twitch Subscriptions API endpoints([d2b232f](https://github.com/m3idnotfree/twitch_highway/commit/d2b232fa8c2303de3f5bf442592c82debc5d9f45))
- **(streams)** Implement Twitch Streams API endpoints([acc9f19](https://github.com/m3idnotfree/twitch_highway/commit/acc9f19989ff6b5171b1d8ce7d02938cba5cb05b))
- **(raid)** Implement Twitch Raid API endpoints([7b1318a](https://github.com/m3idnotfree/twitch_highway/commit/7b1318a5c808d75bcfed42c05f637c143ddba370))
- **(polls)** Implement Twitch Polls API endpoints([e4a687d](https://github.com/m3idnotfree/twitch_highway/commit/e4a687db733699ab8e47e4ee27f50f2098f9cd2c))
- **(hype_train)** Implement Twitch HypeTrain API endpoints([a1cc27c](https://github.com/m3idnotfree/twitch_highway/commit/a1cc27c980924d809cdafe2231654376913ff192))
- **(predictions)** Implement Twitch Predictions API endpoints([ab1912f](https://github.com/m3idnotfree/twitch_highway/commit/ab1912f8951aa020e0ffd6b631b30bcef22e1ba6))
- **(ccls)** Implement Twitch CCLs API endpoints([5ae130d](https://github.com/m3idnotfree/twitch_highway/commit/5ae130db3da3f1bf982a7aafd704f5320fc27ac4))
- **(teams)** Implement Twitch Teams API endpoints([78de1c1](https://github.com/m3idnotfree/twitch_highway/commit/78de1c11ec53cb5c6e363a9abc6b79312605b905))
- **(goals)** Implement Twitch Goals API endpoints([b296602](https://github.com/m3idnotfree/twitch_highway/commit/b2966025313050fc24f603b2bfa38886d3de17c6))
- **(games)** Implement Twitch Games API endpoints([5cc3cb5](https://github.com/m3idnotfree/twitch_highway/commit/5cc3cb579d3aac7c79d91a1fbcb702c593a09807))
- **(extensions)** Implement Twitch Extensions API endpoints([0ffadd2](https://github.com/m3idnotfree/twitch_highway/commit/0ffadd2104cc8c71f38d5efc189ad9d477e6582e))
- **(entitlements)** Implement Twitch Entitlements API endpoints([6fdc48f](https://github.com/m3idnotfree/twitch_highway/commit/6fdc48f61b90a63fa3177fc06e124afb56bd4ad6))
- **(schedule)** Implement Twitch Schedule API endpoints([c0b5f6f](https://github.com/m3idnotfree/twitch_highway/commit/c0b5f6f1a9696703b488b3c9cd700b63c9887c08))
- **(guest-star)** Implement Twitch Guest Star API endpoints([859badc](https://github.com/m3idnotfree/twitch_highway/commit/859badcc8a74582b722fd7ef7b7a6f9d1e941316))
- **(search)** Implement Twitch Search API endpoints([5257678](https://github.com/m3idnotfree/twitch_highway/commit/5257678fed2d59b0af97d302447b30c06ed58563))
- **(flag)** Add full features([4fc9076](https://github.com/m3idnotfree/twitch_highway/commit/4fc907689d9a92347f409702d8bcd52a1f0706f4))
- **(core)** Improve request/response([ff31373](https://github.com/m3idnotfree/twitch_highway/commit/ff31373b39c4e2154dafd3407bd11aaef8d88f09))
- **(analytics)** Restructure Request query([0c8e0b6](https://github.com/m3idnotfree/twitch_highway/commit/0c8e0b6bf43093d927a89dc5819dcc468ae9e736))
- **(queryparams)** Add date_opt([d11af10](https://github.com/m3idnotfree/twitch_highway/commit/d11af10ee5868aac939274e4a79c89455b88de51))
- **(chat)** Implement Send Chat Cnnouncement and Send A Shoutout endpoints([e4f706e](https://github.com/m3idnotfree/twitch_highway/commit/e4f706ebf66fb1d4a097d3de65b95ff4fabbce89))

### Bug Fixes

- **(urlbuilder)** Change query parameter([090c5ef](https://github.com/m3idnotfree/twitch_highway/commit/090c5ef07b77330c2ae7064894e82d8d6ea64d7b))

### Refactor

- **(macros)** Remove Into trait([32e1a90](https://github.com/m3idnotfree/twitch_highway/commit/32e1a90b13425058f7efd62e84f7ed5483a6958a))
- **(types)** Fix new type serialize and deserialize([b68d5a8](https://github.com/m3idnotfree/twitch_highway/commit/b68d5a8e3b19cd1e86fe320b30aa537806fb2182))
- **(bits)** Use newtype BroadcasterId and UserId([0175461](https://github.com/m3idnotfree/twitch_highway/commit/0175461866e81bed156926f90600cc7521cd0ecf))
- **(chat)** Use newtype pattern([1a8b5b2](https://github.com/m3idnotfree/twitch_highway/commit/1a8b5b2462620e764de05ce2d6f5d39480b7d1f3))
- **(user)** Use newtype pattern([7a1c318](https://github.com/m3idnotfree/twitch_highway/commit/7a1c318a8495124f2d2d624834ba0e921311e0d3))
- **(request)** AsBody to RequestBody([2a5db34](https://github.com/m3idnotfree/twitch_highway/commit/2a5db34b259bb264ff895129ee8c0623fcfa62e7))
- **(types)** Move Cost struct to types directory([c772cfd](https://github.com/m3idnotfree/twitch_highway/commit/c772cfdb36808bfc1f8f17823a9ba1beee699887))
- **(request)** Reorganize request module([9615798](https://github.com/m3idnotfree/twitch_highway/commit/9615798433b2e2c632ec9deadfcfd75a3ce2e334))
- **(games)** Extract request types([6a8cd88](https://github.com/m3idnotfree/twitch_highway/commit/6a8cd88ce5eb6407d5a8931bd24bc36d43c0b182))
- **(moderation)** Restructure([ade4878](https://github.com/m3idnotfree/twitch_highway/commit/ade487897cfb124d3fb45999af5b7456401e16ce))
- **(subscriptions)** Restructure([b2ef3f3](https://github.com/m3idnotfree/twitch_highway/commit/b2ef3f395e68bfad9314452feef5b93a0243ff19))
- **(teams)** Restructure([dbf30ab](https://github.com/m3idnotfree/twitch_highway/commit/dbf30ab408a82236a9ebde13e7e6cbd8d722ca4e))
- **(ccls)** Separate request types([a1b14f1](https://github.com/m3idnotfree/twitch_highway/commit/a1b14f11a69b070119d5c06537be9883c863eb92))
- **(macros)** Remove unnecessary part([78e570f](https://github.com/m3idnotfree/twitch_highway/commit/78e570f0e824d547f058a5e6dbc7b631c28d056f))
- **(analytics)** Query push DateTime([41bf06e](https://github.com/m3idnotfree/twitch_highway/commit/41bf06eeee8d218da4c120d6562e967edbb8d406))
- **(bits)** Bits Leaderboard Request required DateTime([3fd0da2](https://github.com/m3idnotfree/twitch_highway/commit/3fd0da206d7dc672c99e07a9329354a7101c0627))
- **(channel-points)** Add RequiredBody instead of Value([fa4d73c](https://github.com/m3idnotfree/twitch_highway/commit/fa4d73c8dd1d1bc84f74bf323c60b992fdaf55ce))
- **(chanels)** Use macros([0d041ba](https://github.com/m3idnotfree/twitch_highway/commit/0d041ba02bf7c8ff8e1cc21347c7436795e78f1b))
- **(chat)** Add Send chat announcement Request body struct([5b774a4](https://github.com/m3idnotfree/twitch_highway/commit/5b774a4db69124ab6b87cb63a9021208f99caff7))
- **(entitlements)** Add impl_body field([d62c04d](https://github.com/m3idnotfree/twitch_highway/commit/d62c04d847284b210c6d40a413a8b93fabdb785c))
- **(extensions)** Update macros([f86a65c](https://github.com/m3idnotfree/twitch_highway/commit/f86a65cb22c4dfffefbc8f98d259aab7c9c39466))
- **(moderation)** Update macro([1b863b0](https://github.com/m3idnotfree/twitch_highway/commit/1b863b03d2272c7d4a1074baa7514ef5354f8c6b))
- **(whispers)** &str to impl Into<String>([b2c4c19](https://github.com/m3idnotfree/twitch_highway/commit/b2c4c191c0875c70f5b46be18ffa9bfdecfaabea))
- **(module)** Remove re-export of chrono modules([6012b80](https://github.com/m3idnotfree/twitch_highway/commit/6012b8034172c7a10449049343c16779d6e8d38a))
- **(users)** &str to UserId([b5c225c](https://github.com/m3idnotfree/twitch_highway/commit/b5c225cbbe637f091b55d941a692aa7b99921a04))
- **(games)** Extract Category struct([edf234c](https://github.com/m3idnotfree/twitch_highway/commit/edf234c473e9af21c2a546377f61824fe10d7fb3))
- **(types)** Use types::Category([2ac9b56](https://github.com/m3idnotfree/twitch_highway/commit/2ac9b561c2ee0d976046b076f0806eec7bf6b42b))
- **(subscriptions)** Rename struct UserSubscription to Subscription([8143296](https://github.com/m3idnotfree/twitch_highway/commit/8143296caf13561d6aaf3e1873c6b0eb7970a8cb))
- **(test)** Restructure module path([036524c](https://github.com/m3idnotfree/twitch_highway/commit/036524c04e9bf7d9a173b35bf1974ca13afca12e))
- [**breaking**] Remove eventsub module([3415632](https://github.com/m3idnotfree/twitch_highway/commit/3415632ceb182356e0262b479f3c0782fe29ac7a))
- **(channel_points)** [**breaking**] Rename flag '-'(dash) instead of '\_'(under dash)([1a58a7f](https://github.com/m3idnotfree/twitch_highway/commit/1a58a7f5c61e8e3e1b6bdc3dc58be8cbd0c34170))

### Documentation

- Add endpoint link([c1e2448](https://github.com/m3idnotfree/twitch_highway/commit/c1e2448dbb8ba89cd186ca587152a850e20a1506))
- Update([44651f3](https://github.com/m3idnotfree/twitch_highway/commit/44651f31b233198228b1fca6237c159b99c9a3b1))
- Update([dfc83d5](https://github.com/m3idnotfree/twitch_highway/commit/dfc83d5287e28a2b472ed2e89676190ff2a5e6b5))

### Testing

- **(users)** Mock_server([32fe394](https://github.com/m3idnotfree/twitch_highway/commit/32fe394f7323e5b6ab328c4b5e74fe561ce95787))
- **(predictions)** Update([aac2d84](https://github.com/m3idnotfree/twitch_highway/commit/aac2d8477297a76ba7432670e471f65578df446c))
- **(whispers)** Add mock_server test([43d34c3](https://github.com/m3idnotfree/twitch_highway/commit/43d34c37889f03f06b3243fa47c53f7db1cb14ca))
- **(videos)** Add mock_server test([e979bfa](https://github.com/m3idnotfree/twitch_highway/commit/e979bfa348bad666444b6e8d3e6d4b73c8e93322))
- **(ads)** Add mock_server test([194dde3](https://github.com/m3idnotfree/twitch_highway/commit/194dde36322bc9aebe99e18011cccc73b48b210b))
- **(analytics)** Add mock_server test([8086106](https://github.com/m3idnotfree/twitch_highway/commit/8086106334c5f24d1355692629fdfe03fe60ca96))
- **(bits)** Add mock_server test([81ee10b](https://github.com/m3idnotfree/twitch_highway/commit/81ee10b1f96f72655dbfd9e199bad5fcf7f09a70))
- **(ccls)** Add mock_server test([8d5e5e1](https://github.com/m3idnotfree/twitch_highway/commit/8d5e5e103cf7121df60770fc170ced9a7fa22384))
- **(channel_points)** Add mock_server test([aaaf3fb](https://github.com/m3idnotfree/twitch_highway/commit/aaaf3fba361f8aa8043b55b7b757bedb2f9b24ab))
- **(channels)** Add mock_server test([751904b](https://github.com/m3idnotfree/twitch_highway/commit/751904b933cbfa9082888d172e2c9ed45b5863e1))
- **(charity)** Add mock_server test([1376ae3](https://github.com/m3idnotfree/twitch_highway/commit/1376ae3d01df5bb5cc2a1ac0adf18318aaf32b75))
- **(chat)** Upgrade([85e22de](https://github.com/m3idnotfree/twitch_highway/commit/85e22dea12224b042a5fd6d8e3b359259fa75373))
- **(clips)** Add mock_server test([83e3362](https://github.com/m3idnotfree/twitch_highway/commit/83e3362982c68ac587632113dfa8993d7e71ceb5))
- **(entitlements)** Upgrade([9216f0a](https://github.com/m3idnotfree/twitch_highway/commit/9216f0a433e33567faf17e2916f45a43ba288231))
- **(extensions)** Update([c6944ac](https://github.com/m3idnotfree/twitch_highway/commit/c6944acdc23fab20a62d0f523f0ecd2dc8b85f1f))
- **(goals)** Update([fb132fc](https://github.com/m3idnotfree/twitch_highway/commit/fb132fc877f9e281ad9d68abedf49c63959aea7c))
- **(guest-star)** Update([a946846](https://github.com/m3idnotfree/twitch_highway/commit/a946846ef9dda9db4330b158eb327cbbdb5698f1))
- **(hype-train)** Add mock_server test([7005917](https://github.com/m3idnotfree/twitch_highway/commit/70059172acf3c192f179ad282d4a4c0657dab490))
- **(polls)** Add mock_server test([19e332e](https://github.com/m3idnotfree/twitch_highway/commit/19e332ec529d595f93e58586cf7d7cbd110b1aad))
- **(predictions)** Add mock_server test([342e31c](https://github.com/m3idnotfree/twitch_highway/commit/342e31cd8fe0b2ba2debbf5eb5187716056005e1))
- **(raid)** Add mock_server test([777d238](https://github.com/m3idnotfree/twitch_highway/commit/777d2382d56948ce827f3c4ce5d9d17e11f7ee7f))
- **(schedule)** Update([e41cae4](https://github.com/m3idnotfree/twitch_highway/commit/e41cae4ef85157e24777ccc66057105bebe3d77c))
- **(search)** Add mock_server test([fc01e15](https://github.com/m3idnotfree/twitch_highway/commit/fc01e1578ea9c707e802084ad9d3bcc033adc3f3))
- **(streams)** Add mock_server test([dc0b5b3](https://github.com/m3idnotfree/twitch_highway/commit/dc0b5b37fcfeac9dce85664565e024798b428d42))
- **(users)** Update([d037bf7](https://github.com/m3idnotfree/twitch_highway/commit/d037bf7843d789c1d1edc507d8084fd20355c09a))
- Update([86e0a71](https://github.com/m3idnotfree/twitch_highway/commit/86e0a71154c21797a2a0b1040785e8b2112436ea))
- **(test_url)** Add helper functions([962ba00](https://github.com/m3idnotfree/twitch_highway/commit/962ba009e7bffdf9f6e155dccf54e3ca3d8b89d0))

### Miscellaneous

- **(dep)** Upgrade([a59deeb](https://github.com/m3idnotfree/twitch_highway/commit/a59deebd92034d0508641e3f11ff1bcac976d400))
- **(gitignore)** Update([4e624cc](https://github.com/m3idnotfree/twitch_highway/commit/4e624ccb4f8f8145657a805890f954e231c3327f))
- **(dep)** Update([3216ca0](https://github.com/m3idnotfree/twitch_highway/commit/3216ca0fdb74424deb558752fce540bcc1f31b7e))

## [0.0.10](https://github.com/m3idnotfree/twitch_highway/compare/v0.0.9..v0.0.10) - 2025-01-02

### Features

- **(core)** Introduce TwitchAPI base architecture([3363a46](https://github.com/m3idnotfree/twitch_highway/commit/3363a46675ccdf92b74d1dc0c6dc5a4b620c2333))
- **(api)** UserAPI trait([cbeb9e6](https://github.com/m3idnotfree/twitch_highway/commit/cbeb9e6276bab4ee05a7e89171f53352326935e1))
- **(api)** ChatAPI trait([9e85176](https://github.com/m3idnotfree/twitch_highway/commit/9e85176db7f3e6824b4512e23d8f8c663682bb18))
- **(api)** EventSubAPI([ff910b5](https://github.com/m3idnotfree/twitch_highway/commit/ff910b5a02b0bd749cdd1421b6f732ed88ac9e2c))

### Refactor

- **(test_url)** Improve TestUrl and TestUrlHold([b96ba55](https://github.com/m3idnotfree/twitch_highway/commit/b96ba556d6ba3bdf2708a62259ec48011dab62af))
- **(core)** Optimize URL builder implementation([72bdbaf](https://github.com/m3idnotfree/twitch_highway/commit/72bdbafeaa2eacd4a19059de98e9b34d6dbbc33d))
- **(deps)** Migrate serde utilities to external crate([379b563](https://github.com/m3idnotfree/twitch_highway/commit/379b563eae70108b61f6f4c9147a1d45f8de111e))

### Documentation

- Update([e30897c](https://github.com/m3idnotfree/twitch_highway/commit/e30897cb7f4e2a6599897370fb325bef5d6c65a7))

### Testing

- Update([c64b2a4](https://github.com/m3idnotfree/twitch_highway/commit/c64b2a4b05666c26137421752452e008cda30616))
- Update([f1e0c41](https://github.com/m3idnotfree/twitch_highway/commit/f1e0c41324402ead4d7111851179bcfa1e474a97))

## [0.0.9](https://github.com/m3idnotfree/twitch_highway/compare/v0.0.8..v0.0.9) - 2024-12-24

### Features

- **(types)** Implement version-aware subscription type deserialization macro([2481d57](https://github.com/m3idnotfree/twitch_highway/commit/2481d5747a0b984e44ae0540d2b4ef3710cf18af))

### Miscellaneous

- Remove unused module([09e2d0b](https://github.com/m3idnotfree/twitch_highway/commit/09e2d0b1fbdd32cea5afdd40642f0644ae2a3cc3))

## [0.0.8](https://github.com/m3idnotfree/twitch_highway/compare/v0.0.7..v0.0.8) - 2024-12-24

### Refactor

- **(macros)** Make macros module private([c919390](https://github.com/m3idnotfree/twitch_highway/commit/c91939051c9758ffecc3ad81ea67d0afc3b4e64a))

## [0.0.7](https://github.com/m3idnotfree/twitch_highway/compare/v0.0.6..v0.0.7) - 2024-12-24

### Features

- **(time)** Re-export Datelike and Timelike for RFC3339 format handling([cda908a](https://github.com/m3idnotfree/twitch_highway/commit/cda908a10e70ec20c3e4be6906b88ae74bd6ce79))

## [0.0.6](https://github.com/m3idnotfree/twitch_highway/compare/v0.0.5..v0.0.6) - 2024-12-23

### Refactor

- **(tests)** Relocate get_user tests to module directory([e88c4e8](https://github.com/m3idnotfree/twitch_highway/commit/e88c4e8a97a8c1ddcf405a9efb9ef569f7a1e5f6))
- **(tests)** Move test macros to support directory([fcedfed](https://github.com/m3idnotfree/twitch_highway/commit/fcedfed4e96c26bde38511c466f9f18fd6d811f5))
- **(users)** Restructure module visibility and organization([98af3d8](https://github.com/m3idnotfree/twitch_highway/commit/98af3d86f3303996660752d71ca12b2f5d2e8105))
- **(types)** Restructure module visibility and organization([f90e376](https://github.com/m3idnotfree/twitch_highway/commit/f90e376fb7e160dbd248a53ac43cfa2ec9c11890))
- **(eventsub)** Restructure module visibility and organization publicly([fe1f345](https://github.com/m3idnotfree/twitch_highway/commit/fe1f3459688671a77e68a8d4aea48d89668d0340))
- **(chat)** Restructure module visibility and organization publicly([c5c7e6d](https://github.com/m3idnotfree/twitch_highway/commit/c5c7e6d7c78f6781e2b4c6baf132714ddee055cf))
- **(features)** Add feature flag conditions([ffab4fa](https://github.com/m3idnotfree/twitch_highway/commit/ffab4fa64725913fb5662d5aaf4c56b4e7477c98))

## [0.0.5](https://github.com/m3idnotfree/twitch_highway/compare/v0.0.4..v0.0.5) - 2024-12-23

### Features

- **(types)** Add pagination support for API responses([c3198e1](https://github.com/m3idnotfree/twitch_highway/commit/c3198e1ec347a9c914a30ff2b1dc92fbdc17ede8))
- **(chat)** Implement Get Chatters endpoint([5d666ed](https://github.com/m3idnotfree/twitch_highway/commit/5d666ed0b5e4a61a5a63f2dd3c084d05c6dc4d46))
- **(chat)** Add get_chatters method to ChatAPI([d973050](https://github.com/m3idnotfree/twitch_highway/commit/d9730505aab92122b49edb2adac4955b3a08dabf))
- **(eventsub)** Add delete method to EventSub([d132cbc](https://github.com/m3idnotfree/twitch_highway/commit/d132cbc8030a114d5b5f7b80396ef758f7018641))
- **(chat)** Add get_chat_settings method to ChatAPI([8f4ba22](https://github.com/m3idnotfree/twitch_highway/commit/8f4ba22769bc5bc9c447c9214cf78003775fe89b))
- **(chat)** Add get_shared_chat_session method([b0f5690](https://github.com/m3idnotfree/twitch_highway/commit/b0f569047a4a5d8f532f359d964a4d673b0de243))
- **(chat)** Add send_chat_message method([4d01379](https://github.com/m3idnotfree/twitch_highway/commit/4d0137996b7c92516d003fba05c022c737462474))
- **(eventsub)** Add get method([3ec118e](https://github.com/m3idnotfree/twitch_highway/commit/3ec118eb780cf28b070293a5bbf665662a6e039d))
- **(chat)** Send chat message([7a9b77e](https://github.com/m3idnotfree/twitch_highway/commit/7a9b77ebf7d0d5cff7c314a3e1b0b6de2ef1e53c))
- Add TestUrl trait and test URL handling utilities([0d4b964](https://github.com/m3idnotfree/twitch_highway/commit/0d4b9645677b42c8e213c6219ac88d3407c8a4e6))
- Add custom serde serialization/deserialization utilities([e499adb](https://github.com/m3idnotfree/twitch_highway/commit/e499adb88b898adfd6f94c026fcdf07374a7bb49))
- **(macro)** Add impl_endpoint([10fea50](https://github.com/m3idnotfree/twitch_highway/commit/10fea509400cb823c95cc4c53a114337545a8f46))
- **(macro)** Add impl_default_header and impl_api_request_method([c935dd0](https://github.com/m3idnotfree/twitch_highway/commit/c935dd0420fdfff463e63235e3baf6e187ca0f79))
- **(types)** Support serialize for SubscriptionTypes([d5e9968](https://github.com/m3idnotfree/twitch_highway/commit/d5e996817afc5bce75cdeb7706d7a7f8da9f10a8))
- **(eventsub)** Add Transport type for EventSub([7bac16b](https://github.com/m3idnotfree/twitch_highway/commit/7bac16b66bd1cd882280dfe2f77b69f4d089af94))
- **(eventsub)** SubscriptionTypes support deserialization([30d8ffc](https://github.com/m3idnotfree/twitch_highway/commit/30d8ffcb7ef43e72f9972f10ccd0f2d638679c64))
- **(features)** Add types features flag([7eeb601](https://github.com/m3idnotfree/twitch_highway/commit/7eeb6010257727694b62ea22204070f6b8b240c8))

### Bug Fixes

- Remove pub field([aabab24](https://github.com/m3idnotfree/twitch_highway/commit/aabab241e3d89043c03bac8766e0a2100e453b9a))

### Refactor

- **(chat)** Change URL setters to use mutable references([e33a66a](https://github.com/m3idnotfree/twitch_highway/commit/e33a66ade5250383dc732e2090c0c141f5d3ba26))
- **(chat)** Optimize URL handling with static initialization([f323a58](https://github.com/m3idnotfree/twitch_highway/commit/f323a5853d160725fb5edd8b34a5c6453da41b39))
- **(chat)** Make URL setters generic over Into<String>([0e4b9ba](https://github.com/m3idnotfree/twitch_highway/commit/0e4b9ba12ad6f62cab2db8420037112f24598846))
- **(chat,eventsub)** Remove lifetime parameters and optimize string handling([652c591](https://github.com/m3idnotfree/twitch_highway/commit/652c5915adfdbbada950e88d635b6624e056f75e))
- Extract types into dedicated module([bf98c6c](https://github.com/m3idnotfree/twitch_highway/commit/bf98c6cc5d57332ed41f4d39356d51228a3c83a4))
- **(users)** Optimize URL handling with static initialization([0b7bad5](https://github.com/m3idnotfree/twitch_highway/commit/0b7bad53015da511ac2a4641918ac14ddf3ff8ae))
- **(eventsub)** Rename EventSub to EventSubAPI([24cd9e9](https://github.com/m3idnotfree/twitch_highway/commit/24cd9e9918b6dcab5acc490074248629a8d694f9))
- (url) optimize URL handling([3e6e6a1](https://github.com/m3idnotfree/twitch_highway/commit/3e6e6a14e5291e54523659c6a545536f9c69ceb4))
- **(chat)** Rename set to set_moderator_id([f594097](https://github.com/m3idnotfree/twitch_highway/commit/f5940976d8207cb1745824dbe457e23dcd5ecb2f))
- **(users)** Improve GetUsers implementation and User types([1a7920a](https://github.com/m3idnotfree/twitch_highway/commit/1a7920a9ec7e6b0e250b7f136838bf9239ec765b))
- **(tests)** Move test util to support module([64ca2e2](https://github.com/m3idnotfree/twitch_highway/commit/64ca2e2e7d3e8ec3d868c9884d102704bfff2c30))
- **(users)** Serialize None email as empty string([f14a3ab](https://github.com/m3idnotfree/twitch_highway/commit/f14a3ab6cc7a21c45d63280f13f832f73c157223))
- **(chat)** Restructure GetChatSetting for test support([a342e26](https://github.com/m3idnotfree/twitch_highway/commit/a342e262cdd71d2a584ae60b43db7a8b0e0bd173))
- **(chat)** Restructure GetChatters for test support([6c1b9f7](https://github.com/m3idnotfree/twitch_highway/commit/6c1b9f78d1e2f16bb71e31a2653c1a16723a4202))
- Simplify token handling([5f23eae](https://github.com/m3idnotfree/twitch_highway/commit/5f23eaee986f68775969915be7fb9aead55db524))
- **(macros)** Remove unnecessary Arc warpping from tokens([379cbf0](https://github.com/m3idnotfree/twitch_highway/commit/379cbf08a0d70f759879039e9d5abd4a4a44f167))
- **(users)** Migrate GetUsers to impl_endpoint macro([d5deb38](https://github.com/m3idnotfree/twitch_highway/commit/d5deb38d3e9c19c8aba1eda7bd5084152ff0400c))
- **(macro)** Rename impl_default_header to impl_api_request_header([609dc51](https://github.com/m3idnotfree/twitch_highway/commit/609dc514679dcda87486e99079b6dcf1c537d240))
- **(users)** Migrate struct to impl_endpoint macro([ad5a87b](https://github.com/m3idnotfree/twitch_highway/commit/ad5a87bb41d7322bddd8d248911f4a806cb824b5))
- **(chat)** Migrate struct to impl_endpoint macro([18174ad](https://github.com/m3idnotfree/twitch_highway/commit/18174ad8af56d1d4387fe284129844790af50101))
- **(eventsub)** Migrate Transport type([1ce2f40](https://github.com/m3idnotfree/twitch_highway/commit/1ce2f40a7c3fd91e12e3f259519608248a65613d))

### Documentation

- Update API endpoints status and roadmap([36f87ba](https://github.com/m3idnotfree/twitch_highway/commit/36f87baa5b57e6b90bd3b001a6b521d7aeca233c))
- **(chat)** Add link get shared chat session([9fbcfb5](https://github.com/m3idnotfree/twitch_highway/commit/9fbcfb5cf2b4ef685970d477e499e5b6810bb06e))
- Update API Implementation([6bb3e57](https://github.com/m3idnotfree/twitch_highway/commit/6bb3e576dc13b2f49f6d29be17abb251fe4aad36))

### Testing

- **(macros)** Add expect_response_json test helper macro([d97ae19](https://github.com/m3idnotfree/twitch_highway/commit/d97ae192b2c7230fd8d72cadc20e4ab8aa108c84))
- **(deps)** Add serde_json([6507616](https://github.com/m3idnotfree/twitch_highway/commit/6507616580318c234bef30acf8ae3fc4dbb1b96a))

### Miscellaneous

- **(macros)** Add APIRequest([7098424](https://github.com/m3idnotfree/twitch_highway/commit/7098424ce7fda102c0294177167f4e34246418fc))
- Remove unused import([6d5ca2f](https://github.com/m3idnotfree/twitch_highway/commit/6d5ca2f041570f3b21342f71e784c3a63484b752))
- Expose serde_util and test_url modules([b780b84](https://github.com/m3idnotfree/twitch_highway/commit/b780b84ced28423b911d66eeccad17de0059ca96))
- **(users)** Forgot import serialize_none_as_empty_string([e8d1b9d](https://github.com/m3idnotfree/twitch_highway/commit/e8d1b9dc4629a97fe3ca27aba6de091ce49ef209))
- **(lib)** Expose impl_endpoint module internally([a4e767d](https://github.com/m3idnotfree/twitch_highway/commit/a4e767da481c34a3ab1c379a0c33bc106dfafaa9))
- **(git)** Update gitignore([4e05289](https://github.com/m3idnotfree/twitch_highway/commit/4e052895939449b76559a58c9676d6e6c594e3a6))

## [0.0.4](https://github.com/m3idnotfree/twitch_highway/compare/v0.0.3..v0.0.4) - 2024-11-11

### Features

- UserAPI([712a51d](https://github.com/m3idnotfree/twitch_highway/commit/712a51d54d20cfe06458e27c50ec7c731c7760a7))
- ChatAPI EmoteAPI([c09a2ee](https://github.com/m3idnotfree/twitch_highway/commit/c09a2ee00f97f12692ceadfe645d579f835f8ee6))
- ChatAPI, BadgeAPI([6950184](https://github.com/m3idnotfree/twitch_highway/commit/6950184a7bfd87bf805f1bee6cee3e7dbe3c095f))
- EventSub([a688244](https://github.com/m3idnotfree/twitch_highway/commit/a688244d402cd5e5a181343358156df3852d2589))
- Error([b2924c3](https://github.com/m3idnotfree/twitch_highway/commit/b2924c3f0fdd2895cc6adfc57fa032ff3156419a))
- Features chat, eventsub, users, full([aa9835d](https://github.com/m3idnotfree/twitch_highway/commit/aa9835dd88ed559ada184eaa94571d9f8f69d7f2))
- Test macros([d800db0](https://github.com/m3idnotfree/twitch_highway/commit/d800db04f7c4975b9e12976327b0e47a9fe57fdd))

### Refactor

- [**breaking**] Remove moderation and utils modules([cf2d547](https://github.com/m3idnotfree/twitch_highway/commit/cf2d547e8e01bea93e396af793f5ff7364883866))

### Testing

- Update([30958a2](https://github.com/m3idnotfree/twitch_highway/commit/30958a2e756395135936c9b5878ba48a6281ec68))

### Miscellaneous

- Update dependencies([27848b7](https://github.com/m3idnotfree/twitch_highway/commit/27848b77cac27f83aa3ce0a56119444a84521caa))
- Update license([957c876](https://github.com/m3idnotfree/twitch_highway/commit/957c8768908862878756ef384f6e8d663f8f923a))

## [0.0.2] - 2024-03-02

### Features

- **(derive)** Add PartialEq([a5d4380](https://github.com/m3idnotfree/twitch_highway/commit/a5d43801ad0deb4781c2494d3edd4f3607baa4bf))

### Miscellaneous

- First commit([00531d9](https://github.com/m3idnotfree/twitch_highway/commit/00531d927286f3afd9eacc5c4bb845c7ee2dadff))
- Cargo.toml add repository([dbbc0ce](https://github.com/m3idnotfree/twitch_highway/commit/dbbc0cef821d9261f79949eaff03d5e2bb5b615b))
