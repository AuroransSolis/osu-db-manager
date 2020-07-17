use crate::databases::{
    load::PartialLoad,
    osu::primitives::GameplayMode,
    scores::{partial_score::PartialScore, partial_scoresdb_beatmap::PartialScoresDbBeatmap},
};
use crate::deserialize_primitives::*;
use crate::load_settings::scores::{
    scoresdb_beatmap_load_settings::ScoresDbBeatmapLoadSettings,
    scoresdb_load_settings::ScoresDbLoadSettings,
};
use crate::masks::scores_mask::ScoresDbMask;
use crate::maybe_deserialize_primitives::*;
use crate::read_error::{
    DbFileParseError, ParseErrorKind, ParseErrorKind::PrimitiveError, ParseFileResult,
};
use crossbeam_utils::thread::{self, Scope, ScopedJoinHandle};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct PartialScoresDb<'a> {
    pub version: Option<i32>,
    pub number_of_beatmaps: i32,
    pub beatmaps: Option<Vec<PartialScoresDbBeatmap<'a>>>,
}

impl<'a> PartialLoad<'a, ScoresDbMask, ScoresDbLoadSettings> for PartialScoresDb<'a> {
    fn read_single_thread(
        settings: ScoresDbLoadSettings,
        bytes: &'a [u8],
    ) -> ParseFileResult<Self> {
        let mut index = 0;
        let i = &mut index;
        let version = if settings.version {
            Some(read_int(&bytes, i)?)
        } else {
            *i += 4;
            None
        };
        let number_of_beatmaps = read_int(&bytes, i)?;
        let beatmaps = if settings.beatmap_load_settings.is_partial() {
            if number_of_beatmaps == 0 {
                None
            } else {
                let mut tmp = Vec::with_capacity(number_of_beatmaps as usize);
                for _ in 0..number_of_beatmaps {
                    tmp.push(PartialScoresDbBeatmap::read_from_bytes(
                        &settings.beatmap_load_settings,
                        &bytes,
                        i,
                    )?);
                }
                Some(tmp)
            }
        } else {
            None
        };
        Ok(PartialScoresDb {
            version,
            number_of_beatmaps,
            beatmaps,
        })
    }

    fn read_multi_thread(
        settings: ScoresDbLoadSettings,
        jobs: usize,
        bytes: &'a [u8],
    ) -> ParseFileResult<Self> {
        let (version, number_of_beatmaps) = {
            let mut index = 0;
            (
                if settings.version {
                    Some(read_int(&bytes, &mut index)?)
                } else {
                    index += 4;
                    None
                },
                read_int(&bytes, &mut index)?,
            )
        };
        let beatmaps = if settings.beatmap_load_settings.ignore_all() {
            None
        } else {
            let counter = Arc::new(Mutex::new(0));
            let start_read = Arc::new(Mutex::new(8));
            let mut results = thread::scope(|s| {
                let threads = (0..jobs)
                    .map(|_| {
                        spawn_partial_scoresdb_beatmap_loader_thread(
                            s,
                            number_of_beatmaps as usize,
                            counter.clone(),
                            start_read.clone(),
                            bytes,
                            &settings.beatmap_load_settings,
                        )
                    })
                    .collect::<Vec<_>>();
                threads
                    .into_iter()
                    .map(|joinhandle| {
                        joinhandle.join().map_err(|_| {
                            DbFileParseError::new(
                                ParseErrorKind::ScoresDbError,
                                "Failed to join scores.db partial beatmap parsing thread.",
                            )
                        })?
                    })
                    .collect::<Vec<_>>()
            })
            .map_err(|_| {
                DbFileParseError::new(
                    ParseErrorKind::ScoresDbError,
                    "Failed to retrieve result from scores.db partial beatmap parsing scope.",
                )
            })?;
            let mut partial_scoredb_beatmaps = results.pop().unwrap()?;
            for partial_scoredb_beatmap_result in results {
                partial_scoredb_beatmaps.append(&mut partial_scoredb_beatmap_result?);
            }
            partial_scoredb_beatmaps.sort_by(|(a, _), (b, _)| a.cmp(b));
            Some(
                partial_scoredb_beatmaps
                    .into_iter()
                    .map(|(_, scoredbbeatmap)| scoredbbeatmap)
                    .collect::<Vec<PartialScoresDbBeatmap>>(),
            )
        };
        Ok(PartialScoresDb {
            version,
            number_of_beatmaps,
            beatmaps,
        })
    }
}

