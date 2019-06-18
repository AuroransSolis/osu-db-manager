use crate::masks::mask::Mask;

#[derive(Copy, Clone, Debug)]
pub struct CollectionMask {
    pub collection_name: bool,
    pub number_of_beatmaps: bool,
    pub md5_beatmap_hashes: bool
}

impl CollectionMask {
    pub fn new(collection_name: bool, number_of_beatmaps: bool, md5_beatmap_hashes: bool) -> Self {
        CollectionMask {
            collection_name,
            number_of_beatmaps,
            md5_beatmap_hashes
        }
    }
}

impl Mask for CollectionMask {
    fn is_complete(&self) -> bool {
        self.collection_name && self.number_of_beatmaps && self.md5_beatmap_hashes
    }

    fn from_show_and_query(show: Self, query: Self) -> Self {
        CollectionMask {
            collection_name: show.collection_name || query.collection_name,
            number_of_beatmaps: show.number_of_beatmaps || query.number_of_beatmaps,
            md5_beatmap_hashes: show.md5_beatmap_hashes || query.md5_beatmap_hashes
        }
    }
}

impl Default for CollectionMask {
    fn default() -> Self {
        CollectionMask {
            collection_name: true,
            number_of_beatmaps: true,
            md5_beatmap_hashes: true
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CollectionDbMask {
    pub version: bool,
    pub number_of_collections: bool,
    pub collections_mask: Option<CollectionMask>
}

impl CollectionDbMask {
    pub fn new(version: bool, number_of_collections: bool, collections_mask: Option<CollectionMask>)
        -> Self {
        CollectionDbMask {
            version,
            number_of_collections,
            collections_mask
        }
    }
}

impl Mask for CollectionDbMask {
    fn is_complete(&self) -> bool {
        if let Some(collection_mask) = self.collections_mask {
            collection_mask.is_complete() && self.version && self.number_of_collections
        } else {
            false
        }
    }

    fn from_show_and_query(show: Self, query: Self) -> Self {
        CollectionDbMask {
            version: show.version || query.version,
            number_of_collections: show.number_of_collections || query.number_of_collections,
            collections_mask: {
                match (show.collections_mask, query.collections_mask) {
                    (Some(show_mask), Some(query_mask)) => {
                        Some(CollectionMask::from_show_and_query(show_mask, query_mask))
                    },
                    (Some(show_mask), None) => Some(show_mask),
                    (None, Some(query_mask)) => Some(query_mask),
                    (None, None) => None
                }
            }
        }
    }
}

impl Default for CollectionDbMask {
    fn default() -> Self {
        CollectionDbMask {
            version: true,
            number_of_collections: true,
            collections_mask: Some(CollectionMask::default())
        }
    }
}