use std::collections::{BTreeMap, BTreeSet};

pub fn btree_example() {
    // BTreeMap: Chave-valor ordenada
    let mut map = BTreeMap::new();
    map.insert(3, "três");
    map.insert(1, "um");
    map.insert(2, "dois");

    println!("BTreeMap: {:?}", map);

    // BTreeSet: Conjunto de elementos ordenados
    let mut set = BTreeSet::new();
    set.insert(3);
    set.insert(1);
    set.insert(2);

    println!("BTreeSet: {:?}", set);
}