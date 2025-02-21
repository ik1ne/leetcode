use std::collections::HashMap;

struct MapSum {
    inner: HashMap<String, i32>,
    prefix: HashMap<char, (i32, Box<MapSum>)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    fn new() -> Self {
        MapSum {
            inner: HashMap::new(),
            prefix: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        if let Some(v) = self.inner.get_mut(&key) {
            let diff = val - *v;
            *v = val;

            self.update_prefix(&key, diff);
        } else {
            self.update_prefix(&key, val);
            self.inner.insert(key, val);
        }
    }

    fn update_prefix(&mut self, key: &str, diff: i32) {
        let mut current = &mut self.prefix;
        for c in key.chars() {
            let (sum, next) = current
                .entry(c)
                .or_insert_with(|| (0, Box::new(MapSum::new())));
            *sum += diff;
            current = &mut next.prefix;
        }
    }

    fn sum(&self, prefix: String) -> i32 {
        let mut result = 0;
        let mut current = &self.prefix;
        for c in prefix.chars() {
            if let Some((i, next)) = current.get(&c) {
                result = *i;
                current = &next.prefix;
            } else {
                return 0;
            }
        }

        result
    }
}
