//! # indexed-map
//! HashMap wrapper where each value corresponds to a uniquely generated `usize` key

use std::collections::HashMap;

/// An indexed map
///
/// TODO: replace `inner` methods with full copy of `HashMap` API
#[derive(Debug, Clone)]
pub struct IndexedMap<T> {
    next_key: usize,
    items: HashMap<usize, T>,
}

impl<T> IndexedMap<T> {
    /// Create a new `IndexedMap'
    ///
    /// # Examples
    ///
    /// ```
    /// use indexed_map::IndexedMap;
    /// let mut foo = IndexedMap::new();
    /// let bar = foo.insert("bar");
    /// println!("{:?}", foo);
    /// ```
    pub fn new() -> IndexedMap<T> {
        IndexedMap {
            next_key: 0,
            items: HashMap::new(),
        }
    }

    fn next_key(&mut self) -> usize {
        let key = self.next_key;
        self.next_key += 1;
        key
    }

    /// Insert an item, creating a new unique key, and return the key
    ///
    /// # Examples
    ///
    /// ```
    /// use indexed_map::IndexedMap;
    ///
    /// // create an empty `IndexedMap`, just like `HashMap`
    /// let mut fruits = IndexedMap::new();
    ///
    /// // insert some values and store their keys for later use
    /// let apple = fruits.insert("Apple");
    /// let orange = fruits.insert("Orange");
    /// let pear = fruits.insert("Pear");
    ///
    /// // list the values we've inserted
    /// for fruit in fruits.inner().values() {
    ///     println!("{}", fruit);
    /// }
    ///
    /// // access using unique keys
    /// assert_eq!("Apple", *fruits.inner().get(&apple).unwrap());
    /// assert_eq!("Orange", *fruits.inner().get(&orange).unwrap());
    /// assert_eq!("Pear", *fruits.inner().get(&pear).unwrap());
    pub fn insert(&mut self, value: T) -> usize {
        let key = self.next_key();
        self.items.insert(key, value);
        key
    }

    /// Access the underlying `HashMap`
    pub fn inner(&self) -> &HashMap<usize, T> {
        &self.items
    }

    /// Mutably access the underlying `HashMap`
    pub fn inner_mut(&mut self) -> &HashMap<usize, T> {
        &mut self.items
    }

    /// Unwrap the `IndexedMap` to return the underlying `HashMap`
    pub fn into_inner(self) -> HashMap<usize, T> {
        self.items
    }
}
