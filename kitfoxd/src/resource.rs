pub mod in_tree_nvme;

use crate::resource::in_tree_nvme::find_in_tree_nvme_devices;
use kitfoxm::actions::{
    ActionPayload, IdentifierArgs, IdentifierPayload, RequestPayload, Resource, ResourceIdentifier,
};
use std::collections::HashMap;

pub struct ResourceManager {
    resources: HashMap<ResourceIdentifier, Box<dyn Resource + Send + Sync>>,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager {
            resources: HashMap::new(),
        }
    }

    pub fn request(&self, payload: &RequestPayload) -> RequestPayload {
        let mut result = RequestPayload {
            payload: Vec::new(),
        };

        for idp in &payload.payload {
            let mut identifier_results_payload = IdentifierPayload {
                identifier: idp.identifier.clone(),
                payload: Vec::new(),
            };

            if let Some(rsc) = self.resources.get(&idp.identifier) {
                for ap in &idp.payload {
                    let result_action_payload = rsc.action(&ap);

                    identifier_results_payload
                        .payload
                        .push(result_action_payload);
                }
            }

            result.payload.push(identifier_results_payload);
        }

        result
    }

    pub fn actions(
        &self,
        identifier: &ResourceIdentifier,
        payload: &Vec<ActionPayload>,
    ) -> Vec<ActionPayload> {
        let mut result: Vec<ActionPayload> = Vec::new();

        if let Some(rsc) = self.resources.get(identifier) {
            for action in payload {
                result.push(rsc.action(action));
            }
        }

        result
    }

    pub fn action(
        &self,
        identifier: &ResourceIdentifier,
        payload: &ActionPayload,
    ) -> ActionPayload {
        if let Some(rsc) = self.resources.get(identifier) {
            return rsc.action(payload);
        }
        Default::default()
    }

    pub fn scan(&mut self) -> Vec<ResourceIdentifier> {
        let mut new_resources: HashMap<ResourceIdentifier, Box<dyn Resource + Send + Sync>> =
            HashMap::new();

        let in_tree_nvme_devices = find_in_tree_nvme_devices();

        for dev in in_tree_nvme_devices {
            let args: IdentifierArgs = Default::default();
            let result = dev.send_identifier(&args);

            if let Some(i) = result.identifier {
                if let Ok(identifier) = i {
                    if let Some(rsc) = self.resources.remove(&identifier) {
                        new_resources.insert(identifier, rsc);
                    } else {
                        new_resources.insert(identifier, Box::new(dev));
                    }
                }
            }
        }

        self.resources = new_resources;

        let mut keys: Vec<ResourceIdentifier> = Vec::new();
        for key in self.resources.keys() {
            keys.push(key.clone());
        }

        keys
    }
}
