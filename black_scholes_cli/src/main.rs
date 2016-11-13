extern crate getopts;
extern crate black_scholes;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect(); // like sys.argv
    fn add_help(mut opts: Options) -> Options { 
        opts.optflag("h", "help", "print this help menu");
        opts
    }

    let opts = Options::new();
    let opts = black_scholes::PricingInput::add_opts(opts);
    let opts = add_help(opts);
    { 
        let just_help = Options::new();
        let just_help = add_help(just_help);
        match just_help.parse(&args[1..]) {
            Err(_) => (), // no help
            Ok(m) => { 
                if m.opt_present("h") {
                    let program = args[0].clone();
                    print_usage(&program, opts);
                    return;
                }
            }
        }
    }
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    let pricing_input = black_scholes::PricingInput::from_matches(matches);
    println!("{:?}", pricing_input);
}

