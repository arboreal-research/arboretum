use std::path::Path;

use arboretum_core::split_u64;

/// A sled based association of strings to ids.
///
/// This is used to support named nodes.
pub struct SledStringStorage {
    str_to_id: sled::Tree,
    id_to_str: sled::Tree,
}

impl SledStringStorage {
    /// Open the sled database at the given path and use it to implement a sled based string to id.
    pub fn from_folder<P: AsRef<Path>>(path: P) -> anyhow::Result<SledStringStorage> {
        let db = sled::open(path)?;

        let str_to_id = db.open_tree("str_to_id")?;
        let id_to_str = db.open_tree("id_to_str")?;

        Ok(SledStringStorage {
            str_to_id,
            id_to_str,
        })
    }

    pub fn dump(&self) {
        for r in self.id_to_str.iter() {
            let (k, v) = r.expect("ok");
            let k = u64::from_be_bytes(k.as_ref().try_into().unwrap());
            let v = String::from_utf8_lossy(v.as_ref()).into_owned();

            let (k_hi, k_low) = split_u64(k);

            if k_low > 289070 && k_low < 289090 {
                if v.len() > 75 {
                    println!("({}, {}) -> <{} bytes>", k_hi, k_low, v.len());
                } else {
                    println!("({}, {}) -> '{}'", k_hi, k_low, v);
                }
            }
        }
    }

    /// Associates `name` with `id`.
    pub fn assoc<S: AsRef<str>>(&self, name: S, id: u64) -> anyhow::Result<()> {
        let name_bytes = name.as_ref().as_bytes();
        let id_bytes = id.to_be_bytes();

        self.str_to_id.insert(name_bytes, &id_bytes)?;
        self.id_to_str.insert(&id_bytes, name_bytes)?;
        Ok(())
    }

    /// Gets the string associated with `id`.
    pub fn get_str(&self, id: u64) -> anyhow::Result<Option<String>> {
        let id_bytes = id.to_be_bytes();
        Ok(self
            .id_to_str
            .get(&id_bytes)?
            .map(|bytes| String::from_utf8_lossy(bytes.as_ref()).into_owned()))
    }

    /// Gets the id associated with `name`.
    pub fn get_id<S: AsRef<str>>(&self, name: S) -> anyhow::Result<Option<u64>> {
        let name_bytes = name.as_ref().as_bytes();

        Ok(self
            .str_to_id
            .get(name_bytes)?
            .map(|bytes| u64::from_be_bytes(bytes.as_ref().try_into().unwrap())))
    }

    /// Gets the id currently associated with `name`.
    ///
    /// If there is no id currently associated, then associates `proposed_id` and returns it.
    pub fn get_or_assoc<S: AsRef<str>>(&self, name: S, proposed_id: u64) -> anyhow::Result<u64> {
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
