extern crate getopts;
use std::str;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[derive(Debug)]
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


#[derive(Debug)]
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

#[derive(Debug)]
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
        opts.reqopt(STOCK_ARG, "stock", "und-price/spot/stock-price", "40.");
        opts.optopt(K_ARG, "strike", "strike", "hmm");
        opts.optopt(TIME_TO_EXP_ARG, "time-to-exp", "time to expiration", "");
        opts.optopt(DISCOUNT_RATE_ARG, "risk-free-rate", "discount rate", "");
        opts.optopt(UND_RATE_ARG, "", "und rate", "");
        opts.optopt(VOL_ARG, "", "vol", "");
        opts.optopt(CALL_OR_PUT_ARG, "", "call or put", "C");
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

pub fn price(pi : PricingInput) {
    pi.s + 10.;
}
