use napi_derive::napi;

#[napi]
pub mod friends {
    use steamworks::FriendFlags;

    use crate::api::localplayer::PlayerSteamId;

    #[napi]
    pub async fn get_friends(flags: u16) -> Vec<PlayerSteamId> {
        let client = crate::client::get_client();
        let friend_flags = FriendFlags::from_bits_truncate(flags);
        client
            .friends()
            .get_friends(friend_flags)
            .iter()
            .map(|friend| PlayerSteamId::from_steamid(friend.id()))
            .collect()
    }
}
