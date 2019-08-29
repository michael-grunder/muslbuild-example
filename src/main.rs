use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Pass bootstrap host and topic name");
        std::process::exit(1);
    }

    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", &args[1])
        .create()
        .expect("Can't create consumer");

    let timeout = std::time::Duration::from_secs(1);
    let metadata = consumer
        .fetch_metadata(Some(&args[2]), timeout)
        .expect("Can't fetch metadata");

    for broker in metadata.brokers() {
        println!(
            "ID: {}; INFO: {}:{}",
            broker.id(),
            broker.host(),
            broker.port()
        );
    }
}
