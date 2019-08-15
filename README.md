# indexed_map

 A wrapper for `HashMap` that maps each value to a uniquely generated `usize` key.

![Crates.io](https://img.shields.io/crates/v/indexed_map)
![Crates.io](https://img.shields.io/crates/l/indexed_map)
[![Build Status](https://travis-ci.com/cameronp98/indexed-map.svg?branch=master)](https://travis-ci.com/cameronp98/indexed-map)

Example usage:

```rust
use indexed_map::IndexedMap;

fn  main() {
    // create an empty `IndexedMap`, just like `HashMap`
    let mut fruits = IndexedMap::new();
    
    // insert some values and store their keys for later use
    let apple = fruits.insert("Apple");
    let orange = fruits.insert("Orange");
    let pear = fruits.insert("Pear");
    
    // list the values we've inserted
    for fruit in fruits.inner().values() {
        println!("{}", fruit);
    }
    
    // remove the values using the unique keys returned from `IndexedMap::insert`
    fruits.inner_mut().remove(&apple);
    fruits.inner_mut().remove(&orange);
    fruits.inner_mut().remove(&pear);
    
    // the map is now empty
    println!("{:?}", fruits);
}
```
