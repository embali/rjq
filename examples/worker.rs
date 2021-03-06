extern crate rjq;

use std::time::Duration;
use std::thread::sleep;
use std::error::Error;
use rjq::Queue;

fn main() {
    fn process(uuid: String, _: Vec<String>) -> Result<String, Box<Error>> {
        sleep(Duration::from_millis(1000));
        println!("{}", uuid);
        Ok(format!("hi from {}", uuid))
    }

    let queue = Queue::new("redis://localhost/", "rjq");
    queue.work(process, None, Some(5), Some(10), None, Some(false), None).unwrap();
}
