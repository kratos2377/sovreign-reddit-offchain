use rdkafka::producer::FutureProducer;

#[derive(Clone)]
pub struct AppDBState {
    pub producer: FutureProducer,
}