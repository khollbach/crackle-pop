#![no_std]
#![no_main]

mod digits;

use cortex_m_rt::entry;
use digits::{BLANK, CRACKLE, DIGITS, POP};
use embedded_hal::blocking::delay::DelayUs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_halt as _;

/// How long to display each `n` in 1..=100, in milliseconds.
const DISPLAY_MS: u32 = 1_000;

/// How long to display a blank screen between iterations, in milliseconds.
const PAUSE_MS: u32 = 300;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut display_ = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    for n in 1..=100 {
        display(&mut display_, &mut timer, n);

        display_.show(&mut timer, BLANK, PAUSE_MS);
    }

    panic!();
}

/// Display `n` (or crackle and/or pop) on the 5x5 led grid, for `DISPLAY_MS` milliseconds.
fn display(display: &mut Display, timer: &mut impl DelayUs<u32>, n: u32) {
    if n % 3 == 0 && n % 5 == 0 {
        display.show(timer, CRACKLE, DISPLAY_MS / 2);
        display.show(timer, POP, ceil_div(DISPLAY_MS, 2));
    } else if n % 3 == 0 {
        display.show(timer, CRACKLE, DISPLAY_MS);
    } else if n % 5 == 0 {
        display.show(timer, POP, DISPLAY_MS);
    } else {
        display_number(display, timer, n);
    }
}

/// `a` divided by `b`, rounded up.
fn ceil_div(a: u32, b: u32) -> u32 {
    let extra = if a % b == 0 { 0 } else { 1 };

    a / b + extra
}

/// Display a number `n` for `DISPLAY_MS` milliseconds.
///
/// For numbers with multiple digits, the timeslice is divided up evenly,
/// and each digit is shown for that fractional amount.
fn display_number(display: &mut Display, timer: &mut impl DelayUs<u32>, n: u32) {
    let mut prev_digit = None;
    let mut remaining_duration = DISPLAY_MS;

    // For example:
    //       v i=0
    // n=12345
    //   ^
    //   i=4
    let num_digits = num_digits(n);
    for i in (0..num_digits).rev() {
        let curr_digit = ith_digit(n, i);

        // The last timeslice might be slightly longer, to account for rounding errors.
        let mut duration = if i == 0 {
            remaining_duration
        } else {
            DISPLAY_MS / num_digits
        };
        remaining_duration -= duration;

        // If this digit is the same as the previous one, show a blank screen briefly
        // to "separate" the two.
        if Some(curr_digit) == prev_digit {
            let pause_duration = duration / 4;
            duration -= pause_duration;

            display.show(timer, BLANK, pause_duration);
        }

        display.show(timer, DIGITS[curr_digit as usize], duration);
        prev_digit = Some(curr_digit);
    }
}

/// How many digits in the base-10 representation of `n`?
fn num_digits(mut n: u32) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut count = 0;

    while n != 0 {
        count += 1;

        n /= 10;
    }

    count
}

/// Return the ith digit of n.
///
/// `i` is zero-indexed, from least-significant to most-significant.
///
/// ```
/// assert_eq!(ith_digit(456, 0), 6);
/// assert_eq!(ith_digit(456, 1), 5);
/// assert_eq!(ith_digit(456, 2), 4);
/// assert_eq!(ith_digit(456, 3), 0);
/// ```
fn ith_digit(n: u32, i: u32) -> u32 {
    // Prevent overflow.
    //
    // i <= 9 is safe beacuse 10^9 is 1 billion, which is less than 2^32 = 4 billion.
    if i > 9 {
        return 0;
    }

    n / 10u32.pow(i) % 10
}
