#![feature(proc_macro)]
#[macro_use] extern crate tarpc;
#[macro_use] extern crate serde_derive;
extern crate getopts;

use std::str;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Years(f64);

impl str::FromStr for Years {
    type Err = std::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        f64::from_str(s).map(|f| Years(f))
    }
}

#[derive(Debug)]
pub enum ParseCallOrPutError {
    Invalid,
}

impl std::fmt::Display for ParseCallOrPutError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParseCallOrPutError::Invalid => write!(f, "Invalid"),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum CallOrPut { 
    Call,
    Put
}

impl str::FromStr for CallOrPut {
    type Err = ParseCallOrPutError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s { 
            "C" => Ok(CallOrPut::Call),
            "P" => Ok(CallOrPut::Put),
            _ => Err(ParseCallOrPutError::Invalid),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PricingInput { 
    pub s : f64,
    pub k : f64,
    pub time_to_exp : Years,
    pub discount_rate : f64,
    pub und_rate : f64,
    pub vol : f64,
    pub call_or_put : CallOrPut,
}

// A bunch of constant strings that I want for command-line parsing.
// I want them in the impl PricingInput block but rust won't let me
// put them there.
const STOCK_ARG : &'static str = "s";
const K_ARG : &'static str = "k";
const TIME_TO_EXP_ARG : &'static str = "t";
const DISCOUNT_RATE_ARG : &'static str = "r";
const UND_RATE_ARG : &'static str = "u";
const VOL_ARG : &'static str = "v";
const CALL_OR_PUT_ARG : &'static str = "c";

impl PricingInput { 
    pub fn add_opts(mut opts: getopts::Options) -> getopts::Options {
        opts.reqopt(STOCK_ARG, "stock", "stock price", "PRICE");
        opts.reqopt(K_ARG, "strike", "strike", "STRIKE");
        opts.reqopt(TIME_TO_EXP_ARG, "time-to-exp", "time to expiration", "TIME_IN_YEARS");
        opts.reqopt(DISCOUNT_RATE_ARG, "risk-free-rate", "discount rate (e.g. 0.01 for 1%)", "RATE");
        opts.reqopt(UND_RATE_ARG, "", "und rate (e.g. 0.01 for 1%)", "RATE");
        opts.reqopt(VOL_ARG, "vol", "vol (e.g. 0.2 for \"20 vol\")", "VOL");
        opts.reqopt(CALL_OR_PUT_ARG, "call-or-put", "call or put", "C");
        opts
    }

    pub fn from_matches(matches: getopts::Matches) -> PricingInput {
        fn parse_arg<T: str::FromStr>(matches: &getopts::Matches, arg: &str, desc: &str) -> T 
            where <T as str::FromStr>::Err: std::fmt::Display {
            match matches.opt_str(arg) {
                None => panic!("Need to supply {}", desc),
                Some(s) => match s.parse() {
                    Err(e) => panic!("Can't parse supplied {} ({}) {}", desc, s, e),
                    Ok(x) => x,
                }
            }
        }
        PricingInput { 
            s : parse_arg(&matches, STOCK_ARG, "stock price"),
            k : parse_arg(&matches, K_ARG, "strike"),
            time_to_exp : parse_arg(&matches, TIME_TO_EXP_ARG, "time to exp"),
            discount_rate : parse_arg(&matches, DISCOUNT_RATE_ARG, "discount rate"),
            und_rate : parse_arg(&matches, UND_RATE_ARG, "und rate"),
            vol : parse_arg(&matches, VOL_ARG, "vol"),
            call_or_put : parse_arg(&matches, CALL_OR_PUT_ARG, "call or put"),
        }
    }
}

pub fn price(pi : PricingInput) -> f64 {
    pi.s + 10.
}

// rpc stuff
pub mod server {
    use PricingInput;
    service! {
        rpc hello(name: String) -> String;
        rpc compute_price(input : PricingInput) -> f64;
    }
}

pub struct Server;

impl server::Service for Server {
    fn hello(&self, s: String) -> String {
        let response = format!("Hello, {}!", s);
        println!("Generated an rpc response of {}", response);
        response
    }

    fn compute_price(&self, input: PricingInput) -> f64 {
        let response = price(input);
        println!("Generated a price of {}", response);
        response
    }
}
