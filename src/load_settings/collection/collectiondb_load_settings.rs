use crate::load_settings::collection::collection_load_settings::CollectionLoadSettings;
use crate::masks::collection_mask::CollectionDbMask;
use std::default::Default;
use structopt::StructOpt;

#[derive(Clone, StructOpt)]
pub struct CollectionDbLoadSettings {
    #[structopt(skip)]
    pub version: bool,
    #[structopt(skip)]
    pub number_of_collections: bool,
    #[structopt(flatten)]
    pub collection_load_settings: CollectionLoadSettings,
}

impl CollectionDbLoadSettings {
    pub fn load_all(&self) -> bool {
        self.version && self.number_of_collections && self.collection_load_settings.load_all()
    }

    pub fn ignore_all(&self) -> bool {
        !self.version && !self.number_of_collections && self.collection_load_settings.ignore_all()
    }

    pub fn is_partial(&self) -> bool {
        !self.version || !self.number_of_collections || self.collection_load_settings.is_partial()
    }

    pub fn set_from_mask(&mut self, mask: &CollectionDbMask) {
        self.version |= mask.version;
        self.number_of_collections |= mask.number_of_collections;
        self.collection_load_settings
            .set_from_mask(&mask.collections_mask);
    }
}

impl Default for CollectionDbLoadSettings {
    fn default() -> Self {
        CollectionDbLoadSettings {
            version: bool::default(),
            number_of_collections: bool::default(),
            collection_load_settings: CollectionLoadSettings::default(),
        }
    }
}
