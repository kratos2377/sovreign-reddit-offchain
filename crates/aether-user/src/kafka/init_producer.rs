use axum::Error;
use rdkafka::producer::FutureProducer;
use rdkafka::producer::Producer;
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use std::time::Duration;

use crate::conf::config_types::KafkaConfiguration;

pub fn create_new_kafka_producer(config: &KafkaConfiguration) -> Result<FutureProducer, Error> {
    // Start using configs
    let nan_id_gen = nano_id::base64::<15>();
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", config.broker.urls.clone())
        .set("request.timeout.ms", "5000") // Maximum amount of time the client will wait for the response of a reques
        .set("delivery.timeout.ms", "5000") // Upper bound on the time to report success or failure after a call to send() returns
        .set("enable.idempotence", "true") // Ensure that exactly one copy of each message is written in the stream
        // Number of unacknowledged requests the client will send on a single connection before
        // blocking
        .set("max.in.flight.requests.per.connection", "5")
        // Period of time in milliseconds after which we force a refresh of metadata even if we
        // haven't seen any partition leadership changes
        .set("metadata.max.age.ms", "10000")
        .set("linger.ms", "1000") // Wait 10ms to group sending messages
        .set("transactional.id", config.producer.transactional_id.clone() + "-" + nan_id_gen.clone().as_str())
        .set("queue.buffering.max.ms", "100") // Buffer messages 100ms
        .set("request.required.acks", "all") // Wait for acknowledge from broker
        .set("message.send.max.retries", "3") // Default
        .set("client.id", config.producer.client_id.clone() +"-" + nan_id_gen.clone().as_str()) // Set an identifiable name for traceability
        .create().unwrap();

    producer.init_transactions(Timeout::from(Duration::from_secs(10))).unwrap();

    Ok(producer)
}