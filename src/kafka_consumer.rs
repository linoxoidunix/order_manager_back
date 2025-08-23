use rdkafka::consumer::{Consumer, StreamConsumer};
use futures::StreamExt;
use rdkafka::message::Message;
use rdkafka::ClientConfig;
use tokio::sync::broadcast;

pub async fn run_kafka_consumer(
    brokers: &str,
    topic: &str,
    tx: broadcast::Sender<String>,
) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "order_ws_group")
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Consumer creation failed");
    consumer.subscribe(&[topic]).expect("Can't subscribe");

    let tx = tx.clone();
    tokio::spawn(async move {
        let mut stream = consumer.stream();
        while let Some(result) = stream.next().await {
            match result {
                Ok(msg) => {
                    if let Some(payload) = msg.payload() {
                        let data = String::from_utf8_lossy(payload).to_string();
                        let _ = tx.send(data);
                    }
                }
                Err(e) => eprintln!("Kafka error: {}", e),
            }
        }
    });
}
