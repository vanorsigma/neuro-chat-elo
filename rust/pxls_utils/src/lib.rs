use std::io::BufReader;

use serde::Deserialize;
use types::SqliteDump;
pub use types::{PxlsUser, IronmousePxlsUser};

mod types;

pub struct PxlsJsonReader {}

impl PxlsJsonReader {
    pub fn read_pxls_from_json_path<U>(path: &str) -> Result<Vec<U>, anyhow::Error>
    where
        U: for<'a> Deserialize<'a>,
    {
        let sqlite_dump: SqliteDump<U> =
            serde_json::from_reader(BufReader::new(std::fs::File::open(path)?))
                .map_err(|e| anyhow::anyhow!(e))?;
        Ok(sqlite_dump.users)
    }
}
