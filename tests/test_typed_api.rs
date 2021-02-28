#![cfg(feature = "experimental_typed_api")]

#[test]
fn test_typed_api() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // This is a simple test to make sure each API function works as expected,
    // For a quick run, use:
    // cargo test --features experimental_typed_api --test test_typed_api --release

    let db = sled::Config::default().temporary(true).open()?;

    // We open a tree and specify the desired encodings for keys and values
    let tree = db
        .open_tree("id_to_name")?
        .with_encodings::<sled::IntegerEncoding<u128>, sled::StringEncoding>();

    // We can now insert entries without manually converting anything into bytes
    assert_eq!(tree.insert(1, "January")?.is_none(), true);
    assert_eq!(tree.insert(2, "Febuary")?.is_none(), true);
    assert_eq!(tree.insert(3, "March")?.is_none(), true);
    assert_eq!(tree.insert(4, "April")?.is_none(), true);
    assert_eq!(tree.insert(5, "May")?.is_none(), true);
    assert_eq!(tree.insert(6, "June")?.is_none(), true);
    assert_eq!(tree.insert(7, "July")?.is_none(), true);
    assert_eq!(tree.insert(8, "August")?.is_none(), true);
    assert_eq!(tree.insert(9, "September")?.is_none(), true);
    assert_eq!(tree.insert(10, "October")?.is_none(), true);
    assert_eq!(tree.insert(11, "November")?.is_none(), true);
    assert_eq!(tree.insert(12, "December")?.is_none(), true);

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

// TODO: Iterators
// TODO: CompareAndSwap
// TODO: Batches
// TODO: Transactions
// TODO: Subscribers and Events
