use clap::Args;

/// Variant flags.
///
/// Specify what variant will be used.
#[derive(Args, Debug)]
#[command(next_help_heading = "Length options", about = None, long_about = None)]
#[group(multiple = false, required = false)]
pub struct LengthOptions {
    /// [deprecated] Use CarettaIdS (Single-length Caretta ID)
    #[arg(short, long)]
    pub single: bool,

    /// [deprecated] Use CarettaIdD (Double-length Caretta ID)
    #[arg(short, long)]
    pub double: bool,

    /// [deprecated] Use CarettaIdT (Triple-length Caretta ID)
    #[arg(short, long)]
    pub triple: bool,

    /// [deprecated] Use CarettaIdQ (Quadruple-length Caretta ID)
    #[arg(short, long)]
    pub quadruple: bool,
}

/// Enum parsed from LengthOptions.
pub enum LengthOption {
    Unspecified,
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
            (false, false, false, false) => Self::Unspecified,
            (_, _, _, _) => unreachable!("one of length options must specified!"),
        }
    }
}
