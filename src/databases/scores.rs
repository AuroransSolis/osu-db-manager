use std::io::{Result as IoResult, Error as IoError, ErrorKind::{Other, InvalidData}};
use std::time::SystemTime;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use crate::deserialize_primitives::*;
use crate::databases::{load::Load, osu::GameplayMode};

#[derive(Debug, Clone)]
pub struct Score {
    pub gameplay_mode: GameplayMode,
    pub score_version: i32,
    pub md5_beatmap_hash: Option<String>,
    pub player_name: Option<String>,
    pub md5_replay_hash: Option<String>,
    pub number_of_300s: i16,
    pub number_of_100s: i16, // 150s in Taiko, 100s in CTB, 200s in Mania
    pub number_of_50s: i16, // small fruit in CTB, 50s in Mania
    pub number_of_gekis: i16, // max 300s in Mania
    pub number_of_katus: i16, // 100s in mania
    pub number_of_misses: i16,
    pub replay_score: i32,
    pub max_combo: i16,
    pub perfect_combo: bool,
    pub mods_used: i32,
    pub empty_string: Option<String>,
    pub replay_timestamp: SystemTime,
    pub negative_one: i32,
    pub online_score_id: i64
}

impl Score {
    pub fn read_from_bytes<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<Self> {
        let gameplay_mode = GameplayMode::read_from_bytes(i)?;
        let score_version = read_int(i)?;
        let md5_beatmap_hash = read_md5_hash(i)?;
        let player_name = read_string_utf8(i, "player name")?;
        let md5_replay_hash = read_md5_hash(i)?;
        let number_of_300s = read_short(i)?;
        let number_of_100s = read_short(i)?;
        let number_of_50s = read_short(i)?;
        let number_of_gekis = read_short(i)?;
        let number_of_katus = read_short(i)?;
        let number_of_misses = read_short(i)?;
        let replay_score = read_int(i)?;
        let max_combo = read_short(i)?;
        let perfect_combo = read_boolean(i)?;
        let mods_used = read_int(i)?;
        let empty_string = read_string_utf8(i, "empty string")?;
        let replay_timestamp = read_datetime(i)?;
        let negative_one = read_int(i)?;
        let online_score_id = read_long(i)?;
        Ok(Score {
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
            online_score_id
        })
    }
}

#[derive(Debug, Clone)]
pub struct ScoreDbBeatmap {
    pub md5_beatmap_hash: Option<String>,
    pub number_of_scores: i32,
    pub scores: Option<Vec<Score>>
}

impl ScoreDbBeatmap {
    pub fn read_from_bytes<I: Iterator<Item = u8>>(i: &mut I) -> IoResult<Self> {
        let md5_beatmap_hash = read_string_utf8(i, "MD5 beatmap hash")?;
        let number_of_scores = read_int(i)?;
        let mut scores = if number_of_scores == 0 {
            None
        } else {
            let mut scores = Vec::with_capacity(number_of_scores as usize);
            for _ in 0..number_of_scores {
                scores.push(Score::read_from_bytes(i)?);
            }
            Some(scores)
        };
        Ok(ScoreDbBeatmap {
            md5_beatmap_hash,
            number_of_scores,
            scores
        })
    }
}

#[derive(Debug, Clone)]
pub struct ScoresDb {
    pub version: i32,
    pub number_of_beatmaps: i32,
    pub beatmaps: Vec<ScoreDbBeatmap>
}

impl Load for ScoresDb {
    fn read_single_thread(bytes: Vec<u8>) -> IoResult<Self> {
        let mut bytes_iter = bytes.into_iter();
        let version = read_int(&mut bytes_iter)?;
        let number_of_beatmaps = read_int(&mut bytes_iter)?;
        let mut beatmaps = Vec::with_capacity(number_of_beatmaps as usize);
        for _ in 0..number_of_beatmaps {
            beatmaps.push(ScoreDbBeatmap::read_from_bytes(&mut bytes_iter)?);
        }
        Ok(ScoresDb {
            version,
            number_of_beatmaps,
            beatmaps
        })
    }

