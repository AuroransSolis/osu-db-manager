use crate::read_error::ParseFileResult;
use crate::deserialize_primitives::*;
use crate::maybe_deserialize_primitives::*;
use crate::databases::osu::primitives::{read_int_double_pair, ByteSingle::{self, *}};

#[derive(Copy, Clone, Debug)]
pub struct Legacy;

#[derive(Copy, Clone, Debug)]
pub struct Modern;

#[derive(Copy, Clone, Debug)]
pub struct ModernWithEntrySize;

pub trait ReadVersionSpecificData {
    fn read_entry_size(bytes: &[u8], i: &mut usize) -> ParseFileResult<Option<i32>>;
    fn read_arcshpod(bytes: &[u8], i: &mut usize) -> ParseFileResult<ByteSingle>;
    fn read_mod_combo_star_ratings(bytes: &[u8], i: &mut usize)
        -> ParseFileResult<(Option<i32>, Option<Vec<(i32, f64)>>)>;
    fn read_unknown_short(bytes: &[u8], i: &mut usize) -> ParseFileResult<Option<i16>>;
}

impl ReadVersionSpecificData for Legacy {
    #[inline]
    fn read_entry_size(_bytes: &[u8], _i: &mut usize) -> ParseFileResult<Option<i32>> {
        Ok(None)
    }

    #[inline]
    fn read_arcshpod(bytes: &[u8], i: &mut usize) -> ParseFileResult<ByteSingle> {
        Ok(Byte(read_byte(bytes, i)?))
    }

    #[inline]
    fn read_mod_combo_star_ratings(_bytes: &[u8], _i: &mut usize)
        -> ParseFileResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        Ok((None, None))
    }

    #[inline]
    fn read_unknown_short(bytes: &[u8], i: &mut usize) -> ParseFileResult<Option<i16>> {
        Ok(Some(read_short(bytes, i)?))
    }
}

impl ReadVersionSpecificData for Modern {
    #[inline]
    fn read_entry_size(_bytes: &[u8], _i: &mut usize) -> ParseFileResult<Option<i32>> {
        Ok(None)
    }

    #[inline]
    fn read_arcshpod(bytes: &[u8], i: &mut usize) -> ParseFileResult<ByteSingle> {
        Ok(Single(read_single(bytes, i)?))
    }

    #[inline]
    fn read_mod_combo_star_ratings(bytes: &[u8], i: &mut usize)
        -> ParseFileResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        let num_int_doubles = read_int(bytes, i)?;
        let mut int_double_pairs = Vec::with_capacity(num_int_doubles as usize);
        for _ in 0..num_int_doubles {
            int_double_pairs.push(read_int_double_pair(bytes, i)?);
        }
        Ok((Some(num_int_doubles), Some(int_double_pairs)))
    }

    #[inline]
    fn read_unknown_short(_bytes: &[u8], _i: &mut usize) -> ParseFileResult<Option<i16>> {
        Ok(None)
    }
}

impl ReadVersionSpecificData for ModernWithEntrySize {
    #[inline]
    fn read_entry_size(bytes: &[u8], i: &mut usize) -> ParseFileResult<Option<i32>> {
        Ok(Some(read_int(bytes, i)?))
    }

    #[inline]
    fn read_arcshpod(bytes: &[u8], i: &mut usize) -> ParseFileResult<ByteSingle> {
        Ok(Single(read_single(bytes, i)?))
    }

    #[inline]
    fn read_mod_combo_star_ratings(bytes: &[u8], i: &mut usize)
        -> ParseFileResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        let num_int_doubles = read_int(bytes, i)?;
        let mut int_double_pairs = Vec::with_capacity(num_int_doubles as usize);
        for _ in 0..num_int_doubles {
            int_double_pairs.push(read_int_double_pair(bytes, i)?);
        }
        Ok((Some(num_int_doubles), Some(int_double_pairs)))
    }

    #[inline]
    fn read_unknown_short(_bytes: &[u8], _i: &mut usize) -> ParseFileResult<Option<i16>> {
        Ok(None)
    }
}

