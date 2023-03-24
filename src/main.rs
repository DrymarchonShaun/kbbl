extern crate getopts;
extern crate nix;

use std::fs::File;
use std::io::{Read, Write};

fn main() {

    //get options
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    //define options
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("i", "increase", "increase keyboard backlight");
    opts.optflag("d", "decrease", "decrease keyboard backlight");
    opts.optflag("m", "max", "set keyboard backlight to max");
    opts.optflag("o", "off", "turn keyboard backlight off");
    opts.optflag("s", "status", "get keyboard backlight status");
    //parse options
    let program = args[0].clone();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    //print help if -h is given
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    //run functions
    if matches.opt_present("s") {
        status();
        return;
    }
    if matches.opt_present("i") {
        check_root(opts, &program);
        increase();
        return;
    }
    if matches.opt_present("d") {
        check_root(opts, &program);
        decrease();
        return;
    }
    if matches.opt_present("m") {
        check_root(opts, &program);
        max();
        return;
    }
    if matches.opt_present("o") {
        check_root(opts, &program);
        off();
        return;
    }


    //functions
    //increase keyboard backlight
    fn increase() {
        let mut file =
            File::create("/sys/class/leds/system76_acpi::kbd_backlight/brightness").unwrap();
        file.write_all("+1".as_bytes()).unwrap();
    }
    //decrease keyboard backlight
    fn decrease() {
        let mut file =
            File::create("/sys/class/leds/system76_acpi::kbd_backlight/brightness").unwrap();
        file.write_all("-1".as_bytes()).unwrap();
    }
    //set keyboard backlight to max
    fn max() {
        let mut file =
            File::create("/sys/class/leds/system76_acpi::kbd_backlight/brightness").unwrap();
        file.write_all("255".as_bytes()).unwrap();
    }
    //turn keyboard backlight off
    fn off() {
        let mut file =
            File::create("/sys/class/leds/system76_acpi::kbd_backlight/brightness").unwrap();
        file.write_all("0".as_bytes()).unwrap();
    }
    //get keyboard backlight status
    fn status() {
        let mut file =
            File::open("/sys/class/leds/system76_acpi::kbd_backlight/brightness").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        //convert to u8
        let contents: u8 = contents.trim().parse().unwrap();
        //convert to percentage
        let contents = (contents as f32 / 255.0) * 100.0;
        //round to 0 decimal places
        let contents = contents.round();
        //convert back to string
        let contents = contents.to_string();
        //print
        println!("{}", contents);
    }
    //print help
    fn print_usage(program: &str, opts: getopts::Options) {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
    }

    //check if user is root
    fn check_root(opts: getopts::Options, program: &str) {
        if nix::unistd::geteuid() != nix::unistd::Uid::from_raw(0) {
            println!("You must be root to run this program.");
            print_usage(program, opts);
            std::process::exit(1);
        }
    }
}