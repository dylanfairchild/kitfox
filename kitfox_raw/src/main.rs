use kitfoxm::actions::{ActionPayload, IdentifierPayload, RequestPayload, ResourceIdentifier};
use serde_json;
use zbus::{dbus_proxy, Connection, Result};

#[dbus_proxy(
    interface = "org.fairchildtech.KitfoxResourceService",
    default_service = "org.fairchildtech.Kitfox",
    default_path = "/org/fairchildtech/Kitfox"
)]
trait ResourceService {
    async fn scan(&mut self) -> Result<String>;
    async fn request(&self, payload: &str) -> Result<String>;
}

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::system().await?;
    let mut proxy = ResourceServiceProxy::new(&connection).await?;
    let reply = proxy.scan().await?;

    println!("Scan Reply: {}", reply);
    let parsed: Vec<ResourceIdentifier> = serde_json::from_str(&reply).unwrap();

    let mut reqp = RequestPayload { payload: vec![] };
    for identifier in parsed {
        let idp = IdentifierPayload {
            identifier: identifier,
            payload: vec![
                ActionPayload::IdentifyArgs(Default::default()),
                ActionPayload::IdentifyArgs(Default::default()),
                ActionPayload::IdentifierArgs(Default::default()),
                ActionPayload::SupportedActionsArgs(Default::default()),
            ],
        };
        reqp.payload.push(idp);
    }
    let payload = serde_json::to_string(&reqp).unwrap();
    println!("Payload OUT: {}", payload);
    let future = proxy.request(&payload);
    let futures = vec![future];
    let replies = futures::future::join_all(futures).await;
    // for reply in replies {
    //     println!("Reply: {}", reply.unwrap());
    // }

    Ok(())
}
