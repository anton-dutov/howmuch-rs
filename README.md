# howmuch

Measure elapsed time by tagged sections

Example

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


Output

    0.000000 |    0.000000 | BEGIN
    1.000100 |    1.000099 | SLEEP 1s
    3.000249 |    2.000148 | SLEEP 2s
    6.000419 |    3.000170 | SLEEP 3s
    6.000426 |    0.000005 | END
