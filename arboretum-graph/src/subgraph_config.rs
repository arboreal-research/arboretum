use std::path::{Path, PathBuf};

use arboretum_core::constant::*;

#[derive(Clone, Debug)]
pub enum SubgraphConfig {
    MmapGraph16 { subgraph_id: u32, rel_path: PathBuf },
    MmapGraph32 { subgraph_id: u32, rel_path: PathBuf },
    MmapGraph64 { rel_path: PathBuf },
    SledGraph64 { rel_path: PathBuf },
}

impl SubgraphConfig {
    pub fn get_memory_estimate(&self, subgraphs_path: &Path) -> anyhow::Result<usize> {
        Ok(match self {
            SubgraphConfig::MmapGraph16 { rel_path, .. } => {
                get_size(subgraphs_path.join(rel_path))?
            }
            SubgraphConfig::MmapGraph32 { rel_path, .. } => {
                get_size(subgraphs_path.join(rel_path))?
            }
            SubgraphConfig::MmapGraph64 { rel_path } => get_size(subgraphs_path.join(rel_path))?,
            SubgraphConfig::SledGraph64 { rel_path } => get_size(subgraphs_path.join(rel_path))?,
        })
    }

    pub fn is_mutable(&self) -> bool {
        match self {
            SubgraphConfig::MmapGraph16 { .. }
            | SubgraphConfig::MmapGraph32 { .. }
            | SubgraphConfig::MmapGraph64 { .. } => false,
            SubgraphConfig::SledGraph64 { .. } => true,
        }
    }

    pub fn from_impl_name(
        impl_name: &str,
        subgraph_id: u32,
        rel_path: PathBuf,
    ) -> Option<SubgraphConfig> {
        match impl_name {
            SUBGRAPH_IMPL_MMAP16 => Some(SubgraphConfig::MmapGraph16 {
                subgraph_id,
                rel_path: rel_path.into(),
            }),
            SUBGRAPH_IMPL_MMAP32 => Some(SubgraphConfig::MmapGraph32 {
                subgraph_id,
                rel_path: rel_path.into(),
            }),
            SUBGRAPH_IMPL_MMAP64 => Some(SubgraphConfig::MmapGraph64 {
                rel_path: rel_path.into(),
            }),
            SUBGRAPH_IMPL_SLED64 => Some(SubgraphConfig::SledGraph64 {
                rel_path: rel_path.into(),
            }),
            _ => None,
        }
    }
}

fn get_size<P: AsRef<Path>>(path: P) -> std::io::Result<usize> {
    let mut total_size = 0;

    // Traverse the directory entries recursively
    if path.as_ref().is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            total_size += get_size(&path)?;
        }
    } else {
        // Add file size
        total_size += std::fs::metadata(path)?.len() as usize;
    }

    Ok(total_size)
}
