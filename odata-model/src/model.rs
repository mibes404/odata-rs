use odata_edm::edm::Edmx;
use std::collections::HashMap;

use crate::resource::ODataResource;

pub struct ODataModel {
    resources: HashMap<String, ODataResource>,
    edm: Edmx,
}

impl ODataModel {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            edm: Edmx::default(),
        }
    }

    pub fn add_resource(&mut self, resource: ODataResource) {
        self.resources.insert(resource.entity.name.clone(), resource);
    }

    pub fn get_resource(&self, name: &str) -> Option<&ODataResource> {
        self.resources.get(name)
    }

    pub fn find_resource(&self, reference: &ODataResource) -> Option<&ODataResource> {
        self.resources.get(&reference.entity.name)
    }
}

impl Default for ODataModel {
    fn default() -> Self {
        Self::new()
    }
}
