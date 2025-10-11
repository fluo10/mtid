use clap::Args;
use mtid::{Dtid, Qtid, Stid, Ttid};

use crate::cli::length_option::{LengthOption, LengthOptions};


/// Encode integer to MTID string.
/// 
/// If variant flag is not set, most small variant will returned.
#[derive(Args, Debug)]
pub struct EncodeArgs{
    #[command(flatten)]
    length: LengthOptions,
    value: u64,
}

impl EncodeArgs {
    pub fn run(self) {
        match LengthOption::from(self.length) {
            LengthOption::Unset => {
                if self.value < (Stid::CAPACITY as u64) {
                    println!("{}", Stid::from_int_lossy(self.value as u16))
                } else if self.value < (Dtid::CAPACITY as u64) {
                    println!("{}", Dtid::from_int_lossy(self.value as u32))
                } else if self.value < Ttid::CAPACITY  {
                    println!("{}", Ttid::from_int_lossy(self.value))
                } else {
                    println!("{}", Qtid::from_int_lossy(self.value))
                }
            }, 
            LengthOption::Single => println!("{}", Stid::from_int_lossy(self.value as u16)),
            LengthOption::Double => println!("{}", Dtid::from_int_lossy(self.value as u32)),
            LengthOption::Triple => println!("{}", Ttid::from_int_lossy(self.value)),
            LengthOption::Quadruple => println!("{}", Qtid::from_int_lossy(self.value)),        }
    }
}