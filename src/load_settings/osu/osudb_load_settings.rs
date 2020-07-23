use crate::load_settings::osu::beatmap_load_settings::BeatmapLoadSettings;
use crate::masks::osu_mask::OsuDbMask;
use std::default::Default;
use structopt::StructOpt;

#[derive(Clone, StructOpt)]
pub struct OsuDbLoadSettings {
    #[structopt(skip)]
    pub version: bool,
    #[structopt(skip)]
    pub folder_count: bool,
    #[structopt(skip)]
    pub account_unlocked: bool,
    #[structopt(skip)]
    pub account_unlock_date: bool,
    #[structopt(skip)]
    pub player_name: bool,
    #[structopt(skip)]
    pub number_of_beatmaps: bool,
    #[structopt(flatten)]
    pub beatmap_load_settings: BeatmapLoadSettings,
    #[structopt(skip)]
    pub unknown_short_or_permissions: bool,
}

impl OsuDbLoadSettings {
    pub fn set_from_mask(&mut self, mask: &OsuDbMask) {
        self.version |= mask.version;
        self.folder_count |= mask.folder_count;
        self.account_unlocked |= mask.account_unlocked;
        self.account_unlock_date |= mask.account_unlock_date;
        self.player_name |= mask.player_name;
        self.number_of_beatmaps |= mask.number_of_beatmaps;
        self.beatmap_load_settings.set_from_mask(&mask.beatmap_mask);
        self.unknown_short_or_permissions |= mask.unknown_short_or_permissions;
    }
}

impl Default for OsuDbLoadSettings {
    fn default() -> Self {
        OsuDbLoadSettings {
            version: bool::default(),
            folder_count: bool::default(),
            account_unlocked: bool::default(),
            account_unlock_date: bool::default(),
            player_name: bool::default(),
            number_of_beatmaps: bool::default(),
            beatmap_load_settings: BeatmapLoadSettings::default(),
            unknown_short_or_permissions: bool::default(),
        }
    }
}
