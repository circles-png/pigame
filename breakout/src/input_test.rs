use std::{thread::sleep, time::Duration};

use pigame::input::{is_active, Input};

fn main() {
    loop {
        println!(
            "{:?}",
            Input::ALL
                .iter()
                .map(|pin| { is_active(*pin) })
                .collect::<Vec<_>>()
        );
        sleep(Duration::from_millis(100));
    }
}