fn spawn_partial_scoresdb_beatmap_loader_thread<'scope, 'b: 'scope, 'a: 'b>(
    scope: &'scope Scope<'b>,
    number_of_scoresdb_beatmaps: usize,
    counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>,
    bytes: &'a [u8],
    settings: &'b ScoresDbBeatmapLoadSettings,
) -> ScopedJoinHandle<'scope, ParseFileResult<Vec<(usize, PartialScoresDbBeatmap<'a>)>>> {
    scope.spawn(move |_| {
        let mut partial_scoresdb_beatmaps = Vec::new();
        loop {
            let mut skip = false;
            let s = &mut skip;
            let (md5_beatmap_hash, number_of_scores, mut start_read, number) = {
                let mut ctr = counter.lock().unwrap();
                let number = if *ctr >= number_of_scoresdb_beatmaps {
                    return Ok(partial_scoresdb_beatmaps);
                } else {
                    *ctr += 1;
                    *ctr - 1
                };
                let mut start = start_read.lock().unwrap();
                let md5_beatmap_hash =
                    maybe_read_md5_hash(&settings.md5_beatmap_hash, s, bytes, start.deref_mut())?;
                let number_of_scores = read_int(bytes, start.deref_mut())?;
                let section_start = *start;
                for _ in 0..number_of_scores {
                    // Skips:
                    // 1 byte for gameplay_mode
                    // 4 bytes for score_version
                    // 34 bytes for MD5 beatmap hash (assumed never to be missing)
                    *start += 39;
                    // Assuming 32 characters max length for username, +2 for indicator and ULEB128
                    let indicator = *bytes.get(*start).ok_or_else(|| {
                        DbFileParseError::new(
                            PrimitiveError,
                            "Failed to read \
                                                indicator for player name.",
                        )
                    })?;
                    let player_name_len = if indicator == 0x0b {
                        Ok(*bytes.get(*start + 1).ok_or_else(|| {
                            DbFileParseError::new(
                                PrimitiveError,
                                "Failed to read player name length.",
                            )
                        })?)
                    } else if indicator == 0 {
                        Ok(0)
                    } else {
                        Err(DbFileParseError::new(
                            PrimitiveError,
                            "Read invalid indicator for score \
                                                player name.",
                        ))
                    }?;
                    // Check if greater than or equal to 32.
                    if player_name_len & 0b10000000 == 0b10000000 {
                        return Err(DbFileParseError::new(
                            PrimitiveError,
                            "Read invalid player name \
                                                length.",
                        ));
                    }
                    if indicator == 0 {
                        *start += 1;
                    } else {
                        *start += 2;
                    }
                    *start += player_name_len as usize + 78;
                    // Skips:
                    // 34 bytes for replay MD5 hash
                    // 2 bytes for number of 300s
                    // 2 bytes for number of 100s
                    // 2 bytes for number of 50s
                    // 2 bytes for number of gekis
                    // 2 bytes for number of katus
                    // 2 bytes for number of misses
                    // 4 bytes for score
                    // 2 bytes for max combo
                    // 1 byte for perfect combo
                    // 4 bytes for mods used
                    // 1 byte for empty string indicator
                    // 8 bytes for replay timestamp
                    // 4 bytes for 0xFFFFFFFF
                    // 8 bytes for score ID
                    // Total of 78
                }
                (
                    md5_beatmap_hash,
                    number_of_scores,
                    section_start,
                    number,
                )
            };
            continue_if!(*s);
            let scores = if settings.score_load_settings.ignore_all() || number_of_scores == 0 {
                None
            } else {
                let mut tmp = Vec::with_capacity(number_of_scores as usize);
                let i = &mut start_read;
                for _ in 0..number_of_scores {
                    let mut skip = false;
                    let s = &mut skip;
                    let gameplay_mode = GameplayMode::maybe_read_from_bytes(
                        settings.score_load_settings.gameplay_mode,
                        s,
                        bytes,
                        i,
                    )?;
                    continue_if!(*s);
                    let score_version =
                        maybe_read_int(settings.score_load_settings.score_version, s, bytes, i)?;
                    continue_if!(*s);
                    let md5_beatmap_hash = maybe_read_md5_hash(
                        &settings.score_load_settings.md5_beatmap_hash,
                        s,
                        bytes,
                        i,
                    )?;
                    continue_if!(*s);
                    let player_name = maybe_read_str_utf8(
                        &settings.score_load_settings.player_name,
                        s,
                        bytes,
                        i,
                        "player name",
                    )?;
                    let md5_replay_hash = maybe_read_md5_hash(
                        &settings.score_load_settings.md5_replay_hash,
                        s,
                        bytes,
                        i,
                    )?;
                    continue_if!(*s);
                    let number_of_300s =
                        maybe_read_short(settings.score_load_settings.number_of_300s, s, bytes, i)?;
                    continue_if!(*s);
                    let number_of_100s =
                        maybe_read_short(settings.score_load_settings.number_of_100s, s, bytes, i)?;
                    continue_if!(*s);
                    let number_of_50s =
                        maybe_read_short(settings.score_load_settings.number_of_50s, s, bytes, i)?;
                    continue_if!(*s);
                    let number_of_gekis = maybe_read_short(
                        settings.score_load_settings.number_of_gekis,
                        s,
                        bytes,
                        i,
                    )?;
                    continue_if!(*s);
                    let number_of_katus = maybe_read_short(
                        settings.score_load_settings.number_of_katus,
                        s,
                        bytes,
                        i,
                    )?;
                    continue_if!(*s);
                    let number_of_misses = maybe_read_short(
                        settings.score_load_settings.number_of_misses,
                        s,
                        bytes,
                        i,
                    )?;
                    continue_if!(*s);
                    let replay_score =
                        maybe_read_int(settings.score_load_settings.replay_score, s, bytes, i)?;
                    continue_if!(*s);
                    let max_combo =
                        maybe_read_short(settings.score_load_settings.max_combo, s, bytes, i)?;
                    continue_if!(*s);
                    let perfect_combo = maybe_read_boolean(
                        settings.score_load_settings.perfect_combo,
                        s,
                        bytes,
                        i,
                    )?;
                    continue_if!(*s);
                    let mods_used =
                        maybe_read_int(settings.score_load_settings.mods_used, s, bytes, i)?;
                    continue_if!(*s);
                    let empty_string = maybe_read_str_utf8_nocomp(
                        settings.score_load_settings.empty_string,
                        s,
                        bytes,
                        i,
                        "empty string",
                    )?;
                    continue_if!(*s);
                    let replay_timestamp = maybe_read_datetime(
                        settings.score_load_settings.replay_timestamp,
                        s,
                        bytes,
                        i,
                    )?;
                    continue_if!(*s);
                    let negative_one = maybe_read_int_nocomp(
                        settings.score_load_settings.negative_one,
                        s,
                        bytes,
                        i,
                    )?;
                    continue_if!(*s);
                    let online_score_id =
                        maybe_read_long(settings.score_load_settings.online_score_id, s, bytes, i)?;
                    continue_if!(*s);
                    tmp.push(PartialScore {
                        gameplay_mode,
                        score_version,
                        md5_beatmap_hash,
                        player_name,
                        md5_replay_hash,
                        number_of_300s,
                        number_of_100s,
                        number_of_50s,
                        number_of_gekis,
                        number_of_katus,
                        number_of_misses,
                        replay_score,
                        max_combo,
                        perfect_combo,
                        mods_used,
                        empty_string,
                        replay_timestamp,
                        negative_one,
                        online_score_id,
                    })
                }
                Some(tmp)
            };
            partial_scoresdb_beatmaps.push((
                number,
                PartialScoresDbBeatmap {
                    md5_beatmap_hash,
                    number_of_scores,
                    scores,
                },
            ));
        }
    })
}
