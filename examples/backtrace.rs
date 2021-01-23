#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
    simple_logger::SimpleLogger::new()
        .init()
        .expect("unable to initialize logger");

    sudo::escalate_if_needed().expect("sudo failed");

    failing_function();
}

#[inline(never)]
fn failing_function() -> ! {
    info!("entering failing_function");
    panic!("now you see me fail")
}
