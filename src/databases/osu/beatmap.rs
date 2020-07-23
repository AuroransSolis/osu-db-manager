use chrono::NaiveDate;

use crate::databases::osu::{primitives::*, versions::ReadVersionSpecificData};
use crate::deserialize_primitives::*;
use crate::read_error::{DbFileParseError, ParseErrorKind::*, ParseFileResult};

/// Beatmap struct according to documentation linked in README.
#[derive(Clone, Debug)]
pub struct Beatmap<'a> {
    pub entry_size: Option<i32>,
    pub artist_name: Option<&'a str>,
    pub artist_name_unicode: Option<&'a str>,
    pub song_title: Option<&'a str>,
    pub song_title_unicode: Option<&'a str>,
    pub creator_name: Option<&'a str>,
    pub difficulty: Option<&'a str>,
    pub audio_file_name: Option<&'a str>,
    pub md5_beatmap_hash: &'a str,
    pub dotosu_file_name: Option<&'a str>,
    pub ranked_status: RankedStatus,
    pub number_of_hitcircles: i16,
    pub number_of_sliders: i16,
    pub number_of_spinners: i16,
    pub last_modification_time: NaiveDate,
    pub approach_rate: ByteSingle,
    pub circle_size: ByteSingle,
    pub hp_drain: ByteSingle,
    pub overall_difficulty: ByteSingle,
    pub slider_velocity: f64,
    pub num_mod_combo_star_ratings_standard: Option<i32>,
    pub mod_combo_star_ratings_standard: Option<Vec<(i32, f64)>>,
    pub num_mod_combo_star_ratings_taiko: Option<i32>,
    pub mod_combo_star_ratings_taiko: Option<Vec<(i32, f64)>>,
    pub num_mod_combo_star_ratings_ctb: Option<i32>,
    pub mod_combo_star_ratings_ctb: Option<Vec<(i32, f64)>>,
    pub num_mod_combo_star_ratings_mania: Option<i32>,
    pub mod_combo_star_ratings_mania: Option<Vec<(i32, f64)>>,
    pub drain_time: i32,
    pub total_time: i32,
    pub preview_offset_from_start_ms: i32,
    pub num_timing_points: i32,
    pub timing_points: Vec<TimingPoint>,
    pub beatmap_id: i32,
    pub beatmap_set_id: i32,
    pub thread_id: i32,
    pub standard_grade: u8,
    pub taiko_grade: u8,
    pub ctb_grade: u8,
    pub mania_grade: u8,
    pub local_offset: i16,
    pub stack_leniency: f32,
    pub gameplay_mode: GameplayMode,
    pub song_source: Option<&'a str>,
    pub song_tags: Option<&'a str>,
    pub online_offset: i16,
    pub font_used_for_song_title: Option<&'a str>,
    pub unplayed: bool,
    pub last_played: NaiveDate,
    pub is_osz2: bool,
    pub beatmap_folder_name: Option<&'a str>,
    pub last_checked_against_repo: NaiveDate,
    pub ignore_beatmap_sound: bool,
    pub ignore_beatmap_skin: bool,
    pub disable_storyboard: bool,
    pub disable_video: bool,
    pub visual_override: bool,
    pub unknown_short: Option<i16>,
    pub offset_from_song_start_in_editor_ms: i32,
    pub mania_scroll_speed: u8,
}

