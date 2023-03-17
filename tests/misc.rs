use std::collections::HashMap;

#[test]
pub fn map_behavior() {
    let mut map: HashMap<usize, usize> = Default::default();
    let mut complementary_map: HashMap<usize, usize> = Default::default();
    (0..10).for_each(|jj| {
        (0..5).for_each(|ii| {
            map.insert(jj * 12 + ii, jj * 12 + ii);
        });
        (5..12).for_each(|ii| {
            complementary_map.insert(jj * 12 + ii, jj * 12 + ii);
        });
        //assert_eq!(map.len(), 5);
        //assert_eq!(complementary_map.len(), 7);
    });

    map.extend(complementary_map.iter());
    assert_eq!(map.len(), 120);
    map.values().for_each(|vv| {
        if *vv == map.len() {
            panic!();
        }
    });
}

#[test]
pub fn filter_and_count() {
    let list = [None, None, Some(123), Some(91), None];
    let n_some = list.iter().filter(|elem| elem.is_some()).count();
    assert_eq!(n_some, 2);
}

#[test]
pub fn position() {
    let arr: [Option<usize>; 6] = [None, None, Some(0), None, Some(1), None];
    let idx = arr.iter().rev().position(|elem| elem.is_some()).unwrap();
    assert_eq!(4, 6 - 1 - idx); // reverse the index
}
