use std::any::Any;
use std::collections::HashMap;

pub struct Context {
    map: HashMap<String, Box<dyn Any>>
}

impl Context {
    pub fn new() -> Self {
        Context {
            map: HashMap::new()
        }
    }

    pub fn insert<T: 'static>(&mut self, key: impl AsRef<str>, val: T) {
        self.map.insert(key.as_ref().to_string(), Box::new(val));
    }

    pub fn get<T: 'static>(&mut self, key: impl AsRef<str>) -> Option<&T> {
        self.map.get(key.as_ref()).and_then(|boxed| (&**boxed as &(dyn Any + 'static)).downcast_ref())
    }

    pub fn get_mut<T: 'static>(&mut self, key: impl AsRef<str>) -> Option<&mut T> {
        self.map.get_mut(key.as_ref()).and_then(|boxed| (&mut **boxed as &mut (dyn Any + 'static)).downcast_mut())
    }
}

