#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let response = client
        .list_usage_trigger(account_sid)
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .recurring("your recurring")
        .trigger_by("your trigger by")
        .usage_category("your usage category")
        .await
        .unwrap();
    println!("{:#?}", response);
}