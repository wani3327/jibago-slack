macro_rules! string {
    ($name: ident) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
        pub struct $name(String);

        impl Into<String> for $name {
            fn into(self) -> String {
                self.0
            }
        }

        impl Into<$name> for String {
            fn into(self) -> $name {
                $name(self)
            }
        }
    };
}

string! {UserId}
string! {ChannelId}

#[derive(serde::Deserialize)]
pub(crate) struct Event<T> {
    pub token: String,
    pub team_id: String,
    pub context_team_id: String,
    pub context_enterprise_id: Option<String>,
    pub api_app_id: String,
    pub event: T,
    #[serde(rename = "type")]
    pub r#type: String,
    pub event_id: String,
    pub event_time: i32,
    // pub authorizations: Unknown,
    pub is_ext_shared_channel: bool,
    pub event_context: String,
}



#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) struct Message {
    pub user: UserId,
    #[serde(rename = "type")]
    pub r#type: String,
    pub ts: String,
    pub client_msg_id: String,
    pub text: String,
    pub team: String,
    pub blocks: Vec<Block>,
    pub channel: ChannelId,
    pub event_ts: String,
    pub channel_type: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) struct Block {
    pub r#type: String,
    pub block_id: String,
    // pub elements: Vec<Element>,
}

pub(crate) struct Element {

}