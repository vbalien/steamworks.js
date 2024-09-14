use friends::FriendState;
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use steamworks::SteamId;

use crate::api::localplayer::PlayerSteamId;

#[napi]
pub struct Friend {
    pub(crate) steam_id: PlayerSteamId,
}
#[napi]
impl Friend {
    #[napi]
    pub fn id(&self) -> PlayerSteamId {
        self.steam_id.clone()
    }

    #[napi]
    pub fn name(&self) -> String {
        let client = crate::client::get_client();
        let friend = client
            .friends()
            .get_friend(SteamId::from_raw(self.steam_id.steam_id64.get_u64().1));
        friend.name()
    }

    #[napi(ts_return_type = "friends.FriendState")]
    pub fn state(&self) -> FriendState {
        let client = crate::client::get_client();
        let friend = client
            .friends()
            .get_friend(SteamId::from_raw(self.steam_id.steam_id64.get_u64().1));
        match friend.state() {
            steamworks::FriendState::Offline => FriendState::Offline,
            steamworks::FriendState::Online => FriendState::Online,
            steamworks::FriendState::Busy => FriendState::Busy,
            steamworks::FriendState::Away => FriendState::Away,
            steamworks::FriendState::Snooze => FriendState::Snooze,
            steamworks::FriendState::LookingToTrade => FriendState::LookingToTrade,
            steamworks::FriendState::LookingToPlay => FriendState::LookingToPlay,
        }
    }

    #[napi]
    pub fn get_small_avatar(&self) -> Option<Buffer> {
        let client = crate::client::get_client();
        let friend = client
            .friends()
            .get_friend(SteamId::from_raw(self.steam_id.steam_id64.get_u64().1));
        let avatar_buf = friend.small_avatar();
        avatar_buf.map(|buf| buf.clone().into())
    }

    #[napi]
    pub fn get_medium_avatar(&self) -> Option<Buffer> {
        let client = crate::client::get_client();
        let friend = client
            .friends()
            .get_friend(SteamId::from_raw(self.steam_id.steam_id64.get_u64().1));
        let avatar_buf = friend.medium_avatar();
        avatar_buf.map(|buf| buf.clone().into())
    }

    #[napi]
    pub fn get_large_avatar(&self) -> Option<Buffer> {
        let client = crate::client::get_client();
        let friend = client
            .friends()
            .get_friend(SteamId::from_raw(self.steam_id.steam_id64.get_u64().1));
        let avatar_buf = friend.large_avatar();
        avatar_buf.map(|buf| buf.clone().into())
    }
}

#[napi]
pub mod friends {
    use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
    use steamworks::FriendFlags;

    use crate::api::localplayer::PlayerSteamId;

    use super::Friend;

    #[napi]
    pub enum FriendState {
        Offline,
        Online,
        Busy,
        Away,
        Snooze,
        LookingToTrade,
        LookingToPlay,
    }

    #[napi]
    pub fn get_friends(flags: u16) -> Vec<Friend> {
        let client = crate::client::get_client();
        let friend_flags = FriendFlags::from_bits_truncate(flags);
        client
            .friends()
            .get_friends(friend_flags)
            .iter()
            .map(|friend| Friend {
                steam_id: PlayerSteamId::from_steamid(friend.id()),
            })
            .collect()
    }
}