    fn read_multi_thread(jobs: usize, bytes: Vec<u8>) -> IoResult<Self> {
        let (version, number_of_beatmaps) = {
            let mut bytes_iter = bytes[0..8].iter().cloned();
            (read_int(&mut bytes_iter)?, read_int(&mut bytes_iter)?)
        };
        println!("version: {}, number of beatmaps: {}", version, number_of_beatmaps);
        let counter = Arc::new(Mutex::new(0));
        let start_read = Arc::new(Mutex::new(8));
        let threads = (0..jobs).map(|i| spawn_scoredbbeatmap_loader(number_of_beatmaps as usize,
            counter.clone(), start_read.clone(), &bytes, i)).collect::<Vec<_>>();
        let mut results = threads.into_iter().map(|joinhandle| joinhandle.join().unwrap())
            .collect::<Vec<_>>();
        let mut scoredbbeatmaps = results.pop().unwrap()?;
        for scoredbbeatmap_result in results {
            scoredbbeatmaps.append(&mut scoredbbeatmap_result?);
        }
        scoredbbeatmaps.sort_by(|(a, _), (b, _)| a.cmp(b));
        let beatmaps = scoredbbeatmaps.into_iter().map(|(_, scoredbbeatmap)| scoredbbeatmap)
            .collect::<Vec<ScoreDbBeatmap>>();
        Ok(ScoresDb {
            version,
            number_of_beatmaps,
            beatmaps
        })
    }
}

fn spawn_scoredbbeatmap_loader(number_of_scoredbbeatmaps: usize, counter: Arc<Mutex<usize>>,
    start_read: Arc<Mutex<usize>>, bytes_pointer: *const Vec<u8>, thread_no: usize)
    -> JoinHandle<IoResult<Vec<(usize, ScoreDbBeatmap)>>> {
    let tmp = bytes_pointer as usize;
    thread::spawn(move || {
        let bytes = unsafe { &*(tmp as *const Vec<u8>) };
        let mut score_db_beatmaps = Vec::new();
        loop {
            println!("Updating counter and start offset {}", thread_no);
            let (md5_beatmap_hash, number_of_scores, start_read, end, number) = {
                let mut ctr = counter.lock().unwrap();
                let number = if *ctr >= number_of_scoredbbeatmaps {
                    println!("Finished loading scores.");
                    return Ok(score_db_beatmaps);
                } else {
                    *ctr += 1;
                    *ctr - 1
                };
                let mut s = start_read.lock().unwrap();
                println!("    locked both mutexes");
                println!("    counter incremented to {}", *ctr);
                println!("    using start offset: {}", *s);
                let md5_beatmap_hash = read_md5_hash(&mut (&bytes[*s..*s+ 34]).iter().cloned())?;
                println!("    read MD5 beatmap hash");
                let number_of_scores = read_int(&mut (&bytes[*s + 18..*s + 22]).iter().cloned())?;
                println!("    read number of scores: {}", number_of_scores);
                // Skips:
                // 34 bytes for beatmap MD5 hash
                // 4 bytes for number of beatmaps
                *s += 38;
                let start_from = *s;
                for i in 0..number_of_scores {
                    println!("        getting player name len info for score {}", i);
                    // Skips:
                    // 1 byte for gameplay_mode
                    // 4 bytes for score_version
                    // 34 bytes for MD5 beatmap hash
                    *s += 39;
                    // Assuming 32 characters max length for username, +2 for indicator and ULEB128
                    let indicator = read_byte(&mut (&bytes[*s..*s + 1]).iter().cloned())?;
                    let player_name_len = if indicator == 0x0b {
                        read_byte(&mut (&bytes[*s + 1..*s + 2]).iter().cloned())?
                    } else if indicator == 0 {
                        0
                    } else {
                        return Err(IoError::new(InvalidData, "Read invalid indicator for score \
                            player name."));
                    };
                    println!("        got player name len");
                    // let (player_name_len, player_name) = read_player_name_with_len(
                        // &mut (&bytes[*s..*s + 34]).iter().cloned())?;
                    *s += player_name_len as usize + 78;
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
                println!("    got necessary info, dropping locks");
                (md5_beatmap_hash, number_of_scores, start_from, *s, number)
            };
            println!("    got MD5 beatmap hash, number of scores, start offset, end offset, and beatmap number");
            let scores = if number_of_scores == 0 {
                None
            } else {
                let mut scores = Vec::with_capacity(number_of_scores as usize);
                let mut i = bytes[start_read..end].iter().cloned();
                for _ in 0..number_of_scores {
                    scores.push(Score::read_from_bytes(&mut i)?);
                }
                Some(scores)
            };
            println!("    read scores");
            score_db_beatmaps.push((number, ScoreDbBeatmap {
                md5_beatmap_hash,
                number_of_scores,
                scores
            }));
        }
    })
}
