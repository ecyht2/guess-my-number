use std::collections::BTreeMap;

use guess_my_number_rs::histogram::Histogram;

#[test]
fn constructor() {
    // From BTreeMap
    let map = BTreeMap::from([(69, 69), (420, 420), (21, 4)]);

    let histogram = Histogram::new(map.clone());
    assert_eq!(histogram.data(), &map);

    // From Vector
    let mut v = vec![69; 69];
    v.append(&mut vec![420; 420]);
    v.append(&mut vec![21; 4]);

    let histogram = Histogram::from_vec(v);
    assert_eq!(histogram.data(), &map);
}

#[test]
fn data_setter() {
    // Creating Object
    let map = BTreeMap::<u32, u128>::new();

    let mut histogram = Histogram::new(map);

    // BTreeMap Setters
    let map = BTreeMap::from([(69, 69), (420, 420), (21, 4)]);

    histogram.set_data(map.clone());
    assert_eq!(histogram.data(), &map);

    // Vector Setters
    let v = vec![1, 1, 69, 420, 3, 3, 3, 3];
    let map = BTreeMap::from([(1, 2), (69, 1), (420, 1), (3, 4)]);

    histogram.set_data_list(v);
    assert_eq!(histogram.data(), &map);
}

#[test]
fn histogram_range() {
    let map = BTreeMap::from([(69, 69), (420, 420), (21, 4)]);

    let histogram = Histogram::new(map.clone());
    assert_eq!(histogram.get_highest(), (&420, &420));
    assert_eq!(histogram.get_lowest(), (&21, &4));
}
