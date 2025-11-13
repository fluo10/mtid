use clap::Args;

/// Variant flags.
///
/// Specify what variant will be used.
#[derive(Args, Debug)]
#[command(next_help_heading = "Length options", about = None, long_about = None)]
#[group(multiple = false, required = true)]
pub struct LengthOptions {
    /// Use CarettaIdS (Single-length Caretta ID)
    #[arg(short, long)]
    pub single: bool,

    /// Use CarettaIdD (Double-length Caretta ID)
    #[arg(short, long)]
    pub double: bool,

    /// Use CarettaIdT (Triple-length Caretta ID)
    #[arg(short, long)]
    pub triple: bool,

    /// Use CarettaIdQ (Quadruple-length Caretta ID)
    #[arg(short, long)]
    pub quadruple: bool,
}

/// Enum parsed from LengthOptions.
pub enum LengthOption {
    Single,
    Double,
    Triple,
    Quadruple,
}

impl From<LengthOptions> for LengthOption {
    fn from(value: LengthOptions) -> Self {
        match (value.single, value.double, value.triple, value.quadruple) {
            (true, false, false, false) => Self::Single,
            (false, true, false, false) => Self::Double,
            (false, false, true, false) => Self::Triple,
            (false, false, false, true) => Self::Quadruple,
            (_, _, _, _) => unreachable!("one of length options must specified!"),
        }
    }
}
