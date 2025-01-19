use serde::Serialize;

request_struct!(
    #[derive(Serialize)]
    SendWhisperBody {
        required {
           message: String
         }
    };
    impl_body: true
);
