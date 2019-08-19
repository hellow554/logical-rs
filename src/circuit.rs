use crate::Updateable;

#[derive(Default)]
pub struct Circuit {
    updater: Vec<Box<dyn Updateable>>,
}

impl Circuit {
    pub fn tick(&mut self) -> bool {
        self.updater.iter_mut().fold(false, |acc, u| acc | u.update())
    }

    pub fn add_updater<T: Updateable + Clone + 'static>(&mut self, updater: &T) {
        self.updater.push(Box::new(updater.clone()))
    }
}
