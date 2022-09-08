mod datetime;

use std::time::SystemTime;

use crate::datetime::IntoDateTime;

fn main() {
    let duration_since_unix_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    println!("{}", duration_since_unix_epoch.into_datetime());
}
