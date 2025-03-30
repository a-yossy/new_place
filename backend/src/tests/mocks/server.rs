use mockito::{Server, ServerOpts};

use crate::consts::url::{MOCK_HOST, MOCK_PORT};

pub struct MockServer(pub Server);

impl MockServer {
    pub async fn new_async() -> Self {
        let opts = ServerOpts {
            host: MOCK_HOST,
            ..Default::default()
        };
        let server = Server::new_with_opts_async(opts).await;

        Self(server)
    }
}
