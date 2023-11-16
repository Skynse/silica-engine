use std::cell::RefCell;

use crate::variant::*;

#[derive(Clone)]
pub struct VariantGroup {
    pub group_name: String,
    elements: Vec<Variant>,
}

impl VariantGroup {
    pub fn get_elements(&self) -> Vec<Variant> {
        self.elements.clone()
    }
}

pub struct ElementManager {
    pub groups: RefCell<Vec<VariantGroup>>,
}
impl ElementManager {
    pub fn new() -> Self {
        ElementManager {
            groups: RefCell::new(Vec::new()),
        }
    }

    pub fn register_group(&self, group_name: &str, elements: Vec<Variant>) {
        self.groups.borrow_mut().push(VariantGroup {
            group_name: group_name.to_string(),
            elements,
        });
    }

    pub fn get_group(&self, group_name: &str) -> Vec<Variant> {
        self.groups
            .borrow()
            .iter()
            .find(|group| group.group_name == group_name)
            .map_or_else(Vec::new, |group| group.elements.clone())
    }

    pub fn get_group_names(&self) -> Vec<String> {
        self.groups
            .borrow()
            .iter()
            .map(|group| group.group_name.clone())
            .collect()
    }
}
