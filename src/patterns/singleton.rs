pub struct Singleton<T> {
    instance: Option<T>,
}

impl<T> Singleton<T> {
    pub fn new() -> Self {
        Self { instance: None }
    }

    pub fn get_or_init<F>(&mut self, init: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        if self.instance.is_none() {
            self.instance = Some(init());
        }
        self.instance.as_mut().unwrap()
    }
}
