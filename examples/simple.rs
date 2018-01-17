extern crate howmuch;

use howmuch::HowMuch;

use std::time::Duration;
use std::thread;

fn main() {

    let mut hm = HowMuch::new();

    thread::sleep(Duration::new(1,0));

    hm.tag("SLEEP 1s");

    thread::sleep(Duration::new(2,0));

    hm.tag("SLEEP 2s");

    thread::sleep(Duration::new(3,0));

    hm.tag("SLEEP 3s");

}