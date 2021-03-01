#![allow(unused_results)]

use super::*;

/// A batch of updates that will be applied atomically to the Tree.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let db = sled::Config::default().temporary(true).open()?;
/// let tree = db
///     .open_tree("batch")?
///     .with_encodings::<sled::IntegerEncoding<u128>, sled::StringEncoding>();
///
/// tree.insert(1, "January")?;
///
/// let mut batch = tree.make_batch();
///
/// batch.insert(2, "February");
/// batch.insert(3, "March");
/// batch.insert(4, "April");
/// batch.remove(1);
///
/// tree.apply_batch(batch)?;
/// // key 1 no longer exists, and keys 2, 3 and 4 now do exist.
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedBatch<K, V> {
    pub(crate) batch: Batch,
    _k: std::marker::PhantomData<K>,
    _v: std::marker::PhantomData<V>,
}

impl<K, V> Default for TypedBatch<K, V> {
    fn default() -> Self {
        TypedBatch {
            batch: Batch::default(),
            _k: std::marker::PhantomData,
            _v: std::marker::PhantomData,
        }
    }
}

impl<K, V> TypedBatch<K, V>
where
    for<'a> K: Encoder<'a>,
    for<'a> V: Encoder<'a>,
{
    /// Set a key to a new value
    pub fn insert<'a, 'b>(
        &mut self,
        key: <K as Encoder<'a>>::In,
        value: <V as Encoder<'b>>::In,
    ) {
        self.batch.insert(
            IVec::<K>::encode(key).into_encoding(),
            IVec::<V>::encode(value).into_encoding(),
        );
    }

    /// Remove a key
    pub fn remove<'a>(&mut self, key: <K as Encoder<'a>>::In) {
        self.batch.remove(IVec::<K>::encode(key).into_encoding());
    }

    /// Get a value if it is present in the `Batch`.
    /// `Some(None)` means it's present as a deletion.
    pub fn get<'a>(
        &self,
        key: <K as Encoder<'a>>::In,
    ) -> Option<Option<IVec<V>>> {
        self.batch
            .get(IVec::<K>::encode(key).into_encoding::<()>())
            .map(|res| res.map(|ivec| ivec.clone().into_encoding()))
    }
}
