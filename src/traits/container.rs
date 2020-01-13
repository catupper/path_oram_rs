pub trait Container<T> {
    fn new(size: usize) -> Self;
    fn get(&self, key: usize) -> Option<&T>;
    fn get_mut(&mut self, key: usize) -> Option<&mut T>;
    fn put(&mut self, key: usize, value: T);
}

impl<T> Container<T> for Vec<T>
where
    T: Default + Clone,
{
    fn new(size: usize) -> Self {
        vec![T::default(); size]
    }

    fn get(&self, key: usize) -> Option<&T> {
        if self.len() < key {
            None
        } else {
            Some(&self[key])
        }
    }

    fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        if self.len() < key {
            None
        } else {
            Some(&mut self[key])
        }
    }

    fn put(&mut self, key: usize, value: T) {
        self[key] = value;
    }
}
