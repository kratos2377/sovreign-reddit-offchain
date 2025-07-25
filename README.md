# Sovreign Reddit Offchain


Collection of crates which listen to events from Sovereign Rollup and does the necessary processing to serve Client APIs.

## This Repo Structure

| Codebase              |      Description          |
| :-------------------- | :-----------------------: |
| [DB-Layer](crates/db-layer)    |    DB Layer to save/fetch Models |
| [dark-matter](crates/dark-matter)            |      Entity Models        |
| [migration](crates/migration)|       Migration Logic     |
| [cosmic](crates/cosmic)        |   Common Models for WS and some kafka events |
| [aether-user](crates/aether-user)    |  Consume User Kafka Events and process           |
| [aether-subreddit](crates/aether-subreddit)        |  Consume Subreddit Kafka Events and process |
| [aether-post](crates/aether-post)        |  Consume Post Kafka Events and process |
| [Pulsar](crates/Pulsar)        |  Consumes Events from RabbitMQ and sends to respective topic after parsing |
| [comet](crates/comet)        |  Subscribe to WS url and publish RabbitMQ events |


## Design

![App Desing](./assets/diagram.png)