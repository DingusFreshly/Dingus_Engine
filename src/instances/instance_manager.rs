use slotmap::{SlotMap, new_key_type};
use crate::instances::instance_descriptors::{Instance, Renderable};

new_key_type! { pub struct InstanceKey; }

pub struct InstanceManager {
    instances: SlotMap<InstanceKey, Box<dyn Instance>>,
}

impl InstanceManager {
    pub fn new() -> Self {
        Self { instances: SlotMap::with_key() }
    }
    pub fn add<I: Instance + 'static>(&mut self, instance: I) -> InstanceKey {
        
        let key = self.instances.insert(Box::new(instance));
        
        key
    }

    pub fn get(&self, key: InstanceKey) -> Option<&Box<dyn Instance>> {
        self.instances.get(key)
    }

    pub fn get_mut(&mut self, key: InstanceKey) -> Option<&mut Box<dyn Instance>> {
        self.instances.get_mut(key)
    }

}

