use arboretum_core::{Prefix, PropsType};
use rkyv::{AlignedVec, Deserialize, Infallible};
use std::{
    collections::HashMap,
    ops::Bound,
    path::{Path, PathBuf},
};

pub struct SledGraph<NodeProps, EdgeProps>
where
    NodeProps: PropsType,
    EdgeProps: PropsType,
{
    path: PathBuf,
    db: sled::Db,
    node_props: sled::Tree,
    edge_props: sled::Tree,
    spo: sled::Tree,
    pos: sled::Tree,
    osp: sled::Tree,
    phantom: std::marker::PhantomData<(NodeProps, EdgeProps)>,
}

impl<NodeProps, EdgeProps> SledGraph<NodeProps, EdgeProps>
where
    NodeProps: PropsType,
    NodeProps::Archived: Deserialize<NodeProps, rkyv::Infallible>,
    EdgeProps: PropsType,
    EdgeProps::Archived: Deserialize<EdgeProps, rkyv::Infallible>,
{
    pub fn min_value(&self) -> u64 {
        u64::MIN
    }

    pub fn max_value(&self) -> u64 {
        u64::MAX
    }

    pub fn get_memory_usage(&self) -> anyhow::Result<usize> {
        Ok(0)
    }

    fn next_node_id(&self) -> anyhow::Result<u64> {
        let mut result = None;
        self.db.fetch_and_update(b"next_node_id", |old| {
            let number = match old {
                Some(bytes) => {
                    let array: [u8; 8] = bytes.try_into().unwrap();
                    let number = u64::from_be_bytes(array);
                    number + 1
                }
                None => 0,
            };
            result = Some(number - 1);

            Some(number.to_be_bytes().to_vec())
        })?;

        result.ok_or_else(|| anyhow::anyhow!("Sled DB is missing next_node_id."))
    }

    fn next_edge_id(&self) -> anyhow::Result<u64> {
        let mut result = None;
        self.db.fetch_and_update(b"next_edge_id", |old| {
            let number = match old {
                Some(bytes) => {
                    let array: [u8; 8] = bytes.try_into().unwrap();
                    let number = u64::from_be_bytes(array);
                    number + 1
                }
                None => 1,
            };
            result = Some(number);

            Some(number.to_be_bytes().to_vec())
        })?;

        result.ok_or_else(|| anyhow::anyhow!("Sled DB is missing next_edge_id."))
    }

    pub fn from_file<'a, P: AsRef<Path>>(
        path: P,
    ) -> anyhow::Result<SledGraph<NodeProps, EdgeProps>> {
        let db = sled::open(path.as_ref())?;

        let node_props = db.open_tree("node_props")?;
        let edge_props = db.open_tree("edge_props")?;
        let spo = db.open_tree("spo")?;
        let pos = db.open_tree("pos")?;
        let osp = db.open_tree("osp")?;

        // Initialize next_node_id to 0 if it doesn't exist.
        db.fetch_and_update(b"next_node_id", |old| match old {
            Some(bytes) => Some(bytes.to_vec()),
            None => Some(0u64.to_be_bytes().to_vec()),
        })?;

        // Initialize next_edge_id to 1 if it doesn't exist.
        db.fetch_and_update(b"next_edge_id", |old| match old {
            Some(bytes) => Some(bytes.to_vec()),
            None => Some(1u64.to_be_bytes().to_vec()),
        })?;

        Ok(Self {
            path: path.as_ref().into(),
            db,
            node_props,
            edge_props,
            spo,
            pos,
            osp,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn fetch_and_update_node_props<F>(
        &self,
        key: u64,
        mut f: F,
    ) -> anyhow::Result<Option<NodeProps>>
    where
        F: FnMut(Option<&NodeProps>) -> Option<NodeProps>,
    {
        let mut result = None;

        self.node_props.fetch_and_update(key.to_be_bytes(), |old| {
            result = old.map(|bytes| {
                let mut aligned = AlignedVec::with_capacity(bytes.len());
                aligned.extend_from_slice(bytes.as_ref());

                <NodeProps::Archived as Deserialize<NodeProps, Infallible>>::deserialize(
                    unsafe { rkyv::archived_root::<NodeProps>(aligned.as_ref()) },
                    &mut Infallible,
                )
                .unwrap()
            });

            f(result.as_ref()).map(|props| rkyv::to_bytes(&props).unwrap().to_vec())
        })?;

        Ok(result)
    }

    pub fn add_node(&self) -> anyhow::Result<u64> {
        Ok(self.next_node_id()?)
    }

    pub fn add_node_with_props(&self, props: NodeProps) -> anyhow::Result<u64> {
        let id = self.next_node_id()?;
        self.node_props
            .insert(id.to_be_bytes(), rkyv::to_bytes(&props)?.as_ref())?;
        Ok(id)
    }

    pub fn set_node_props(&self, id: u64, props: NodeProps) -> anyhow::Result<()> {
        self.node_props
            .insert(id.to_be_bytes(), rkyv::to_bytes(&props)?.as_ref())?;
        Ok(())
    }

    pub fn add_edge(&self, (s, p, o): (u64, u64, u64)) -> anyhow::Result<()> {
        let mut triple_bytes = [0u8; 24];
        let zero = 0u64.to_be_bytes();

        // SPO
        triple_to_bytes((s, p, o), &mut triple_bytes);
        self.spo.insert(&triple_bytes, &zero)?;

        // POS
        triple_to_bytes((p, o, s), &mut triple_bytes);
        self.pos.insert(&triple_bytes, &zero)?;

        // OSP
        triple_to_bytes((o, s, p), &mut triple_bytes);
        self.osp.insert(&triple_bytes, &zero)?;

        Ok(())
    }

    pub fn add_edge_with_props(
        &self,
        (s, p, o): (u64, u64, u64),
        props: EdgeProps,
    ) -> anyhow::Result<()> {
        let id = self.next_edge_id()?;
        self.edge_props
            .insert(id.to_be_bytes(), rkyv::to_bytes(&props)?.as_ref())?;

        let mut triple_bytes = [0u8; 24];
        let id_bytes = id.to_be_bytes();

        // SPO
        triple_to_bytes((s, p, o), &mut triple_bytes);
        self.spo.insert(&triple_bytes, &id_bytes)?;

        // POS
        triple_to_bytes((p, o, s), &mut triple_bytes);
        self.pos.insert(&triple_bytes, &id_bytes)?;

        // OSP
        triple_to_bytes((o, s, p), &mut triple_bytes);
        self.osp.insert(&triple_bytes, &id_bytes)?;

        Ok(())
    }

    pub fn get_node_props(&self, id: u64) -> anyhow::Result<Option<NodeProps>> {
        Ok(self.node_props.get(&id.to_be_bytes())?.map(|bytes| {
            let mut aligned = AlignedVec::with_capacity(bytes.len());
            aligned.extend_from_slice(bytes.as_ref());

            <NodeProps::Archived as Deserialize<NodeProps, Infallible>>::deserialize(
                unsafe { rkyv::archived_root::<NodeProps>(aligned.as_ref()) },
                &mut Infallible,
            )
            .unwrap()
        }))
    }

    pub fn iter_node_props(&self) -> anyhow::Result<impl Iterator<Item = (u64, NodeProps)>> {
        let v = self
            .node_props
            .iter()
            .map(|r| -> anyhow::Result<(u64, NodeProps)> {
                let (k, v) = r?;

                let k = u64::from_be_bytes(k.as_ref().try_into()?);

                let mut aligned = AlignedVec::with_capacity(v.len());
                aligned.extend_from_slice(v.as_ref());
                let v = <NodeProps::Archived as Deserialize<NodeProps, Infallible>>::deserialize(
                    unsafe { rkyv::archived_root::<NodeProps>(aligned.as_ref()) },
                    &mut Infallible,
                )
                .unwrap();

                Ok((k, v))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(v.into_iter())
    }

    pub fn prefix_edges_spo(
        &self,
        prefix: Prefix<u64>,
    ) -> anyhow::Result<impl Iterator<Item = (u64, u64, u64, Option<EdgeProps>)>> {
        let (start_bound, end_bound) = {
            match prefix {
                Prefix::One(a) => (
                    build_bound((a, u64::MIN, u64::MIN)),
                    build_bound((a, u64::MAX, u64::MAX)),
                ),

                Prefix::Two(a, b) => (build_bound((a, b, u64::MIN)), build_bound((a, b, u64::MAX))),

                Prefix::Three(a, b, c) => (build_bound((a, b, c)), build_bound((a, b, c))),
            }
        };

        let mut result = Vec::new();
        for r in self.spo.range((start_bound, end_bound)) {
            let (k, v_bytes) = r?;

            let (s, p, o) = bytes_to_triple(k.as_ref());

            let v = u64::from_be_bytes(v_bytes.as_ref().try_into()?);

            let edge_props = if v != 0 {
                self.edge_props.get(v_bytes)?.map(|bytes| {
                    let mut aligned = AlignedVec::with_capacity(bytes.len());
                    aligned.extend_from_slice(bytes.as_ref());
                    <EdgeProps::Archived as Deserialize<EdgeProps, Infallible>>::deserialize(
                        unsafe { rkyv::archived_root::<EdgeProps>(aligned.as_ref()) },
                        &mut Infallible,
                    )
                    .unwrap()
                })
            } else {
                None
            };

            result.push((s, p, o, edge_props))
        }

        Ok(result.into_iter())
    }

    pub fn prefix_edges_pos(
        &self,
        prefix: Prefix<u64>,
    ) -> anyhow::Result<impl Iterator<Item = (u64, u64, u64, Option<EdgeProps>)>> {
        let (start_bound, end_bound) = {
            match prefix {
                Prefix::One(a) => (
                    build_bound((a, u64::MIN, u64::MIN)),
                    build_bound((a, u64::MAX, u64::MAX)),
                ),

                Prefix::Two(a, b) => (build_bound((a, b, u64::MIN)), build_bound((a, b, u64::MAX))),

                Prefix::Three(a, b, c) => (build_bound((a, b, c)), build_bound((a, b, c))),
            }
        };

        let mut result = Vec::new();
        for r in self.spo.range((start_bound, end_bound)) {
            let (k, v_bytes) = r?;

            let (p, o, s) = bytes_to_triple(k.as_ref());

            let v = u64::from_be_bytes(v_bytes.as_ref().try_into()?);

            let edge_props = if v != 0 {
                self.edge_props.get(v_bytes)?.map(|bytes| {
                    let mut aligned = AlignedVec::with_capacity(bytes.len());
                    aligned.extend_from_slice(bytes.as_ref());
                    <EdgeProps::Archived as Deserialize<EdgeProps, Infallible>>::deserialize(
                        unsafe { rkyv::archived_root::<EdgeProps>(aligned.as_ref()) },
                        &mut Infallible,
                    )
                    .unwrap()
                })
            } else {
                None
            };

            result.push((s, p, o, edge_props))
        }

        Ok(result.into_iter())
    }

    pub fn prefix_edges_osp(
        &self,
        prefix: Prefix<u64>,
    ) -> anyhow::Result<impl Iterator<Item = (u64, u64, u64, Option<EdgeProps>)>> {
        let (start_bound, end_bound) = {
            match prefix {
                Prefix::One(a) => (
                    build_bound((a, u64::MIN, u64::MIN)),
                    build_bound((a, u64::MAX, u64::MAX)),
                ),

                Prefix::Two(a, b) => (build_bound((a, b, u64::MIN)), build_bound((a, b, u64::MAX))),

                Prefix::Three(a, b, c) => (build_bound((a, b, c)), build_bound((a, b, c))),
            }
        };

        let mut result = Vec::new();
        for r in self.spo.range((start_bound, end_bound)) {
            let (k, v_bytes) = r?;

            let (o, s, p) = bytes_to_triple(k.as_ref());

            let v = u64::from_be_bytes(v_bytes.as_ref().try_into()?);

            let edge_props = if v != 0 {
                self.edge_props.get(v_bytes)?.map(|bytes| {
                    let mut aligned = AlignedVec::with_capacity(bytes.len());
                    aligned.extend_from_slice(bytes.as_ref());
                    <EdgeProps::Archived as Deserialize<EdgeProps, Infallible>>::deserialize(
                        unsafe { rkyv::archived_root::<EdgeProps>(aligned.as_ref()) },
                        &mut Infallible,
                    )
                    .unwrap()
                })
            } else {
                None
            };

            result.push((s, p, o, edge_props))
        }

        Ok(result.into_iter())
    }

    pub fn extend_with(
        &self,
        (node_props, edges): (
            HashMap<u64, NodeProps>,
            HashMap<(u64, u64, u64), Option<EdgeProps>>,
        ),
    ) -> anyhow::Result<()> {
        for (id, value) in node_props {
            self.set_node_props(id, value)?;
        }

        for (edge_key, value) in edges {
            match value {
                Some(value) => self.add_edge_with_props(edge_key, value)?,
                None => self.add_edge(edge_key)?,
            }
        }

        Ok(())
    }
}

impl<NodeProps, EdgeProps> std::fmt::Debug for SledGraph<NodeProps, EdgeProps>
where
    NodeProps: PropsType,
    EdgeProps: PropsType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SledGraph{\n")?;
        f.write_fmt(format_args!(
            "  path: \"{}\"\n",
            self.path.to_string_lossy()
        ))?;
        f.write_fmt(format_args!(
            "  # node props: \"{}\"\n",
            self.node_props.len()
        ))?;
        f.write_fmt(format_args!(
            "  # edge props: \"{}\"\n",
            self.edge_props.len()
        ))?;
        f.write_fmt(format_args!("  # spo edges: \"{}\"\n", self.spo.len()))?;
        f.write_fmt(format_args!("  # pos edges: \"{}\"\n", self.pos.len()))?;
        f.write_fmt(format_args!("  # osp edges: \"{}\"\n", self.osp.len()))?;
        f.write_str("}\n")
    }
}

fn triple_to_bytes(triple: (u64, u64, u64), out_bytes: &mut [u8; 24]) {
    out_bytes[0..8].copy_from_slice(&triple.0.to_be_bytes());
    out_bytes[8..16].copy_from_slice(&triple.1.to_be_bytes());
    out_bytes[16..24].copy_from_slice(&triple.2.to_be_bytes());
}

fn bytes_to_triple(bytes: &[u8]) -> (u64, u64, u64) {
    (
        u64::from_be_bytes(bytes[0..8].try_into().unwrap()),
        u64::from_be_bytes(bytes[8..16].try_into().unwrap()),
        u64::from_be_bytes(bytes[16..24].try_into().unwrap()),
    )
}

fn build_bound(bound: (u64, u64, u64)) -> Bound<[u8; 24]> {
    let mut b = [0u8; 24];
    triple_to_bytes(bound, &mut b);
    Bound::Included(b)
}
