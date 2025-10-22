use actix_web::{Error, HttpRequest, HttpResponse, rt, web};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;

pub async fn ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    if text == "ping" {
                        session.text("pong").await.unwrap();
                    } else {
                        session.text(text).await.unwrap();
                    }
                }

                _ => {
                    session
                        .text("For now. websocket only supports text format")
                        .await
                        .unwrap();
                }
            }
        }
    });

    Ok(res)
}
