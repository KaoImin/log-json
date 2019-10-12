//!
//! ```rust
//! use log_json::log_json;
//! use serde_json::json;
//!
//! fn main() {
//!     let info = json!({"ID": 0});
//!     log_json(None, info);  
//! }
//! ```

use std::time::{SystemTime, UNIX_EPOCH};

use log::trace;
use serde_json::{json, Value};

/// Log the context message and a json value in json format at the trace level.
pub fn log_json(ctx_value: Option<Value>, info_value: Value) {
    #[cfg(feature = "off")]
    return;

    let output = if let Some(ctx) = ctx_value {
        json!({"context": ctx, "info": info_value, "timestamp": sys_now()})
    } else {
        json!({"info": info_value, "timestamp": sys_now()})
    };
    trace!("{:?}", output);
}

fn sys_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod test {
    use super::*;
    use env_logger;

    #[test]
    fn test_log() {
        env_logger::init();

        let ctx = json!({"name": "creep".to_string()});
        let log = json!({"epoch_id": 0});
        log_json(Some(ctx), log);
    }
}
