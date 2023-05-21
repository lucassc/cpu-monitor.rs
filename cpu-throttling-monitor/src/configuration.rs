use std::{env, str::FromStr};

const THRESHOLD: &str = "CTM_THRESHOLD";
const NOTIFICATION_TITLE: &str = "CTM_NOTIFICATION_TITLE";
const NOTIFICATION_BODY: &str = "CTM_NOTIFICATION_BODY";
const NOTIFICATION_SECONDS_INTERVAL: &str = "CTM_NOTIFICATION_SECONDS_INTERVAL";

const THRESHOLD_DEFAULT: u64 = 400;
const NOTIFICATION_TITLE_DEFAULT: &str = "Warning";
const NOTIFICATION_BODY_DEFAULT: &str = "CPU being throttled";
const NOTIFICATION_SECONDS_INTERVAL_DEFAULT: u64 = 30;

pub struct Configuration {
    pub threshold: u64,
    pub notification_title: String,
    pub notification_body: String,
    pub notification_seconds_interval: u64,
}

impl ::std::default::Default for Configuration {
    fn default() -> Self {
        Self {
            threshold: get_env::<u64>(THRESHOLD, THRESHOLD_DEFAULT),
            notification_title: get_env::<String>(
                NOTIFICATION_TITLE,
                NOTIFICATION_TITLE_DEFAULT.to_string(),
            ),
            notification_body: get_env::<String>(
                NOTIFICATION_BODY,
                NOTIFICATION_BODY_DEFAULT.to_string(),
            ),
            notification_seconds_interval: get_env::<u64>(
                NOTIFICATION_SECONDS_INTERVAL,
                NOTIFICATION_SECONDS_INTERVAL_DEFAULT,
            ),
        }
    }
}

fn get_env<T: FromStr>(env_name: &str, default_value: T) -> T {
    match env::var(env_name) {
        Ok(value) => match value.parse::<T>() {
            Ok(parsed_value) => parsed_value,
            Err(_) => default_value,
        },
        Err(_) => {
            println!("{} not set, using default value", env_name);
            default_value
        }
    }
}
