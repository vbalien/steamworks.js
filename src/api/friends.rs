use napi_derive::napi;

#[napi]
pub mod friends {
    use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
    use steamworks::{FriendFlags, SteamId};

    use crate::api::localplayer::PlayerSteamId;

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
    pub struct Friend {
        steam_id: PlayerSteamId,
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

        #[napi]
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
