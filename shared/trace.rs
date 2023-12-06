use rand::prelude::*;
use std::fmt::Write;

pub fn generate_trace_id() -> String {
    let mut rng = rand::thread_rng();
    let hex_string: String = (0..6)
        .map(|_| rng.gen_range(0..=15))
        .fold(String::new(), |mut output, b| {
            let _ = write!(output, "{:02X}", b);
            output
        });

    hex_string
}