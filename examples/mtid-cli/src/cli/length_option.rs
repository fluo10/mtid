use clap::Args;

/// Variant flags.
/// 
/// Specify what variant will be used.
#[derive(Args, Debug)]
#[command(next_help_heading = "Length options", about = None, long_about = None)]
#[group(multiple = false,)]
pub struct LengthOptions {

    /// Use STID (Single-length Triplet ID)
    #[arg(short, long, visible_alias = "stid")]
    pub single: bool,

    /// Use DTID (Double-length Triplet ID)
    #[arg(short, long, visible_alias = "dtid")]
    pub double: bool,

    /// Use TTID (Triple-length Triplet ID)
    #[arg(short, long, visible_alias = "ttid")]
    pub triple: bool,
    
    /// Use QTID (Quadruple-length Triplet ID)
    #[arg(short, long, visible_alias = "qtid")]
    pub quadruple: bool,
}

/// Enum parsed from LengthOptions.
pub enum LengthOption {
    Unset,
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
            (_, _, _, _) => Self::Unset,
        }
    }
}