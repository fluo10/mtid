use clap::Args;
use mtid::{Stid, Dtid, Ttid, Qtid};

use crate::cli::length_option::{LengthOption, LengthOptions};

/// Decode MTID string to integer.
/// 
#[derive(Args, Debug)]
#[command(override_usage("mtid-cli decode [<-s|-d|-t|-q>] <MTID>"))]
pub struct DecodeArgs {
    /// Forse to parse specified variants.
    #[command(flatten)]
    length: LengthOptions,
    /// MDID String
    /// 
    /// Examples: "123" "456-789" "abc-def-ghj" "kmn-pqr-stv-wxy"
    mtid: String,
}

impl DecodeArgs {
    pub fn run(self) {
        match LengthOption::from(self.length) {
            LengthOption::Unset => match (self.mtid.parse::<Stid>(), self.mtid.parse::<Dtid>(), self.mtid.parse::<Ttid>(), self.mtid.parse::<Qtid>()) {
                (Ok(x), _, _, _) => println!("{}", u16::from(x)),
                (Err(_), Ok(x), _, _) => println!("{}", u32::from(x)),
                (Err(_), Err(_), Ok(x), _) => println!("{}", u64::from(x)),
                (Err(_), Err(_), Err(_), Ok(x)) => println!("{}", u64::from(x)),
                (Err(e1), Err(e2), Err(e3), Err(e4) ) => panic!("Failed to parse all format: {:?}", (e1, e2, e3, e4))
            },
            LengthOption::Single => println!("{}", u16::from(self.mtid.parse::<Stid>().unwrap())),
            LengthOption::Double => println!("{}", u32::from(self.mtid.parse::<Dtid>().unwrap())),
            LengthOption::Triple => println!("{}", u64::from(self.mtid.parse::<Ttid>().unwrap())),
            LengthOption::Quadruple => println!("{}", u64::from(self.mtid.parse::<Qtid>().unwrap())),
        }
    }
}