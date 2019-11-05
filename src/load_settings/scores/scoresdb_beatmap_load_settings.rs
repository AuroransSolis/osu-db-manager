use std::io::Result as IoResult;

use clap::{App, AppSettings, Arg, SubCommand};

use crate::load_settings::{
    query::QueryStruct, scores::score_load_settings::ScoreLoadSettings, EqualClone, LoadSetting,
    Relational,
};
use crate::masks::scores_mask::ScoresDbBeatmapMask;

pub struct ScoresDbBeatmapLoadSettings {
    pub md5_beatmap_hash: LoadSetting<EqualClone<String>>,
    pub number_of_scores: LoadSetting<Relational<i32>>,
    pub score_load_settings: ScoreLoadSettings,
}

impl ScoresDbBeatmapLoadSettings {
    pub fn load_all(&self) -> bool {
        self.score_load_settings.load_all()
            && self.md5_beatmap_hash.is_load()
            && self.number_of_scores.is_load()
    }

    pub fn ignore_all(&self) -> bool {
        self.score_load_settings.ignore_all()
            && self.md5_beatmap_hash.is_ignore()
            && self.number_of_scores.is_ignore()
    }

    pub fn is_partial(&self) -> bool {
        self.score_load_settings.is_partial()
            || self.md5_beatmap_hash.is_ignore()
            || self.number_of_scores.is_ignore()
    }

    pub fn set_from_query(&mut self, args: Vec<&str>) -> IoResult<()> {
        let matches = App::new("scores.db beatmap query parser")
            .arg(
                Arg::with_name("MD5 beatmap hash")
                    .long("MD5-BEATMAP-HASH")
                    .required(false)
                    .multiple(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("HASH"),
            )
            .arg(
                Arg::with_name("Number of scores")
                    .long("NUMBER-OF-SCORES")
                    .required(false)
                    .multiple(false)
                    .takes_value(true)
                    .number_of_values(1)
                    .value_name("NUMBER/RANGE"),
            )
            .subcommand(
                SubCommand::with_name("SCORE-QUERY")
                    .arg(
                        Arg::with_name("Gameplay mode")
                            .long("GAMEPLAY-MODE")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("MODE"),
                    )
                    .arg(
                        Arg::with_name("Score version")
                            .long("SCORE-VERSION")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("NUMBER/RANGE"),
                    )
                    .arg(
                        Arg::with_name("MD5 beatmap hash")
                            .long("MD5-BEATMAP-HASH")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("HASH"),
                    )
                    .arg(
                        Arg::with_name("Player name")
                            .long("PLAYER-NAME")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("NAME"),
                    )
                    .arg(
                        Arg::with_name("MD5 replay hash")
                            .long("MD5-REPLAY-HASH")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("HASH"),
                    )
                    .arg(
                        Arg::with_name("Number of 300s")
                            .long("NUMBER-OF-300S")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("NUMBER/RANGE"),
                    )
                    .arg(
                        Arg::with_name("Number of 100s (standard/CTB)/150s (taiko)/200s (mania)")
                            .long("NUMBER-OF-100S")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("NUMBER/RANGE"),
                    )
                    .arg(
                        Arg::with_name("Number of 50s (standard/mania)/small fruit (CTB)")
                            .long("NUMBER-OF-50S")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("NUMBER/RANGE"),
                    )
                    .arg(
                        Arg::with_name("Number of gekis/max 300s (mania)")
                            .long("NUMBER-OF-GEKIS")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("NUMBER/RANGE"),
                    )
                    .arg(
                        Arg::with_name("Number of katus/100s (mania)")
                            .long("NUMBER-OF-KATUS")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("NUMBER/RANGE"),
                    )
                    .arg(
                        Arg::with_name("Number of misses")
                            .long("NUMBER-OF-MISSES")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("NUMBER/RANGE"),
                    )
                    .arg(
                        Arg::with_name("Replay score")
                            .long("REPLAY-SCORE")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("NUMBER/RANGE"),
                    )
                    .arg(
                        Arg::with_name("Max combo")
                            .long("MAX-COMBO")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("NUMBER/RANGE"),
                    )
                    .arg(
                        Arg::with_name("Perfect combo")
                            .long("PERFECT-COMBO")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("BOOL"),
                    )
                    .arg(
                        Arg::with_name("Mods used")
                            .long("MODS-USED")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("MODS"),
                    )
                    .arg(
                        Arg::with_name("Replay timestamp")
                            .long("REPLAY-TIMESTAMP")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("DATE/RANGE"),
                    )
                    .arg(
                        Arg::with_name("Online score ID")
                            .long("ONLINE-SCORE-ID")
                            .required(false)
                            .multiple(false)
                            .takes_value(true)
                            .number_of_values(1)
                            .value_name("NUMBER/RANGE"),
                    ),
            )
            .get_matches_from(args.into_iter());
        self.md5_beatmap_hash = EqualClone::from_matches(&matches, "MD5 beatmap hash")?.into();
        self.number_of_scores = Relational::from_matches(&matches, "Number of scores")?.into();
        if let Some(m) = matches.subcommand_matches("SCORE-QUERY") {
            self.score_load_settings.set_from_query(m)
        } else {
            Ok(())
        }
    }

    pub fn set_from_mask(&mut self, mask: ScoresDbBeatmapMask) {
        if self.md5_beatmap_hash.is_ignore() && mask.md5_beatmap_hash {
            self.md5_beatmap_hash = LoadSetting::Load;
        }
        if self.number_of_scores.is_ignore() && mask.number_of_scores {
            self.number_of_scores = LoadSetting::Load;
        }
        if let Some(m) = mask.scores_mask {
            self.score_load_settings.set_from_mask(m);
        }
    }
}
