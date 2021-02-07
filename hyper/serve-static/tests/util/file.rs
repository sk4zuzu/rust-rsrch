use crate::util;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::io::Write;
use tempfile::NamedTempFile;

pub fn random(size: usize) -> serve_static::Result<(NamedTempFile, String)> {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();

    let mut file = NamedTempFile::new()?;
    log::debug!("{:?}", file);

    file.write_all(rand_string.as_bytes())?;

    Ok((file, util::hash::from_bytes(rand_string.as_bytes())?))
}
