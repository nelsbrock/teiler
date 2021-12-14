use integer_sqrt::IntegerSquareRoot;
use std::{cmp, env, process, thread};

type Num = u128;

const MAX_THREADS_DIVISOR: usize = 65536;

fn threaded_divisors(number: Num) -> Vec<Num> {
    let upper_limit = number.integer_sqrt() + 1;
    let threads = cmp::max(
        cmp::min(
            num_cpus::get_physical(),
            upper_limit as usize / MAX_THREADS_DIVISOR,
        ),
        1,
    );

    let mut handles = Vec::with_capacity(threads);

    for thread_num in 0..(threads as Num) {
        let handle = thread::spawn(move || {
            (thread_num + 1..upper_limit)
                .step_by(threads)
                .filter(|d| number % *d == 0)
                .flat_map(|d| {
                    [
                        Some(d),
                        if number / d == d {
                            None
                        } else {
                            Some(number / d)
                        },
                    ]
                })
                .flatten()
                .collect::<Vec<_>>()
        });
        handles.push(handle);
    }

    handles
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect()
}

fn main() {
    let mut args = env::args_os();
    let program_name = args
        .next()
        .and_then(|s| s.into_string().ok())
        .unwrap_or_else(|| "(program name)".into());
    let number = match args
        .next()
        .and_then(|s| s.to_str().and_then(|s| s.parse().ok()))
    {
        Some(number) => number,
        None => {
            eprintln!("USAGE: {} <number>", program_name);
            process::exit(1);
        }
    };

    let mut divs = threaded_divisors(number);
    divs.sort_unstable();
    for divisor in divs {
        println!("{}", divisor);
    }
}
