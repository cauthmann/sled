#![cfg(feature = "experimental_typed_api")]

// These are simple tests to make sure each API function works as expected,
// For a quick run, use:
// cargo test --features experimental_typed_api --test test_typed_api --release

#[test]
fn test_typed_api() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = sled::Config::default().temporary(true).open()?;

    // We open a tree and specify the desired encodings for keys and values
    let tree = db
        .open_tree("id_to_name")?
        .with_encodings::<sled::IntegerEncoding<u128>, sled::StringEncoding>();

    // We can now insert entries without manually converting anything into bytes
    assert!(tree.insert(1, "January")?.is_none());
    assert!(tree.insert(2, "Febuary")?.is_none());
    assert!(tree.insert(3, "March")?.is_none());
    assert!(tree.insert(4, "April")?.is_none());
    assert!(tree.insert(5, "May")?.is_none());
    assert!(tree.insert(6, "June")?.is_none());
    assert!(tree.insert(7, "July")?.is_none());
    assert!(tree.insert(8, "August")?.is_none());
    assert!(tree.insert(9, "September")?.is_none());
    assert!(tree.insert(10, "October")?.is_none());
    assert!(tree.insert(11, "November")?.is_none());
    assert!(tree.insert(12, "December")?.is_none());

    // Fix spelling
    assert_eq!(tree.insert(2, "February")?.unwrap().decode()?, "Febuary");
    assert_eq!(tree.get(2)?.unwrap().decode()?, "February");

    assert_eq!(tree.contains_key(13)?, false);
    assert_eq!(tree.contains_key(3)?, true);

    // Get the first and last month of the year
    assert_eq!(tree.first()?.unwrap().0.decode()?, 1);
    assert_eq!(tree.first()?.unwrap().1.decode()?, "January");
    assert_eq!(tree.last()?.unwrap().0.decode()?, 12);
    assert_eq!(tree.last()?.unwrap().1.decode()?, "December");

    // what's around March?
    assert_eq!(tree.get_lt(3)?.unwrap().0.decode()?, 2);
    assert_eq!(tree.get_lt(3)?.unwrap().1.decode()?, "February");
    assert_eq!(tree.get_gt(3)?.unwrap().0.decode()?, 4);
    assert_eq!(tree.get_gt(3)?.unwrap().1.decode()?, "April");

    // Remove December, January and February
    let max = tree.pop_max()?.unwrap();
    assert_eq!(max.0.decode()?, 12);
    assert_eq!(max.1.decode()?, "December");
    assert_eq!(tree.last()?.unwrap().0.decode()?, 11);
    assert_eq!(tree.last()?.unwrap().1.decode()?, "November");

    let min = tree.pop_min()?.unwrap();
    assert_eq!(min.0.decode()?, 1);
    assert_eq!(min.1.decode()?, "January");
    assert_eq!(tree.first()?.unwrap().0.decode()?, 2);
    assert_eq!(tree.first()?.unwrap().1.decode()?, "February");

    assert_eq!(tree.remove(2)?.unwrap().decode()?, "February");

    assert_eq!(tree.len(), 9);

    Ok(())
}

#[test]
fn test_typed_batches() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = sled::Config::default().temporary(true).open()?;

    // We open a tree and specify the desired encodings for keys and values
    let tree = db
        .open_tree("id_to_name")?
        .with_encodings::<sled::IntegerEncoding<u128>, sled::StringEncoding>();

    // Try to insert the months, but using a Batch
    tree.insert(2, "Febuary")?;
    tree.insert(13, "Encore")?;

    let mut batch = tree.make_batch();
    batch.remove(13);
    batch.insert(1, "January");
    batch.insert(2, "February");
    batch.insert(3, "March");
    batch.insert(4, "April");
    batch.insert(5, "May");
    batch.insert(6, "June");
    batch.insert(7, "July");
    batch.insert(8, "August");
    batch.insert(9, "September");
    batch.insert(10, "October");
    batch.insert(11, "November");
    batch.insert(12, "December");

    assert_eq!(batch.get(2).unwrap().unwrap().decode()?, "February");
    assert!(batch.get(13).unwrap().is_none());
    assert!(batch.get(14).is_none());

    tree.apply_batch(batch)?;

    assert_eq!(tree.get(2)?.unwrap().decode()?, "February");
    assert!(tree.get(13)?.is_none());
    assert_eq!(tree.len(), 12);

    Ok(())
}

#[test]
fn test_typed_cas() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = sled::Config::default().temporary(true).open()?;

    let tree = db
        .open_tree("id_to_name")?
        .with_encodings::<sled::IntegerEncoding<u128>, sled::StringEncoding>();

    assert!(tree.get(1)?.is_none());
    let err =
        tree.compare_and_swap(1, Some("Before"), Some("January"))?.unwrap_err();
    assert!(err.current.is_none());
    assert_eq!(err.proposed.unwrap().decode()?, "January");
    assert!(tree.get(1)?.is_none());
    assert!(tree.compare_and_swap(1, None, Some("January"))?.is_ok());
    assert_eq!(tree.get(1)?.unwrap().decode()?, "January");

    assert!(tree.compare_and_swap(1, None, Some("February"))?.is_err());
    assert!(tree
        .compare_and_swap(1, Some("March"), Some("February"))?
        .is_err());
    assert!(tree
        .compare_and_swap(1, Some("January"), Some("February"))?
        .is_ok());
    assert_eq!(tree.get(1)?.unwrap().decode()?, "February");

    Ok(())
}

#[test]
fn test_typed_update_fetch(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = sled::Config::default().temporary(true).open()?;

    // We open a tree and specify the desired encodings for keys and values
    let tree = db
        .open_tree("id_to_counter")?
        .with_encodings::<sled::IntegerEncoding<u128>, sled::IntegerEncoding<u128>>();

    let new_counter = tree.update_and_fetch(1, |old| {
        let new = match old {
            None => 1,
            Some(ivec) => ivec.decode().unwrap() + 1,
        };
        Some(sled::IVec::encode(new))
    })?;
    assert_eq!(new_counter.unwrap().decode()?, 1);
    assert_eq!(tree.get(1)?.unwrap().decode()?, 1);

    let old_counter = tree.fetch_and_update(1, |old| {
        let new = match old {
            None => 1,
            Some(ivec) => ivec.decode().unwrap() + 1,
        };
        Some(sled::IVec::encode(new))
    })?;
    assert_eq!(old_counter.unwrap().decode()?, 1);
    assert_eq!(tree.get(1)?.unwrap().decode()?, 2);

    Ok(())
}

// TODO: Iterators
// TODO: Transactions
// TODO: Subscribers and Events
