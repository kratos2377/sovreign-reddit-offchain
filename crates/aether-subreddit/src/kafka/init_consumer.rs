use std::collections::HashMap;

use axum::Error;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use rdkafka::ClientConfig;

use crate::conf::config_types::KafkaConfiguration;



pub fn init_consumers(
    config: &KafkaConfiguration,
) -> Result<HashMap<String, StreamConsumer>, Error> {
    let mut consumers = HashMap::new();

    for consumer_configuration in &config.consumer {
        // Initialize consumer
        let consumer = init_consumer(
            config.broker.urls.clone(),
            consumer_configuration.client_id.clone(),
            consumer_configuration.group_id.clone(),
            consumer_configuration.topic.clone(),
        ).unwrap();

        // Add consumer with id to the result map
        consumers.insert(consumer_configuration.id.clone(), consumer);
    }
    Ok(consumers)
}

fn init_consumer(
    bootstrap_servers: String,
    client_id: String,
    group_id: String,
    topics: Vec<String>,
) -> Result<StreamConsumer, Error> {
    // Initialize consumer
    let consumer: StreamConsumer = ClientConfig::new()
        .set("auto.offset.reset", "latest")
        .set("allow.auto.create.topics", "false")
        .set("bootstrap.servers", bootstrap_servers)
        .set("enable.auto.commit", "true")
        .set("enable.auto.offset.store", "false") // Don't update offsets in store automatically
        .set("auto.commit.interval.ms", "5000") // Default
        .set("isolation.level", "read_committed")
        .set("max.poll.interval.ms", "60000")
        .set("group.id", group_id)
        .set("client.id", client_id)
        .set("metadata.max.age.ms", "60000")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create().unwrap();

    // Convert topic names into &str
    let topics: Vec<&str> = topics.iter().map(|t| &**t).collect();

    // println!("THE TOPICS LIST IS: {:?}", topics);
    // Subscribe to the specified topic names
    consumer.subscribe(&topics).unwrap();

    Ok(consumer)
}