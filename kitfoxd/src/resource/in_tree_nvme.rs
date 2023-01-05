use kitfoxm::actions::{Identify, IdentifyArgs, IdentifyResult, Resource};
use kitfoxm_derive::Resource;

pub fn find_in_tree_nvme_devices() -> Vec<InTreeNvmeDevice> {
    vec![InTreeNvmeDevice {
        path: String::from("/dev/nvme0n1"),
    }]
}

//TODO: Actually reference some real structure in kitfoxi which holds data like
//      path etc. probably. It will also implement the actual commands & buffer parsing etc.
#[derive(Resource, Debug)]
pub struct InTreeNvmeDevice {
    pub path: String,
}

impl Identify for InTreeNvmeDevice {
    fn identify(&self, _args: &IdentifyArgs) -> IdentifyResult {
        IdentifyResult {
            name: Some(Ok(self.path.clone())),
            ..Default::default()
        }
    }
}
