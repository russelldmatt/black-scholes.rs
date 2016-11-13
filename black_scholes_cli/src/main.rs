extern crate getopts;
extern crate black_scholes;
use getopts::Options;
use std::env;

fn print_usage_and_exit(program: &str, opts: &Options, exit_code: i32) -> ! {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
    std::process::exit(exit_code);
}

fn print_usage_and_exit_if_help_present(program: &str, opts: &Options, m: &getopts::Matches) {
    if m.opt_present("h") {
        print_usage_and_exit(&program, opts, 0)
    }
}

use black_scholes::server::*;

fn main() {
    let args: Vec<String> = env::args().collect(); // like sys.argv
    let program = args[0].clone();
    fn add_help(mut opts: Options) -> Options { 
        opts.optflag("h", "help", "print this help menu");
        opts
    }

    let opts = Options::new();
    let opts = black_scholes::PricingInput::add_opts(opts);
    let opts = add_help(opts);

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { 
            print_usage_and_exit_if_help_present(&program, &opts, &m);
            m
        },
        Err(f) => {
            println!("{}", f.to_string());
            print_usage_and_exit(&program, &opts, 1)
        },
    };
    let pricing_input = black_scholes::PricingInput::from_matches(matches);
    println!("{:?}", pricing_input);
    
    let addr = "127.0.0.1:9000";
    let client = Client::new(addr).unwrap();
    println!("{}", client.hello("Mom".to_string()).unwrap());
    assert_eq!("Hello, Mom!".to_string(),
               client.hello("Mom".to_string()).unwrap());
    println!("{}", client.compute_price(pricing_input).unwrap());
    drop(client);
}

