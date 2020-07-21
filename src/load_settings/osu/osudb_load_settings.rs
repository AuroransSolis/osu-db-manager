use crate::load_settings::osu::beatmap_load_settings::BeatmapLoadSettings;
use crate::masks::osu_mask::OsuDbMask;
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
    pub unknown_short: bool,
}

impl OsuDbLoadSettings {
    fn load_all(&self) -> bool {
        self.version
            && self.folder_count
            && self.account_unlocked
            && self.account_unlock_date
            && self.player_name
            && self.unknown_short
            && self.beatmap_load_settings.load_all()
    }

    fn ignore_all(&self) -> bool {
        !self.version
            && !self.folder_count
            && !self.account_unlocked
            && !self.account_unlock_date
            && !self.player_name
            && !self.unknown_short
            && self.beatmap_load_settings.ignore_all()
    }

    fn is_partial(&self) -> bool {
        !self.version
            || !self.folder_count
            || !self.account_unlocked
            || !self.account_unlock_date
            || !self.player_name
            || !self.unknown_short
            || self.beatmap_load_settings.is_partial()
    }

    fn set_from_mask(&mut self, mask: &OsuDbMask) {
        self.version |= mask.version;
        self.folder_count |= mask.folder_count;
        self.account_unlocked |= mask.account_unlocked;
        self.account_unlock_date |= mask.account_unlock_date;
        self.player_name |= mask.player_name;
        self.number_of_beatmaps |= mask.number_of_beatmaps;
        self.beatmap_load_settings.set_from_mask(&mask.beatmap_mask);
        self.unknown_short |= mask.unknown_short;
    }
}
