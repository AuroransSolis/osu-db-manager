use structopt::StructOpt;

#[derive(Copy, Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct CollectionMask {
    #[structopt(long = "show-collection-name")]
    pub collection_name: bool,
    #[structopt(long = "show-number-of-beatmaps")]
    pub number_of_beatmaps: bool,
    #[structopt(long = "show-md5-beatmap-hashes")]
    pub md5_beatmap_hashes: bool,
}

impl CollectionMask {
    pub fn ignore_all(&self) -> bool {
        !self.collection_name && !self.number_of_beatmaps && !self.md5_beatmap_hashes
    }
}

#[derive(Copy, Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct CollectionDbMask {
    #[structopt(long = "show-version")]
    pub version: bool,
    #[structopt(long = "show-number-of-collections")]
    pub number_of_collections: bool,
    #[structopt(flatten)]
    pub collections_mask: CollectionMask,
}

impl CollectionDbMask {
    pub fn ignore_all(&self) -> bool {
        !self.version && !self.number_of_collections && self.collections_mask.ignore_all()
    }
}
