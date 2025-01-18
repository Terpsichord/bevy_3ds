use ctru::prelude::*;

fn main() {
    ctru::applets::error::set_panic_hook(false);

    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();

    let mut top_screen = gfx.top_screen.borrow_mut();
    top_screen.set_wide_mode(true);
    let _console = Console::new(top_screen);


    let _soc = Soc::new().expect("Failed to SOC");
    println!("made soc");
    let mut stream = std::net::TcpStream::connect("192.168.1.118:5000").expect("Failed to connect");
    println!("connected tcp stream");
    use std::io::Write;
    stream.write_all(b"hello world").expect("failed to write");
    println!("wrote to stream");

    while apt.main_loop() {
        hid.scan_input();
        let input = hid.keys_down();


        if input.contains(KeyPad::START) {
            break;
        }
    }
}
