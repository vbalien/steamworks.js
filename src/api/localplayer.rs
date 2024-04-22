use napi::bindgen_prelude::BigInt;
use napi_derive::napi;
use steamworks::SteamId;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct PlayerSteamId {
    pub steam_id64: BigInt,
    pub steam_id32: String,
    pub account_id: u32,
}

impl PlayerSteamId {
    pub(crate) fn from_steamid(steam_id: SteamId) -> Self {
        Self {
            steam_id64: steam_id.raw().into(),
            steam_id32: steam_id.steamid32(),
            account_id: steam_id.account_id().raw(),
        }
    }
}

#[napi]
pub mod localplayer {
    use napi::bindgen_prelude::Buffer;

    use super::PlayerSteamId;

    #[napi]
    pub fn get_steam_id() -> PlayerSteamId {
        let client = crate::client::get_client();
        let steam_id = client.user().steam_id();
        PlayerSteamId::from_steamid(steam_id)
    }

    #[napi]
    pub fn get_name() -> String {
        let client = crate::client::get_client();
        client.friends().name()
    }

    #[napi]
    pub fn get_small_avatar() -> Option<Buffer> {
        let client = crate::client::get_client();
        let steam_id = client.user().steam_id();
        let avatar_buf = client.friends().get_friend(steam_id).small_avatar();
        avatar_buf.map(|buf| buf.clone().into())
    }

    #[napi]
    pub fn get_medium_avatar() -> Option<Buffer> {
        let client = crate::client::get_client();
        let steam_id = client.user().steam_id();
        let avatar_buf = client.friends().get_friend(steam_id).medium_avatar();
        avatar_buf.map(|buf| buf.clone().into())
    }

    #[napi]
    pub fn get_large_avatar() -> Option<Buffer> {
        let client = crate::client::get_client();
        let steam_id = client.user().steam_id();
        let avatar_buf = client.friends().get_friend(steam_id).large_avatar();
        avatar_buf.map(|buf| buf.clone().into())
    }

    #[napi]
    pub fn get_level() -> u32 {
        let client = crate::client::get_client();
        client.user().level()
    }

    /// @returns the 2 digit ISO 3166-1-alpha-2 format country code which client is running in, e.g. "US" or "UK".
    #[napi]
    pub fn get_ip_country() -> String {
        let client = crate::client::get_client();
        client.utils().ip_country()
    }

    #[napi]
    pub fn set_rich_presence(key: String, value: Option<String>) {
        let client = crate::client::get_client();
        client.friends().set_rich_presence(&key, value.as_deref());
    }
}
