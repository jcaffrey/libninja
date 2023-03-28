#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let recording_sid = "your recording sid";
    let sid = "your sid";
    let response = client
        .fetch_recording_transcription(account_sid, recording_sid, sid)
        .await
        .unwrap();
    println!("{:#?}", response);
}