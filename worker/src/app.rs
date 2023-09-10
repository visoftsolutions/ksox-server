use axum::{routing, Router};

pub fn get_app() -> Router {
    Router::new()
        .route("/", routing::get(http::root))
        .route("/sse", routing::get(sse::root))
        .route("/ws", routing::get(ws::root))
}

mod http {
    use chrono::Utc;

    pub async fn root() -> String {
        format!("Hello from server! Time: {}\n", Utc::now())
    }
}

mod sse {
    use std::{convert::Infallible, time::Duration};

    use axum::response::{sse::Event, Sse};
    use chrono::Utc;
    use futures::stream;
    use tokio_stream::{Stream, StreamExt};

    pub async fn root() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
        let stream = stream::repeat_with(|| {
            Event::default().data(format!("Hello from server! Time: {}\n", Utc::now()))
        })
        .map(Ok)
        .throttle(Duration::from_secs(1));

        Sse::new(stream).keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(25))
                .text("keep-alive-text"),
        )
    }
}

mod ws {
    use axum::{
        extract::{ws::Message, WebSocketUpgrade},
        response::Response,
    };
    use chrono::Utc;
    use futures::{Sink, SinkExt, Stream, StreamExt};

    pub async fn root(ws: WebSocketUpgrade) -> Response {
        ws.on_upgrade(|socket| {
            let (write, read) = socket.split();
            handler(write, read)
        })
    }

    async fn handler<W, R>(mut write: W, mut read: R)
    where
        W: Sink<Message> + Unpin,
        R: Stream<Item = Result<Message, axum::Error>> + Unpin,
    {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(msg) = msg {
                if write
                    .send(Message::Text(format!(
                        "Hello from server! Time: {}\n You said: {msg}\n",
                        Utc::now()
                    )))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }
    }
}
