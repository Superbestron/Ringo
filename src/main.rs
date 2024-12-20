// #![feature(core_intrinsics)]

use awc::{BoxedSocket, ClientResponse};
use std::{io, thread};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use actix_codec::Framed;
use actix_web::web::{Bytes, BytesMut};
use awc::error::WsClientError;
use awc::ws;
use awc::ws::{Codec, Frame, Message, WebsocketsRequest};
use actix_http::ws::Item;
use bytestring::ByteString;
use futures_util::{SinkExt as _, StreamExt as _};
use tokio::{sync::mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;

mod agrona;
pub mod bit_util;

async fn read_integer(ws: &mut Framed<BoxedSocket, Codec>) -> Option<i32> {
    if let Some(Ok(Frame::Text(bytes))) = ws.next().await {
        let text = String::from_utf8_lossy(&bytes).to_string();
        text.parse::<i32>().ok()
    } else {
        None
    }
}

async fn connect(url: &str) -> (ClientResponse, Framed<BoxedSocket, Codec>) {
    let ws = awc::Client::new()
        .ws(url);
    let ret = ws.max_frame_size(16 * 1024 * 1024)
        .connect()
        .await.unwrap();
    log::info!("Connected to {url}");
    ret
}

async fn disconnect(ws :&mut Framed<BoxedSocket, Codec>, url: &str) {
    ws.close().await.unwrap();
    log::info!("Disconnected from {url}");
}

async fn try_close(ws :&mut Framed<BoxedSocket, Codec>, url: &str) {
    match ws.send(Message::Close(None)).await {
        Ok(_) => {
            // log::info!("Closing server");
            disconnect(ws, &url).await;
        }
        Err(_) => {
            // log::info!("Error Closing server");
        }
    }
}

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let case_count_url = "ws://127.0.0.1:9001/getCaseCount";
    let (res, mut ws) = connect(case_count_url).await;

    let count = read_integer(&mut ws).await.unwrap();
    log::info!("Starting test suite - count:[{}]", count);

    try_close(&mut ws, case_count_url).await;

    for i in 0..count {
        let mut buffer: Vec<u8> = Vec::new(); // Can be optimised into raw buffers
        let url = format!("ws://127.0.0.1:9001/runCase?case={}&agent=actix", i + 1);

        let (res, mut ws) = connect(&url).await;
        let mut is_text = true;
        let mut is_continuous = false;
        loop {
            tokio::select! {
                Some(msg) = ws.next() => {
                    match msg {
                        Ok(Frame::Text(txt)) => {
                            match std::str::from_utf8(&txt) {
                                Ok(valid) => {
                                    let text = String::from_utf8_lossy(&txt).to_string();
                                    // log::info!("Received text from server");
                                    if is_continuous {
                                        is_continuous = false;
                                        try_close(&mut ws, &url).await;
                                        break;
                                    }
                                    ws.send(Message::Text(ByteString::from(text))).await.unwrap();
                                },
                                Err(_) => {
                                    try_close(&mut ws, &url).await;
                                    break;
                                }
                            }
                        }

                        Ok(Frame::Binary(bytes)) => {
                            // log::info!("Received bytes from server");
                            if is_continuous {
                                is_continuous = false;
                                try_close(&mut ws, &url).await;
                                break;
                            }
                            ws.send(Message::Binary(bytes)).await.unwrap();
                        }
                        Ok(Frame::Ping(bytes)) => {
                            // log::info!("Received ping from server");
                            ws.send(Message::Pong(bytes)).await.unwrap();
                        }
                        Ok(Frame::Pong(_)) => {
                            // log::info!("Received pong from_server");
                        }
                        Ok(Frame::Close(bytes)) => {
                            // log::info!("Received close from server");
                            try_close(&mut ws, &url).await;
                            break;
                        }
                        Ok(Frame::Continuation(mut bytes)) => {
                            match &mut bytes {
                                Item::FirstText(data) => {
                                    buffer.extend_from_slice(data);
                                    // log::info!("Received First Text from server");
                                    is_text = true;
                                    is_continuous = true;
                                }
                                Item::FirstBinary(data) => {
                                    buffer.extend_from_slice(data);
                                    // log::info!("Received First Binary from server");
                                    is_text = false;
                                    is_continuous = true;
                                }
                                Item::Continue(data) => {
                                    // log::info!("Received Continue from server");
                                    buffer.extend_from_slice(data);
                                },
                                Item::Last(data) => {
                                    if is_text {
                                        buffer.extend_from_slice(data);
                                        // log::info!("Received Last Text from server");
                                        let tmp = ByteString::try_from(buffer);
                                        match tmp {
                                            Ok(tmp) => {
                                                ws.send(Message::Text(tmp)).await.unwrap();
                                            },
                                            Err(_) => {
                                                try_close(&mut ws, &url).await;
                                            }
                                        }

                                    } else {
                                        buffer.extend_from_slice(data);
                                        // log::info!("Received Last Binary from server");
                                        ws.send(Message::Binary(actix_web::web::Bytes::from(buffer))).await.unwrap();
                                    }
                                    is_continuous = false;
                                    buffer = Vec::new();
                                }
                            }

                        },
                        Err(err) => {
                            // log::info!("Received error from server: {:?}", err);
                            try_close(&mut ws, &url).await;
                            break;
                        }
                    }
                }
            }
        }
        log::info!("Test completed - progress:[{}/{}]", i + 1, count);
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    let update_report_url = "ws://127.0.0.1:9001/updateReports?agent=actix";
    connect(update_report_url).await;
    log::info!("Reports updated, test suite is done!");
}
