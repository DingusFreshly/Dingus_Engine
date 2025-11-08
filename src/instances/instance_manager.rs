use slotmap::{SlotMap, new_key_type};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::instances::instance_descriptors::{Instance, InstanceFlags, Physics, Renderable, InstanceNode};
//use crate::instances::instance_descriptors::InstanceFlags;
new_key_type! { pub struct InstanceKey; }
/*TODO:
1: Create Layer struct to hold multiple layers of instances
 */
type Rc_Instance = Rc<RefCell<dyn Instance>>;

pub struct InstanceManager {
    pub instances: SlotMap<InstanceKey, Rc_Instance>,
    
    //nodes stores the hierarchy data of the instances
    //pub nodes: HashMap<InstanceKey, InstanceNode>,
    //this si for easy lookup by name
    pub name_lookup : HashMap<String, Vec<InstanceKey>>,
    //i could eventually store this in a struct called Layer for multiple layers if i want a true hierarchy
}

/*
Todo:
1: Make this readable please add comments
2: Add headers to each section to maek finding stuff easier
 */

impl InstanceManager {
    pub fn new() -> Self {
        Self {
            instances: SlotMap::with_key(),
            //nodes: HashMap::new(),
            name_lookup: HashMap::new()       
        }
    }
    //adds an instance and returns its wrapper
    pub fn add<I: Instance + 'static>(
        &mut self, instance: I, 
        name: Option<String>, 
        parent: Option<InstanceKey>) -> InstanceKey 
    {
        //let flags = instance.get_flags();
        let default_name = instance.class_name().to_string();//this is the default class name
        let name = name.unwrap_or(default_name);
        
        let rc_wrapped: Rc_Instance = Rc::new(RefCell::new(instance));

        let key = self.instances.insert(rc_wrapped);
        
        let wrapper = InstanceNode {
            name: name,
            key,
            parent: None,
            children: Vec::new(),
        };
        self.name_lookup.entry(wrapper.name.clone())//push to name lookup for easy finding
            .or_insert(Vec::new())//create new vec if empty
            .push(key);

        //self.nodes.insert(key, wrapper);
        
        /*if they ovverride parent, set it, but
        not implementing parents now   
        if let Some(parent_key) = parent {
            if let Some(parent_node) = self.nodes.get_mut(&parent_key) {
                parent_node.children.push(key);
            }
        }
        */
        
        key
    }

    pub fn remove_by_key(&mut self, key: InstanceKey) {
        self.instances.remove(key);
        //self.nodes.remove(&key);
        self.name_lookup.iter_mut().for_each(|(_, keys)| {
            keys.retain(|&k| k != key);
        });
    }
    
    pub fn get_instance_by_name(&self, name: &str) -> Rc_Instance {
        //this is a very error prone implementation, it will be very strict for now
        //*eh-hem* unlike lua
        let keys = self.name_lookup.get(name).unwrap();
        let first_key = keys[0];
        self.instances.get(first_key).cloned().unwrap()
    }
    pub fn get_all_with_name(&self, name: &str) -> Vec<Rc_Instance> {
        let mut result = Vec::new();
        if let Some(keys) = self.name_lookup.get(name) {
            for key in keys {
                if let Some(instance) = self.instances.get(*key) {
                    result.push(instance.clone());
                }
            }
        }
        result
    }
    pub fn get_by_key(&self, key: InstanceKey) -> Option<Rc_Instance> {
        self.instances.get(key).cloned()
    }
}
/*
pub struct Layer {
    //nodes stores the hierarchy data of the instances
    pub nodes: HashMap<InstanceKey, InstanceNode>,
    //this si for easy lookup by name
    pub name_lookup : HashMap<String, Vec<InstanceKey>>,
}
impl Layer {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            name_lookup: HashMap::new(),
        }
    }
    pub fn find_first_by_name(&self, name: &str) -> Option<InstanceKey> {
        if let Some(keys) = self.name_lookup.get(name) {
            if !keys.is_empty() {
                return Some(keys[0]);
            }
        }
        None
    }
}
*/