impl<'a> Beatmap<'a> {
    pub fn read_from_bytes<T: ReadVersionSpecificData>(
        bytes: &'a [u8],
        i: &mut usize,
    ) -> ParseFileResult<Self> {
        let entry_size = T::read_entry_size(bytes, i)?;
        let artist_name = read_str_utf8(bytes, i, "non-Unicode artist name")?;
        let artist_name_unicode = read_str_utf8(bytes, i, "Unicode artist name")?;
        let song_title = read_str_utf8(bytes, i, "non-Unicode song title")?;
        let song_title_unicode = read_str_utf8(bytes, i, "Unicode song title")?;
        let creator_name = read_player_name(bytes, i).map_err(|_| {
            let msg = format!("Error reading creator name.");
            DbFileParseError::new(PrimitiveError, msg.as_str())
        })?;
        let difficulty = read_str_utf8(bytes, i, "difficulty")?;
        let audio_file_name = read_str_utf8(bytes, i, "audio file name")?;
        let md5_beatmap_hash = read_md5_hash(bytes, i)?;
        let dotosu_file_name = read_str_utf8(bytes, i, "corresponding .osu file name")?;
        let ranked_status = RankedStatus::read_from_bytes(bytes, i)?;
        let number_of_hitcircles = read_short(bytes, i)?;
        let number_of_sliders = read_short(bytes, i)?;
        let number_of_spinners = read_short(bytes, i)?;
        let last_modification_time = read_datetime(bytes, i)?;
        let approach_rate = T::read_arcshpod(bytes, i)?;
        let circle_size = T::read_arcshpod(bytes, i)?;
        let hp_drain = T::read_arcshpod(bytes, i)?;
        let overall_difficulty = T::read_arcshpod(bytes, i)?;
        let slider_velocity = read_double(bytes, i)?;
        let (num_mcsr_standard, mcsr_standard) = T::read_mod_combo_star_ratings(bytes, i)?;
        let (num_mcsr_taiko, mcsr_taiko) = T::read_mod_combo_star_ratings(bytes, i)?;
        let (num_mcsr_ctb, mcsr_ctb) = T::read_mod_combo_star_ratings(bytes, i)?;
        let (num_mcsr_mania, mcsr_mania) = T::read_mod_combo_star_ratings(bytes, i)?;
        let drain_time = read_int(bytes, i)?;
        let total_time = read_int(bytes, i)?;
        let preview_offset_from_start_ms = read_int(bytes, i)?;
        let num_timing_points = read_int(bytes, i)?;
        let mut timing_points = Vec::with_capacity(num_timing_points as usize);
        for _ in 0..num_timing_points {
            timing_points.push(TimingPoint::read_from_bytes(bytes, i)?);
        }
        let beatmap_id = read_int(bytes, i)?;
        let beatmap_set_id = read_int(bytes, i)?;
        let thread_id = read_int(bytes, i)?;
        let standard_grade = read_byte(bytes, i)?;
        let taiko_grade = read_byte(bytes, i)?;
        let ctb_grade = read_byte(bytes, i)?;
        let mania_grade = read_byte(bytes, i)?;
        let local_offset = read_short(bytes, i)?;
        let stack_leniency = read_single(bytes, i)?;
        let gameplay_mode = GameplayMode::read_from_bytes(bytes, i)?;
        let song_source = read_str_utf8(bytes, i, "song source")?;
        let song_tags = read_str_utf8(bytes, i, "song tags")?;
        let online_offset = read_short(bytes, i)?;
        let font_used_for_song_title = read_str_utf8(bytes, i, "font used for song title")?;
        let unplayed = read_boolean(bytes, i)?;
        let last_played = read_datetime(bytes, i)?;
        let is_osz2 = read_boolean(bytes, i)?;
        let beatmap_folder_name = read_str_utf8(bytes, i, "folder name")?;
        let last_checked_against_repo = read_datetime(bytes, i)?;
        let ignore_beatmap_sound = read_boolean(bytes, i)?;
        let ignore_beatmap_skin = read_boolean(bytes, i)?;
        let disable_storyboard = read_boolean(bytes, i)?;
        let disable_video = read_boolean(bytes, i)?;
        let visual_override = read_boolean(bytes, i)?;
        let unknown_short = T::read_unknown_short(bytes, i)?;
        let offset_from_song_start_in_editor_ms = read_int(bytes, i)?;
        let mania_scroll_speed = read_byte(bytes, i)?;
        Ok(Beatmap {
            entry_size,
            artist_name,
            artist_name_unicode,
            song_title,
            song_title_unicode,
            creator_name,
            difficulty,
            audio_file_name,
            md5_beatmap_hash,
            dotosu_file_name,
            ranked_status,
            number_of_hitcircles,
            number_of_sliders,
            number_of_spinners,
            last_modification_time,
            approach_rate,
            circle_size,
            hp_drain,
            overall_difficulty,
            slider_velocity,
            num_mod_combo_star_ratings_standard: num_mcsr_standard,
            mod_combo_star_ratings_standard: mcsr_standard,
            num_mod_combo_star_ratings_taiko: num_mcsr_taiko,
            mod_combo_star_ratings_taiko: mcsr_taiko,
            num_mod_combo_star_ratings_ctb: num_mcsr_ctb,
            mod_combo_star_ratings_ctb: mcsr_ctb,
            num_mod_combo_star_ratings_mania: num_mcsr_mania,
            mod_combo_star_ratings_mania: mcsr_mania,
            drain_time,
            total_time,
            preview_offset_from_start_ms,
            num_timing_points,
            timing_points,
            beatmap_id,
            beatmap_set_id,
            thread_id,
            standard_grade,
            taiko_grade,
            ctb_grade,
            mania_grade,
            local_offset,
            stack_leniency,
            gameplay_mode,
            song_source,
            song_tags,
            online_offset,
            font_used_for_song_title,
            unplayed,
            last_played,
            is_osz2,
            beatmap_folder_name,
            last_checked_against_repo,
            ignore_beatmap_sound,
            ignore_beatmap_skin,
            disable_storyboard,
            disable_video,
            visual_override,
            unknown_short,
            offset_from_song_start_in_editor_ms,
            mania_scroll_speed,
        })
    }

