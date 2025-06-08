use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct LoggingConfiguration {
    pub level: LogLevelConfiguration,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct LogLevelConfiguration {
    pub root: Option<String>,
    pub directives: Vec<LoggingDirective>,
}

#[derive(Debug, Deserialize , Clone)]
#[allow(unused)]
pub struct LoggingDirective {
    pub namespace: String,
    pub level: String,
}


#[derive(Debug, Deserialize , Clone)]
#[allow(unused)]
pub struct ServerConfiguration {
    pub port: u16,
}

#[derive(Debug, Deserialize , Clone)]
#[allow(unused)]
pub struct RabbitMQConfiguration {
    pub ampq_addr: String,
}


//Kakfa Structs



#[derive(Debug, Deserialize , Clone)]
#[allow(unused)]
pub struct BrokerProperties {
    pub urls: String,
}

#[derive(Debug, Deserialize , Clone)]
#[allow(unused)]
pub struct ConsumerConfiguration {
    pub id: String,
    pub topic: Vec<String>,
    pub client_id: String,
    pub group_id: String,
}

#[derive(Debug, Deserialize , Clone)]
#[allow(unused)]
pub struct SchemaRegistryProperties {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize )]
#[allow(unused)]
pub struct TopicConfiguration {
    pub mappings: Vec<TopicProperties>,
}

#[derive(Clone, Debug, Deserialize )]
#[allow(unused)]
pub struct TopicProperties {
    pub id: String,
    pub topic_name: String,
    pub partitions: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct KafkaConfiguration {
    pub broker: BrokerProperties,
    pub schema_registry: SchemaRegistryProperties,
    pub producer: ProducerProperties,
    pub topic: TopicConfiguration,
}

#[derive(Debug, Deserialize , Clone)]
#[allow(unused)]
pub struct ProducerProperties {
    pub client_id: String,
    pub transactional_id: String,
}