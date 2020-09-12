use crate::wrappers::structs::{
    UniqueNetId,
    SkillRating,
    SkillRank
};

pub fn is_syncing(player_id: UniqueNetId) -> bool {
    unsafe { IsSyncing(player_id) }
}

pub fn is_synced(player_id: UniqueNetId, playlist_id: i32) -> bool {
    unsafe { IsSynced(player_id, playlist_id) }
}

pub fn is_ranked(playlist_id: i32) -> bool {
    unsafe { IsRanked(playlist_id) }
}

pub fn get_player_skill_rating(player_id: UniqueNetId, playlist_id: i32) -> SkillRating {
    let mut result = SkillRating::new();
    let result_ptr = &mut result as *const SkillRating;

    unsafe { GetPlayerSkillRating(player_id, playlist_id, result_ptr) }
    result
}

pub fn get_player_rank(player_id: UniqueNetId, playlist_id: i32) -> SkillRank {
    let mut result = SkillRank::new();
    let result_ptr = &mut result as *const SkillRank;

    unsafe { GetPlayerRank(player_id, playlist_id, result_ptr) }
    result
}

pub fn get_player_mmr(player_id: UniqueNetId, playlist_id: i32) -> f32 {
    unsafe { GetPlayerMMR(player_id, playlist_id) }
}

pub fn calculate_mmr(skill_rating: SkillRating, disregard_placements: bool) -> f32 {
    let skill_rating = &skill_rating as *const SkillRating;
    unsafe { CalculateMMR(skill_rating, disregard_placements) }
}

pub fn get_current_playlist() -> i32 {
    unsafe { GetCurrentPlaylist() }
}

extern "C" {
    fn IsSyncing(playerID: UniqueNetId) -> bool;
    fn IsSynced(playerID: UniqueNetId, playlistID: i32) -> bool;
    fn IsRanked(playlistID: i32) -> bool;
    fn GetPlayerSkillRating(playerID: UniqueNetId, playlistID: i32, result: *const SkillRating);
    fn GetPlayerRank(playerID: UniqueNetId, playlistID: i32, result: *const SkillRank);
    fn GetPlayerMMR(playerID: UniqueNetId, playlistID: i32) -> f32;
    fn CalculateMMR(sr: *const SkillRating, disregardPlacements: bool) -> f32;
    fn GetCurrentPlaylist() -> i32;
}