
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiV2010AccountSipSipCredentialList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl std::fmt::Display for ApiV2010AccountSipSipCredentialList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}