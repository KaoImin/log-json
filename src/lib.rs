//!

use std::time::{SystemTime, UNIX_EPOCH};

use creep::{Cloneable, Context};
use log::trace;
use serde::Serialize;
use serde_json::{json, Value};

/// Log the context message and a json value in json format at the trace level.
pub fn log_json<V>(ctx: Option<Context>, key: Option<&str>, json_value: Value)
where
    V: Cloneable + Serialize + 'static,
{
    #[cfg(feature = "off")]
    return;

    if let Some(cx) = ctx {
        let key = key.unwrap();
        let output = if let Some(value) = cx.get::<V>(key) {
            json!({key: value.to_owned(), "info": json_value, "timestamp": sys_now()})
        } else {
            json!({"info": json_value, "timestamp": sys_now()})
        };

        trace!("{:?}", output);
    } else {
        let output = json!({"info": json_value, "timestamp": sys_now()});
        trace!("{:?}", output);
    }
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

        let key = "name";
        let ctx = Context::new().with_value::<String>(key, "creep".to_owned());
        let log = json!({"epoch_id": 0});
        log_json::<String>(Some(ctx), Some(key), log);
    }
}
