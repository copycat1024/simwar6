mod app;
mod launcher;
mod util;
mod widget;

use std::panic::{set_hook, PanicInfo};

fn main() {
    set_hook(Box::new(panic_hook));
    launcher::run();
}

fn panic_hook(info: &PanicInfo<'_>) {
    use backtrace::Backtrace;

    if let Some(s) = info.payload().downcast_ref::<&str>() {
        println!("Panic: {s:?}");
    } else {
        println!("Panic unknown");
    }
    let bt = Backtrace::new();
    println!("{bt:?}");
}
