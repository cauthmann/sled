#![cfg(feature = "experimental_typed_api")]

use crate::{
    encoding::{Decoder, Encoder},
    tree::*,
    IVec, Result, *,
};

/// A wrapper around regular `[Tree]`s with a different, typed API.
///
pub struct TypedTree<K, V> {
    tree: Tree,
    _k: std::marker::PhantomData<K>,
    _v: std::marker::PhantomData<V>,
}

impl<K, V> std::ops::Deref for TypedTree<K, V> {
    type Target = Tree;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

// Note: we'll implement these one by one.
// The method list should be complete, but signatures of unimplemented methods will change.
impl<K, V> TypedTree<K, V>
where
    for<'a> K: Encoder<'a>,
    //for<'a> K: Decoder<'a>,
    for<'a> V: Encoder<'a>,
    for<'a> V: Decoder<'a>,
{
    /// Create a new typed tree
    pub fn new(tree: Tree) -> Self {
        Self {
            tree: tree,
            _k: std::marker::PhantomData,
            _v: std::marker::PhantomData,
        }
    }

    /// Insert a value
    pub fn insert<'a, 'b>(
        &self,
        key: <K as Encoder<'a>>::In,
        value: <V as Encoder<'b>>::In,
    ) -> Result<Option<IVec<V>>> {
        self.tree
            .insert(K::encode(key), IVec::<V>::encode(value).into_encoding())
            .map(|res| res.map(|ivec| ivec.into_encoding()))
    }

    /// TODO
    pub fn transaction() {
        todo!()
    }

    /// TODO
    pub fn apply_batch(&self, batch: Batch) -> Result<()> {
        todo!()
    }

    /// Get a value
    pub fn get<'a>(
        &self,
        key: <K as Encoder<'a>>::In,
    ) -> Result<Option<IVec<V>>> {
        self.tree
            .get(K::encode(key))
            .map(|res| res.map(|ivec| ivec.into_encoding()))
    }

    /// TODO
    pub fn get_zero_copy() {
        todo!()
    }

    /// Remove an item
    pub fn remove<'a>(
        &self,
        key: <K as Encoder<'a>>::In,
    ) -> Result<Option<IVec<V>>> {
        self.tree
            .remove(K::encode(key))
            .map(|res| res.map(|ivec| ivec.into_encoding()))
    }

    /// TODO
    pub fn compare_and_swap() {
        todo!()
    }

    /// TODO
    pub fn update_and_fetch() {
        todo!()
    }

    /// TODO
    pub fn fetch_and_update() {
        todo!()
    }

    /// TODO
    pub fn watch_prefix<P: AsRef<[u8]>>(&self, prefix: P) -> Subscriber {
        todo!()
    }

    /// contains_key
    pub fn contains_key<'a>(
        &self,
        key: <K as Encoder<'a>>::In,
    ) -> Result<bool> {
        self.tree.contains_key(K::encode(key))
    }

    /// TODO: is this still a useful API? Probably for integer keys, but otherwise?
    pub fn get_lt<'a>(
        &self,
        key: <K as Encoder<'a>>::In,
    ) -> Result<Option<(IVec<K>, IVec<V>)>> {
        self.tree
            .get_lt(K::encode(key))
            .map(|res| res.map(|(k, v)| (k.into_encoding(), v.into_encoding())))
    }

    /// get_gt
    pub fn get_gt<'a>(
        &self,
        key: <K as Encoder<'a>>::In,
    ) -> Result<Option<(IVec<K>, IVec<V>)>> {
        self.tree
            .get_gt(K::encode(key))
            .map(|res| res.map(|(k, v)| (k.into_encoding(), v.into_encoding())))
    }

    /// TODO
    pub fn merge<'a, 'b>(
        &self,
        key: <K as Encoder<'a>>::In,
        value: <V as Encoder<'b>>::In,
    ) -> Result<Option<IVec>> {
        todo!()
    }

    /// TODO
    pub fn iter(&self) -> Iter {
        todo!()
    }

    /// TODO
    pub fn range<R>(&self, range: ()) -> Iter {
        todo!()
    }

    /// TODO: is the encoder useful here, or do we expect the user to pass in &[u8]?
    pub fn scan_prefix(&self, prefix: ()) -> Iter {
        todo!()
    }

    /// Return the first value
    pub fn first(&self) -> Result<Option<(IVec<K>, IVec<V>)>> {
        self.tree
            .first()
            .map(|res| res.map(|(k, v)| (k.into_encoding(), v.into_encoding())))
    }

    /// Return the last value
    pub fn last(&self) -> Result<Option<(IVec<K>, IVec<V>)>> {
        self.tree
            .last()
            .map(|res| res.map(|(k, v)| (k.into_encoding(), v.into_encoding())))
    }

    /// Removes the last value
    pub fn pop_max(&self) -> Result<Option<(IVec<K>, IVec<V>)>> {
        self.tree
            .pop_max()
            .map(|res| res.map(|(k, v)| (k.into_encoding(), v.into_encoding())))
    }

    /// Removes the first value
    pub fn pop_min(&self) -> Result<Option<(IVec<K>, IVec<V>)>> {
        self.tree
            .pop_min()
            .map(|res| res.map(|(k, v)| (k.into_encoding(), v.into_encoding())))
    }
}
