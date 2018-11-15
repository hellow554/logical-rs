use crate::Updateable;

#[derive(Default)]
pub struct Reactor {
    updater: Vec<Box<Updateable>>,
}

impl Reactor {
    pub fn new() -> Self {
        Reactor { updater: vec![] }
    }

    pub fn tick(&mut self) {
        for u in &mut self.updater {
            u.update();
        }
    }

    pub fn add_updater<T: Updateable + Clone + 'static>(&mut self, updater: &T) {
        self.updater.push(Box::new(updater.clone()))
    }
}
