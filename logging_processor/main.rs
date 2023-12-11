use std::{collections::HashMap, sync::Arc, time::Duration};

use rdkafka::{
    config::FromClientConfig,
    producer::{BaseRecord, FutureProducer, FutureRecord},
    ClientConfig,
};
use tokio::io::{stdin, AsyncBufRead, AsyncBufReadExt, AsyncRead, BufReader, Lines};

async fn handle_line(
    line: &String,
    topic: &str,
    sink: Arc<FutureProducer>,
) -> Result<(), anyhow::Error> {
    let record: FutureRecord<'_, str, String> = FutureRecord {
        topic,
        payload: Some(line),
        key: None,
        partition: None,
        timestamp: None,
        headers: None,
    };

    sink.send(record, Duration::from_millis(1000))
        .await
        .map_err(|(e, _)| e)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let raw_env_vars: HashMap<String, String> = std::env::vars().collect();

    let sink_topic = match raw_env_vars.get("KAFKA_TOPIC") {
        Some(topic) => topic.clone(),
        None => String::from("log_sink"),
    };

    let mut stdin = BufReader::new(stdin()).lines();

    let mut config = ClientConfig::new();
    
    match raw_env_vars.get("KAFKA_BOOTSTRAP_SERVERS") {
        Some(bootstrap_servers) => {
            config.set("bootstrap.servers", bootstrap_servers);
        }
        None => panic!("KAFKA_BOOTSTRAP_SERVERS is not set"),
    }

    let sink = Arc::new(FutureProducer::from_config(&config)?);

    println!("Listening for lines from stdin...");

    while let Some(line) = stdin.next_line().await? {
        //println!("Sending line: {line}");
        let sink_ref = sink.clone();
        let sink_topic = sink_topic.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_line(&line, &sink_topic, sink_ref).await {
                println!("Unable to send {e}")
            }
        });
    }

    Ok(())
}
