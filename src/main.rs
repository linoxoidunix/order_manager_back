mod kafka_consumer;
mod ws_server;
mod config;

use tokio::sync::broadcast;
use crate::config::Config;

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel::<String>(1024);
    let config = Config::load();
    // Kafka
    kafka_consumer::run_kafka_consumer(&config.kafka.brokers, &config.kafka.topic, tx.clone()).await;

    // WebSocket
    ws_server::run_ws_server(&config.server.addr, tx.clone()).await;
}
