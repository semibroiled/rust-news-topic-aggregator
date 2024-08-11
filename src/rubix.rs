use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Counter<'a>{
    pub map: HashMap<&'a str, u64>,
}

impl<'a> Counter<'a> {

    // Constructor
    pub fn new() -> Self {

        Counter {
            map: HashMap::new(),
        }
    }

    // Implement Get
    pub fn get(&self, key: &str) -> Option<&u64> {
        self.map.get(key)
    }

    // Implement Incrementation
    pub fn increment(&mut self, key: &'a str) {
        let count = self.map.entry(key).or_insert(0); //Make Key and init value to 0 if doesn't exist
        *count += 1;
    }

    // Generic from method to accept any iterable type
    pub fn from<I>(items: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut counter = Counter::new();
        for item in items {
            counter.increment(item);
        }
        counter
    }

}