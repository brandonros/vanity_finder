#![no_std]
#![no_main]

mod common;

extern crate alloc;

use libc_alloc::LibcAlloc;
use libc_print::libc_println;

#[global_allocator]
static GLOBAL_ALLOC: LibcAlloc = LibcAlloc;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! { 
    panic_no_std::panic(info, b'P')
}

// https://github.com/rust-lang/rust/issues/106864
#[unsafe(no_mangle)]
extern "C" fn rust_eh_personality() {}

fn get_random_u64() -> Result<u64, getrandom::Error> {
    let mut buf = [0u8; 8];
    getrandom::fill(&mut buf)?;
    Ok(u64::from_ne_bytes(buf))
}

fn get_nanoseconds() -> i64 {
    unsafe {
        let mut ts: libc::timespec = core::mem::MaybeUninit::zeroed().assume_init();
        libc::clock_gettime(libc::CLOCK_REALTIME, &mut ts);
        (ts.tv_sec as i64) * 1_000_000_000 + (ts.tv_nsec as i64)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let rng_seed = get_random_u64().unwrap();
    let vanity_prefix = b"aaaa";
    let start = get_nanoseconds();
    libc_println!("[{start}] start rng_seed: {rng_seed} vanity_prefix: {vanity_prefix:02x?}");
    let num_iterations = common::find_vanity_private_key(vanity_prefix, rng_seed);
    let end = get_nanoseconds();
    let elapsed_ns = end - start;
    let elapsed_ms = elapsed_ns / 1_000_000;
    let iterations_per_second = (num_iterations as f64 * 1_000_000_000.0) / (elapsed_ns as f64);
    libc_println!("[{end}] end num_iterations: {num_iterations} elapsed: {elapsed_ms} ms rate: {iterations_per_second:.2} iterations/second");
    unsafe {
        libc::exit(0);
    }
}
