use chrono::Utc;
use std::sync::Mutex;

const EPOCH: i64 = 1704096000000; // Custom Epoch (2024-01-01T00:00:00Z)
const MACHINE_ID_BITS: i64 = 5;
const SEQUENCE_BITS: i64 = 12;

const MAX_MACHINE_ID: i64 = (1 << MACHINE_ID_BITS) - 1;
const MAX_SEQUENCE: i64 = (1 << SEQUENCE_BITS) - 1;

#[derive(Debug)]
pub struct SnowflakeGenerator {
    machine_id: i64,
    last_timestamp: Mutex<i64>,
    sequence: Mutex<i64>,
}

impl SnowflakeGenerator {
    pub fn new(machine_id: i64) -> Self {
        if machine_id < 0 || machine_id > MAX_MACHINE_ID {
            panic!("Machine ID must be between 0 and {}", MAX_MACHINE_ID);
        }
        SnowflakeGenerator {
            machine_id,
            last_timestamp: Mutex::new(0),
            sequence: Mutex::new(0),
        }
    }

    pub fn generate_id(&self) -> i64 {
        let mut last_timestamp = self.last_timestamp.lock().unwrap();
        let mut sequence = self.sequence.lock().unwrap();

        let mut timestamp = current_timestamp();

        if timestamp == *last_timestamp {
            *sequence = (*sequence + 1) & MAX_SEQUENCE;
            if *sequence == 0 {
                timestamp = til_next_millis(*last_timestamp);
            }
        } else {
            *sequence = 0;
        }

        *last_timestamp = timestamp;

        ((timestamp - EPOCH) << (MACHINE_ID_BITS + SEQUENCE_BITS)) |
            (self.machine_id << SEQUENCE_BITS) |
            *sequence
    }
}

fn current_timestamp() -> i64 {
    Utc::now().timestamp_millis()
}

fn til_next_millis(last_timestamp: i64) -> i64 {
    let mut timestamp = current_timestamp();
    while timestamp <= last_timestamp {
        timestamp = current_timestamp();
    }
    timestamp
}
