use kitfoxi::linux::in_tree_nvme as kitfoxi;
use kitfoxm::actions::{Identify, IdentifyArgs, IdentifyResult, Resource};
use kitfoxm_derive::Resource;

pub fn find_in_tree_nvme_devices() -> Vec<InTreeNvmeDevice> {
    let devs = kitfoxi::find_in_tree_nvme_devices();

    let mut res: Vec<InTreeNvmeDevice> = Vec::new();
    for dev in devs {
        res.push(InTreeNvmeDevice { dev: dev });
    }

    res
}

//TODO: Actually reference some real structure in kitfoxi which holds data like
//      path etc. probably. It will also implement the actual commands & buffer parsing etc.
#[derive(Resource, Debug)]
pub struct InTreeNvmeDevice {
    dev: kitfoxi::InTreeNvmeDevice,
}

impl Identify for InTreeNvmeDevice {
    fn identify(&self, _args: &IdentifyArgs) -> IdentifyResult {
        let mut res: IdentifyResult = Default::default();

        if let Some(s) = self.dev.path.to_str() {
            res.name = Some(Ok(String::from(s)));
        }

        res
    }
}
