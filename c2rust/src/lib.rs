#![warn(clippy::pedantic)]
#![deny(clippy::all)]

use rand::{thread_rng, Rng};

#[no_mangle]
pub extern "C" fn roll(count: u8, sides: u8) -> u16 {
    let mut rng = thread_rng();

    if count < 1 || sides < 2 {
        return 0;
    }

    (0..count)
        .try_fold(0, move |sum: u16, _| {
            let n = rng.gen_range(1..=sides);
            sum.checked_add(n.into())
        })
        .unwrap_or(0)
}
