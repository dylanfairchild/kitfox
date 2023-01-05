// use std::{error::Error, fmt};

// #[derive(Debug)]
// pub struct NotImplementedError {
//     sig: String,
// }
// impl Error for NotImplementedError {}
// impl fmt::Display for NotImplementedError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Not Implemented: {}", self.sig)
//     }
// }

// pub trait Identifiable {
//     fn name(&self) -> String;
// }

// pub trait Eraseable {
//     fn erase(&self);
// }

// pub trait Resource {
//     fn identification(&self) -> Result<String, NotImplementedError> {
//         Err(NotImplementedError {
//             sig: String::from("identification"),
//         })
//     }

//     fn erase(&self) -> Result<String, NotImplementedError> {
//         Err(NotImplementedError {
//             sig: String::from("erase"),
//         })
//     }
// }

// struct InTreeNvmeDevice {}

// impl Resource for InTreeNvmeDevice {}

use std::collections::HashMap;

struct InTreeNvmeDevice {}

trait Capabilities {
    fn capabilities(&self) -> HashMap<String, HashMap<String, String>>;
}

impl Capabilities for InTreeNvmeDevice {
    fn capabilities(&self) -> HashMap<String, HashMap<String, String>> {
        let mut caps = HashMap::new();
        let mut identify = HashMap::new();
        identify.insert(String::from("details"), String::from("bool"));

        caps.insert(String::from("identify"), identify);

        caps
    }
}

#[derive(Default)]
struct Identification {
    name: Option<String>,
}

impl Identification {
    fn new() -> Self {
        Default::default()
    }
}

trait Identify {
    fn identify(&self) -> Identification;
}
