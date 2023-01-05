use kitfoxm::actions::{Identify, IdentifyArgs, IdentifyResult};

pub fn find_in_tree_nvme_devices() -> Vec<InTreeNvmeDevice> {
    vec![InTreeNvmeDevice {
        path: String::from("/dev/nvme0n1"),
    }]
}

#[derive(Debug)]
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
