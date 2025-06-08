use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct LoggingConfiguration {
    pub level: LogLevelConfiguration,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct LogLevelConfiguration {
    pub root: Option<String>,
    pub directives: Vec<LoggingDirective>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct LoggingDirective {
    pub namespace: String,
    pub level: String,
}


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ServerConfiguration {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RabbitMQConfiguration {
    pub ampq_addr: String,
}