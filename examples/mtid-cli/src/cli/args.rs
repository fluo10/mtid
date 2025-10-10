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