pub trait ReadPartialVersionSpecificData {
    fn maybe_read_entry_size(c: bool, bytes: &[u8], i: &mut usize) -> ParseFileResult<Option<i32>>;
    fn maybe_read_arcshpod(c: bool, bytes: &[u8], i: &mut usize)
        -> ParseFileResult<Option<ByteSingle>>;
    fn maybe_read_mod_combo_star_ratings(c: bool, bytes: &[u8], i: &mut usize)
        -> ParseFileResult<(Option<i32>, Option<Vec<(i32, f64)>>)>;
    fn maybe_read_unknown_short(c: bool, bytes: &[u8], i: &mut usize)
        -> ParseFileResult<Option<i16>>;
}

impl ReadPartialVersionSpecificData for Legacy {
    #[inline]
    fn maybe_read_entry_size(_c: bool, _bytes: &[u8], _i: &mut usize)
        -> ParseFileResult<Option<i32>> {
        Ok(None)
    }

    #[inline]
    fn maybe_read_arcshpod(c: bool, bytes: &[u8], i: &mut usize)
        -> ParseFileResult<Option<ByteSingle>> {
        if c {
            Ok(Byte(read_byte(bytes, i)?))
        } else {
            *i += 1;
            Ok(None)
        }
    }

    #[inline]
    fn maybe_read_mod_combo_star_ratings(_c: bool, _bytes: &[u8], _i: &mut usize)
        -> ParseFileResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        Ok((None, None))
    }

    #[inline]
    fn maybe_read_unknown_short(c: bool, bytes: &[u8], i: &mut usize)
        -> ParseFileResult<Option<i16>> {
        maybe_read_short(c, bytes, i)
    }
}

impl ReadPartialVersionSpecificData for Modern {
    #[inline]
    fn maybe_read_entry_size(_c: bool, _bytes: &[u8], _i: &mut usize)
        -> ParseFileResult<Option<i32>> {
        Ok(None)
    }

    #[inline]
    fn maybe_read_arcshpod(c: bool, bytes: &[u8], i: &mut usize)
        -> ParseFileResult<Option<ByteSingle>> {
        if c {
            Ok(Single(read_single(bytes, i)?))
        } else {
            *i += 4;
            Ok(None)
        }
    }

    #[inline]
    fn maybe_read_mod_combo_star_ratings(c: bool, bytes: &[u8], i: &mut usize)
        -> ParseFileResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        let num_int_doubles = read_int(bytes, i)?;
        if c {
            let mut int_double_pairs = Vec::with_capacity(num_int_doubles as usize);
            for _ in 0..num_int_doubles {
                int_double_pairs.push(read_int_double_pair(bytes, i)?);
            }
            Ok((Some(num_int_doubles), Some(int_double_pairs)))
        } else {
            *i += num_int_doubles as usize * 14;
            Ok(None, None)
        }
    }

    #[inline]
    fn maybe_read_unknown_short(_c: bool, _bytes: &[u8], _i: &mut usize)
        -> ParseFileResult<Option<i16>> {
        Ok(None)
    }
}

impl ReadPartialVersionSpecificData for ModernWithEntrySize {
    #[inline]
    fn maybe_read_entry_size(c: bool, bytes: &[u8], i: &mut usize) -> ParseFileResult<Option<i32>> {
        maybe_read_int(c, bytes, i)
    }

    #[inline]
    fn maybe_read_arcshpod(c: bool, bytes: &[u8], i: &mut usize)
        -> ParseFileResult<Option<ByteSingle>> {
        if c {
            Ok(Single(read_single(bytes, i)?))
        } else {
            *i += 4;
            Ok(None)
        }
    }

    #[inline]
    fn maybe_read_mod_combo_star_ratings(c: bool, bytes: &[u8], i: &mut usize)
        -> ParseFileResult<(Option<i32>, Option<Vec<(i32, f64)>>)> {
        let num_int_doubles = read_int(bytes, i)?;
        if c {
            let mut int_double_pairs = Vec::with_capacity(num_int_doubles as usize);
            for _ in 0..num_int_doubles {
                int_double_pairs.push(read_int_double_pair(bytes, i)?);
            }
            Ok((Some(num_int_doubles), Some(int_double_pairs)))
        } else {
            *i += num_int_doubles as usize * 14;
            Ok(None, None)
        }
    }

    #[inline]
    fn maybe_read_unknown_short(_c: bool, _bytes: &[u8], _i: &mut usize)
        -> ParseFileResult<Option<i16>> {
        Ok(None)
    }
}