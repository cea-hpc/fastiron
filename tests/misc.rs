use std::collections::HashMap;

#[test]
pub fn map_behavior() {
    let mut map: HashMap<usize, usize> = Default::default();
    map.insert(0, map.len());
    map.insert(1, map.len());
    map.insert(2, map.len());
    assert_eq!(map[&0], 0);
    assert_eq!(map[&1], 1);
    assert_eq!(map[&2], 2);
}