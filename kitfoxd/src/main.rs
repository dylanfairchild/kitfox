use kitfoxd::resource::ResourceManager;
use kitfoxm::actions::RequestPayload;
use serde_json;
use zbus::{dbus_interface, ConnectionBuilder, Result};

struct ResourceService {
    manager: ResourceManager,
}

#[dbus_interface(name = "org.fairchildtech.KitfoxResourceService")]
impl ResourceService {
    async fn scan(&mut self) -> String {
        serde_json::to_string(&self.manager.scan()).unwrap()
    }

    async fn request(&self, payload: &str) -> String {
        if let Ok(req) = serde_json::from_str::<RequestPayload>(payload) {
            let result = self.manager.request(&req);
            serde_json::to_string(&result).unwrap()
        } else {
            let result = RequestPayload {
                payload: Vec::new(),
            };
            serde_json::to_string(&result).unwrap()
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let resource_service = ResourceService {
        manager: ResourceManager::new(),
    };

    let _ = ConnectionBuilder::session()?
        .name("org.fairchildtech.Kitfox")?
        .serve_at("/org/fairchildtech/Kitfox", resource_service)?
        .build()
        .await?;

    loop {}
}
