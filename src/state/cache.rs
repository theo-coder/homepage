use std::time::SystemTime;

#[derive(Default, Debug, Clone)]
pub struct Cache {
    image: Option<String>,
    timestamp: Option<SystemTime>,
}

impl Cache {
    pub fn get(&self) -> Option<(&String, &SystemTime)> {
        self.image.as_ref().zip(self.timestamp.as_ref())
    }

    pub fn update(&mut self, image: String) {
        self.image = Some(image);
        self.timestamp = Some(SystemTime::now());
    }
}
