use odata_edm::edm::{Edmx, EntityType};
use std::collections::HashMap;

use crate::resource::{Entity, ODataResource};

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

    pub fn get_entity_type(&self, reference: &ODataResource) -> Option<&EntityType> {
        let name = &reference.entity.name;
        if let Some(schema) = self.edm.data_services.schema.get(0) {
            if let Some(entity_type) = schema.entity_type.as_ref() {
                return entity_type.iter().find(|et| et.name == *name);
            }
        }

        None
    }

    pub fn with_entity_type(mut self, et: EntityType) -> Self {
        let resource = ODataResource {
            entity: Entity::from(&et),
            ..Default::default()
        };
        self.add_resource(resource);

        if let Some(schema) = self.edm.data_services.schema.get_mut(0) {
            if let Some(entity_type) = schema.entity_type.as_mut() {
                entity_type.push(et);
            } else {
                schema.entity_type = Some(vec![et]);
            }
        }

        self
    }

    pub fn edm(&self) -> &Edmx {
        &self.edm
    }
}

impl Default for ODataModel {
    fn default() -> Self {
        Self::new()
    }
}
