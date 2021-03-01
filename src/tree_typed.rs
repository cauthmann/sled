#![cfg(feature = "experimental_typed_api")]

use std::{
    fmt::{self, Debug, Display},
    ops::{Deref, RangeBounds},
};

use crate::{
    batch_typed::TypedBatch,
    encoding::{Decoder, Encoder},
    tree::{CompareAndSwapError, Tree},
    Error, IVec, Iter, Result, Subscriber,
};

/// A wrapper around regular `[Tree]`s with a different, typed API.
///
pub struct TypedTree<K, V> {
    tree: Tree,
    _k: std::marker::PhantomData<K>,
    _v: std::marker::PhantomData<V>,
}

impl<K, V> Deref for TypedTree<K, V> {
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

    /// Create a new TypedBatch that matches the encodings of this tree
    pub fn make_batch(&self) -> TypedBatch<K, V> {
        TypedBatch::<K, V>::default()
    }

    /// Apply a matching TypedBatch
    pub fn apply_batch(&self, batch: TypedBatch<K, V>) -> Result<()> {
        self.tree.apply_batch(batch.batch)
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

    /// Compare and swap
    pub fn compare_and_swap<'a, 'b, 'c>(
        &self,
        key: <K as Encoder<'a>>::In,
        old: Option<<V as Encoder<'b>>::In>,
        new: Option<<V as Encoder<'c>>::In>,
    ) -> TypedCompareAndSwapResult<K, V> {
        let key = K::encode(key);
        let old = old.map(|v| V::encode(v));
        let new = new.map(|v| V::encode(v));
        Ok(self
            .tree
            .compare_and_swap(
                key,
                old.as_ref().map(|v| v.as_ref()),
                new.as_ref().map(|v| v.as_ref()),
            )?
            .map_err(|e| TypedCompareAndSwapError {
                current: e.current.map(|ivec| ivec.into_encoding()),
                proposed: e.proposed.map(|ivec| ivec.into_encoding()),
            }))
    }

    /// Fetch the value, apply a function to it and return the result.
    pub fn update_and_fetch<'a, F>(
        &self,
        key: <K as Encoder<'a>>::In,
        mut f: F,
    ) -> Result<Option<IVec<V>>>
    where
        F: FnMut(Option<IVec<V>>) -> Option<IVec<V>>,
    {
        let key = K::encode(key);
        let key_ref = key.as_ref();
        let mut current = self.tree.get(key_ref)?;

        loop {
            let next =
                f(current.as_ref().map(|ivec| ivec.clone().into_encoding()));
            match self.tree.compare_and_swap::<_, _, IVec>(
                key_ref,
                current,
                next.as_ref().map(|ivec| ivec.clone().into_encoding()),
            )? {
                Ok(()) => return Ok(next),
                Err(CompareAndSwapError { current: cur, .. }) => {
                    current = cur;
                }
            }
        }
    }

    /// Fetch the value, apply a function to it and return the previous value.
    pub fn fetch_and_update<'a, F>(
        &self,
        key: <K as Encoder<'a>>::In,
        mut f: F,
    ) -> Result<Option<IVec<V>>>
    where
        F: FnMut(Option<IVec<V>>) -> Option<IVec<V>>,
    {
        let key = K::encode(key);
        let key_ref = key.as_ref();
        let mut current = self.tree.get(key_ref)?;

        loop {
            let next =
                f(current.as_ref().map(|ivec| ivec.clone().into_encoding()));
            match self.tree.compare_and_swap::<_, _, IVec>(
                key_ref,
                current.clone(),
                next.as_ref().map(|ivec| ivec.clone().into_encoding()),
            )? {
                Ok(()) => return Ok(current.map(|ivec| ivec.into_encoding())),
                Err(CompareAndSwapError { current: cur, .. }) => {
                    current = cur;
                }
            }
        }
    }

    /// TODO
    pub fn watch_prefix<P: AsRef<[u8]>>(&self, _prefix: P) -> Subscriber {
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
        _key: <K as Encoder<'a>>::In,
        _value: <V as Encoder<'b>>::In,
    ) -> Result<Option<IVec>> {
        todo!()
    }

    /// TODO
    pub fn iter(&self) -> Iter {
        todo!()
    }

    /// TODO
    pub fn range<R>(&self, _range: ()) -> Iter {
        todo!()
    }

    /// TODO: is the encoder useful here, or do we expect the user to pass in &[u8]?
    pub fn scan_prefix(&self, _prefix: ()) -> Iter {
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

/// Typed Compare and swap result.
pub type TypedCompareAndSwapResult<K, V> =
    Result<std::result::Result<(), TypedCompareAndSwapError<K, V>>>;

impl<K, V> From<Error> for TypedCompareAndSwapResult<K, V> {
    fn from(error: Error) -> Self {
        Err(error)
    }
}

/// Typed Compare and swap error.
#[derive(Clone)]
pub struct TypedCompareAndSwapError<K, V> {
    /// The current value which caused your CAS to fail.
    pub current: Option<IVec<K>>,
    /// Returned value that was proposed unsuccessfully.
    pub proposed: Option<IVec<V>>,
}
impl<K, V> Debug for TypedCompareAndSwapError<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Neither K nor V are Debug, nor should they be.
        // We cannot really implement this ourselves, but we can use the existing formatter:
        let case = CompareAndSwapError {
            current: self
                .current
                .as_ref()
                .map(|ivec| ivec.clone().into_encoding()),
            proposed: self
                .proposed
                .as_ref()
                .map(|ivec| ivec.clone().into_encoding()),
        };
        Debug::fmt(&case, f)
    }
}

impl<K, V> Display for TypedCompareAndSwapError<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Compare and swap conflict")
    }
}

impl<K, V> std::error::Error for TypedCompareAndSwapError<K, V> {}
