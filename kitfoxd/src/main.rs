use kitfoxd::resource::ResourceManager;
use kitfoxm::actions::{ActionPayload, RequestPayload};
use serde_json;
use zbus::{dbus_interface, Connection, ConnectionBuilder, MessageHeader, Result};
use zbus_polkit::policykit1::{AuthorityProxy, CheckAuthorizationFlags, Subject};

struct ResourceService {
    manager: ResourceManager,
}

async fn check_authorization(hdr: &MessageHeader<'_>, payload: &RequestPayload) -> bool {
    // Scan for actions which require authorization.
    let mut needs_verification = false;
    for idp in &payload.payload {
        for ap in &idp.payload {
            match ap {
                ActionPayload::EraseArgs(_) | ActionPayload::EraseResumeArgs(_) => {
                    needs_verification = true;
                }
                _ => {}
            }
        }
    }
    if !needs_verification {
        return true;
    }

    // Authorize
    let connection = Connection::system().await.unwrap();
    let proxy = AuthorityProxy::new(&connection).await.unwrap();
    let subject = Subject::new_for_message_header(&hdr).unwrap();
    let result = proxy
        .check_authorization(
            &subject,
            "org.fairchildtech.Kitfox.KitfoxResourceService",
            &std::collections::HashMap::new(), // Check out known keys to customize message https://polkit.pages.freedesktop.org/polkit/eggdbus-interface-org.freedesktop.PolicyKit1.Authority.html#eggdbus-method-org.freedesktop.PolicyKit1.Authority.CheckAuthorization
            CheckAuthorizationFlags::AllowUserInteraction.into(),
            "",
        )
        .await
        .unwrap();
    println!("KITFOXD -- AuthorizationResult: {:?}", result);

    result.is_authorized
}

#[dbus_interface(name = "org.fairchildtech.KitfoxResourceService")]
impl ResourceService {
    async fn scan(&mut self) -> String {
        println!("Received scan");
        serde_json::to_string(&self.manager.scan()).unwrap()
    }

    async fn request(&self, #[zbus(header)] hdr: MessageHeader<'_>, payload: &str) -> String {
        println!("Received request: {}", payload);
        // serde_json::from_str::<RequestPayload>(payload).unwrap(); // for debugging
        if let Ok(req) = serde_json::from_str::<RequestPayload>(payload) {
            // Request authorization for privileged actions if present in the request.
            if !check_authorization(&hdr, &req).await {
                let result = RequestPayload {
                    payload: Vec::new(),
                };
                return serde_json::to_string(&result).unwrap();
            }

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

    let _ = ConnectionBuilder::system()?
        .name("org.fairchildtech.Kitfox")?
        .serve_at("/org/fairchildtech/Kitfox", resource_service)?
        .build()
        .await?;

    println!("Starting up the kitfoxd service");
    loop {}
}
