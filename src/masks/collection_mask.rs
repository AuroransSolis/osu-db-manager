use structopt::StructOpt;

#[derive(Copy, Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct CollectionMask {
    #[structopt(long)]
    pub collection_name: bool,
    #[structopt(long)]
    pub number_of_beatmaps: bool,
    #[structopt(long)]
    pub md5_beatmap_hashes: bool,
}

impl CollectionMask {
    pub fn ignore_all(&self) -> bool {
        !self.collection_name && !self.number_of_beatmaps && !self.md5_beatmap_hashes
    }

    pub fn is_complete(&self) -> bool {
        self.collection_name && self.number_of_beatmaps && self.md5_beatmap_hashes
    }
}

#[derive(Copy, Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct CollectionDbMask {
    #[structopt(long)]
    pub version: bool,
    #[structopt(long)]
    pub number_of_collections: bool,
    #[structopt(flatten)]
    pub collections_mask: CollectionMask,
}

impl CollectionDbMask {
    pub fn ignore_all(&self) -> bool {
        !self.version && !self.number_of_collections && self.collections_mask.ignore_all()
    }

    pub fn is_complete(&self) -> bool {
        self.version && self.number_of_collections && self.collections_mask.is_complete()
    }
}
