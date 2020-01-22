// LineBotConfigの構造体の定義、JSONパース追加
#[derive(Serialize, Deserialize, Clone)]
pub struct LineBotConfig {
    channel_secret: String,
    channel_token:  String,
}

// 引数にLINE秘密鍵、トークンを受け、LineBotConfigを返す
impl LineBotConfig {
    pub fn new(channel_secret: &str, channel_token: &str) -> LineBotConfig{
        LineBotConfig {
            channel_secret: String::from(channel_secret),
            channel_token:  String::from(channel_token)
        }
    }

    // channel_secretの複製を返す
    pub fn get_channel_secret(&self) -> String{
        self.channel_secret.clone()
    }
    // channel_secret_tokenの複製を返す
    pub fn get_channel_token(&self) -> String{
        self.channel_token.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Rectangle {
    pub x:      u64,
    pub y:      u64,
    pub width:  u64,
    pub height: u64
}

