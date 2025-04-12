use mockito::{Server, ServerOpts};
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};

use crate::consts::url::{MOCK_HOST, MOCK_PORT};

type SharedServer = Arc<Mutex<Server>>;

static SHARED_SERVER: OnceCell<SharedServer> = OnceCell::const_new();

#[derive(Clone)]
pub struct MockServer(pub SharedServer);

impl MockServer {
    pub async fn new_async() -> Self {
        let server = SHARED_SERVER
            .get_or_init(|| async {
                let opts = ServerOpts {
                    host: MOCK_HOST,
                    port: MOCK_PORT,
                    ..Default::default()
                };
                Arc::new(Mutex::new(Server::new_with_opts_async(opts).await))
            })
            .await;

        Self(Arc::clone(server))
    }
}
