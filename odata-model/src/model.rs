use odata_edm::edm::{Edmx, EntityType};
use std::collections::HashMap;

use crate::resource::{Entity, ODataResource};

pub struct ODataModel {
    base_url: String,
    resources: HashMap<String, ODataResource>,
    edm: Edmx,
}

impl ODataModel {
    pub fn new<U: Into<String>>(base_url: U) -> Self {
        Self {
            base_url: base_url.into(),
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
        self.get_entity_type_by_name(name)
    }

    fn get_entity_type_by_name(&self, name: &str) -> Option<&EntityType> {
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

    pub fn context_for_entity(&self, entity_id: &str) -> Option<String> {
        let entity_type = self.get_entity_type_by_name(entity_id);
        let base_url = &self.base_url;

        entity_type.map(|entity_type| format!("{}/$metadata#{}", base_url, entity_type.name))
    }
}

impl Default for ODataModel {
    fn default() -> Self {
        Self::new("https://example.com/V4/SampleService")
    }
}

impl std::fmt::Debug for ODataModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ODataModel")
            .field("resources", &self.resources)
            .field("edm", &self.edm)
            .finish()
    }
}
