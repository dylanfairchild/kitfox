use kitfoxm::actions::{
    ActionPayload, IdentifierArgs, IdentifierPayload, RequestPayload, Resource, ResourceIdentifier,
};
use kitfoxr;
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
        // TODO: Should request/action/actions be locked? So that only one is going to a resource
        //       at a time? Or should an individual resource be locked?
        if let Some(rsc) = self.resources.get(identifier) {
            return rsc.action(payload);
        }
        Default::default()
    }

    pub fn scan(&mut self) -> Vec<ResourceIdentifier> {
        //TODO: Need lock on this because self.resources should not be modified from 2 threads at once?
        //      Maybe need to copy out our resource to run whatever needs to be run on it so scans can
        //      happen while things are outstanding.
        //
        // Box should already reference count, right? So I don't know if we need to copy it.
        // Other methods don't mutate resources, only read from it... maybe it is safe?

        let mut new_resources: HashMap<ResourceIdentifier, Box<dyn Resource + Send + Sync>> =
            HashMap::new();

        let lnr = kitfoxr::linux::nvme::scan();

        for dev in lnr {
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
