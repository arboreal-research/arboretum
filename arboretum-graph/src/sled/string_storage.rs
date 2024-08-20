use std::path::Path;

use crate::Error;

pub struct SledStringStorage {
    str_to_id: sled::Tree,
    id_to_str: sled::Tree,
}

impl SledStringStorage {
    pub fn from_folder<P: AsRef<Path>>(path: P) -> Result<SledStringStorage, Error> {
        let db = sled::open(path)?;

        let str_to_id = db.open_tree("str_to_id")?;
        let id_to_str = db.open_tree("id_to_str")?;

        Ok(SledStringStorage {
            str_to_id,
            id_to_str,
        })
    }

    pub fn assoc<S: AsRef<str>>(&self, name: S, id: u64) -> Result<(), Error> {
        let name_bytes = name.as_ref().as_bytes();
        let id_bytes = id.to_be_bytes();

        self.str_to_id.insert(name_bytes, &id_bytes)?;
        self.id_to_str.insert(&id_bytes, name_bytes)?;
        Ok(())
    }

    pub fn get_str(&self, id: u64) -> Result<Option<String>, Error> {
        let id_bytes = id.to_be_bytes();
        Ok(self
            .id_to_str
            .get(&id_bytes)?
            .map(|bytes| String::from_utf8_lossy(bytes.as_ref()).into_owned()))
    }

    pub fn get_id<S: AsRef<str>>(&self, name: S) -> Result<Option<u64>, Error> {
        let name_bytes = name.as_ref().as_bytes();

        Ok(self
            .str_to_id
            .get(name_bytes)?
            .map(|bytes| u64::from_be_bytes(bytes.as_ref().try_into().unwrap())))
    }

    pub fn get_or_assoc<S: AsRef<str>>(&self, name: S, proposed_id: u64) -> Result<u64, Error> {
        let name_bytes = name.as_ref().as_bytes();
        let proposed_bytes = proposed_id.to_be_bytes();

        let result = self
            .str_to_id
            .fetch_and_update(name_bytes, |old| match old {
                Some(actual_id) => Some(actual_id.to_vec()),
                None => Some(proposed_bytes.to_vec()),
            })?
            .map(|bytes| u64::from_be_bytes(bytes.as_ref().try_into().unwrap()))
            .unwrap_or(proposed_id);

        if result == proposed_id {
            self.id_to_str.insert(proposed_bytes, name_bytes)?;
        }

        Ok(result)
    }
}