    pub fn display(&self) {
        if self.entry_size.is_some() {
            println!("    entry size: {}", self.entry_size.as_ref().unwrap());
        }
        if self.artist_name.is_some() {
            println!("    artist name: {}", self.artist_name.as_ref().unwrap());
        } else {
            println!("    artist name:");
        }
        if self.artist_name_unicode.is_some() {
            println!(
                "    artist name unicode: {}",
                self.artist_name_unicode.as_ref().unwrap()
            );
        } else {
            println!("    artist name unicode:");
        }
        if self.song_title.is_some() {
            println!("    song title: {}", self.song_title.as_ref().unwrap());
        } else {
            println!("    song title:");
        }
        if self.song_title_unicode.is_some() {
            println!(
                "    song title unicode: {}",
                self.song_title_unicode.as_ref().unwrap()
            );
        } else {
            println!("    song title unicode:");
        }
        if self.creator_name.is_some() {
            println!("    creator name: {}", self.creator_name.as_ref().unwrap());
        } else {
            println!("    creator name:");
        }
        if self.difficulty.is_some() {
            println!("    difficulty: {}", self.difficulty.as_ref().unwrap());
        } else {
            println!("    difficulty:");
        }
        if self.audio_file_name.is_some() {
            println!(
                "    audio file name: {}",
                self.audio_file_name.as_ref().unwrap()
            );
        } else {
            println!("    audio file name:");
        }
        println!("md5 beatmap hash: {}", self.md5_beatmap_hash);
        if self.dotosu_file_name.is_some() {
            println!(
                "    .osu file name: {}",
                self.dotosu_file_name.as_ref().unwrap()
            );
        } else {
            println!("    .osu file name:");
        }
        println!("    ranked status: {}", self.ranked_status);
        println!("    number of hitcircles: {}", self.number_of_hitcircles);
        println!("    number of sliders: {}", self.number_of_sliders);
        println!("    number of spinners: {}", self.number_of_spinners);
        println!(
            "    last modification time: {}",
            self.last_modification_time
        );
        println!("    approach rate: {}", self.approach_rate);
        println!("    circle size: {}", self.circle_size);
        println!("    hp drain: {}", self.hp_drain);
        println!("    overall difficulty: {}", self.overall_difficulty);
        println!("    slider velocity: {}", self.slider_velocity);
        if self.num_mod_combo_star_ratings_standard.is_some() {
            println!(
                "    num mod combo star ratings osu!standard: {}",
                self.num_mod_combo_star_ratings_standard.as_ref().unwrap()
            );
        }
        if self.mod_combo_star_ratings_standard.is_some() {
            println!("    MSCR osu!standard {{");
            for (mods, star_rating) in self.mod_combo_star_ratings_standard.as_ref().unwrap() {
                println!("        mods: {} | star rating: {}*", mods, star_rating);
            }
            println!("    }}");
        }
        if self.num_mod_combo_star_ratings_taiko.is_some() {
            println!(
                "    num mod combo star ratings osu!taiko: {}",
                self.num_mod_combo_star_ratings_taiko.as_ref().unwrap()
            );
        }
        if self.mod_combo_star_ratings_taiko.is_some() {
            println!("    MSCR osu!taiko {{");
            for (mods, star_rating) in self.mod_combo_star_ratings_taiko.as_ref().unwrap() {
                println!("        mods: {} | star rating: {}*", mods, star_rating);
            }
            println!("    }}");
        }
        if self.num_mod_combo_star_ratings_ctb.is_some() {
            println!(
                "    num mod combo star ratings osu!ctb: {}",
                self.num_mod_combo_star_ratings_ctb.as_ref().unwrap()
            );
        }
        if self.mod_combo_star_ratings_ctb.is_some() {
            println!("    MSCR osu!ctb {{");
            for (mods, star_rating) in self.mod_combo_star_ratings_ctb.as_ref().unwrap() {
                println!("        mods: {} | star rating: {}*", mods, star_rating);
            }
            println!("    }}");
        }
        if self.num_mod_combo_star_ratings_mania.is_some() {
            println!(
                "    num mod combo star ratings osu!mania: {}",
                self.num_mod_combo_star_ratings_mania.as_ref().unwrap()
            );
        }
        if self.mod_combo_star_ratings_mania.is_some() {
            println!("    MSCR osu!mania {{");
            for (mods, star_rating) in self.mod_combo_star_ratings_mania.as_ref().unwrap() {
                println!("        mods: {} | star rating: {}*", mods, star_rating);
            }
            println!("    }}");
        }
        println!("    drain time: {}", self.drain_time);
        println!("    total time: {}", self.total_time);
        println!(
            "    preview offset from start ms: {}",
            self.preview_offset_from_start_ms
        );
        println!("    num timing points: {}", self.num_timing_points);
        println!("    timing points {{");
        for timing_point in &self.timing_points {
            println!("        {}", timing_point);
        }
        println!("    }}");
        println!("    beatmap id: {}", self.beatmap_id);
        println!("    beatmap set id: {}", self.beatmap_set_id);
        println!("    thread id: {}", self.thread_id);
        println!("    standard grade: {}", self.standard_grade);
        println!("    taiko grade: {}", self.taiko_grade);
        println!("    ctb grade: {}", self.ctb_grade);
        println!("    mania grade: {}", self.mania_grade);
        println!("    local offset: {}", self.local_offset);
        println!("    stack leniency: {}", self.stack_leniency);
        println!("    gameplay mode: {}", self.gameplay_mode);
        if self.song_source.is_some() {
            println!("    song source: {}", self.song_source.as_ref().unwrap());
        } else {
            println!("    song source:");
        }
        if self.song_tags.is_some() {
            println!("    song tags: {}", self.song_tags.as_ref().unwrap());
        } else {
            println!("    song tags:");
        }
        println!("    online offset: {}", self.online_offset);
        if self.font_used_for_song_title.is_some() {
            println!(
                "    font used for song title: {}",
                self.font_used_for_song_title.as_ref().unwrap()
            );
        } else {
            println!("    font used for song title:");
        }
        println!("    unplayed: {}", self.unplayed);
        println!("    last played: {}", self.last_played);
        println!("    is osz2: {}", self.is_osz2);
        if self.beatmap_folder_name.is_some() {
            println!(
                "    beatmap folder name: {}",
                self.beatmap_folder_name.as_ref().unwrap()
            );
        } else {
            println!("    beatmap folder name:");
        }
        println!(
            "    last checked against repo: {}",
            self.last_checked_against_repo
        );
        println!("    ignore beatmap sound: {}", self.ignore_beatmap_sound);
        println!("    ignore beatmap skin: {}", self.ignore_beatmap_skin);
        println!("    disable storyboard: {}", self.disable_storyboard);
        println!("    disable video: {}", self.disable_video);
        println!("    visual override: {}", self.visual_override);
        if self.unknown_short.is_some() {
            println!(
                "    unknown short: {}",
                self.unknown_short.as_ref().unwrap()
            );
        } else {
            println!("    unknown short:");
        }
        println!(
            "    offset from song start in editor ms: {}",
            self.offset_from_song_start_in_editor_ms
        );
        println!("    mania scroll speed: {}", self.mania_scroll_speed);
    }
}
