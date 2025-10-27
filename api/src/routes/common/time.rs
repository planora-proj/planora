use chrono::Utc;

pub fn current_utc_timestamp() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
