#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let domain_sid = "your domain sid";
    let sid = "your sid";
    let response = client
        .fetch_sip_credential_list_mapping(account_sid, domain_sid, sid)
        .await
        .unwrap();
    println!("{:#?}", response);
}