use std::time::SystemTime;

fn main() {
    let duration_since_unix_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    println!("{}", duration_since_unix_epoch.as_secs_f64());
}
