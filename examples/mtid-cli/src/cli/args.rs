use clap::Args;

/// Variant flags.
/// 
/// Specify what variant will be used.
#[derive(Args, Debug)]
#[command(next_help_heading = "Length options", about = None, long_about = None)]
#[group(multiple = false,)]
pub struct Length {

    /// Force to use STID (Single-length Triplet ID)
    #[arg(short, long, visible_alias = "stid")]
    single: bool,

    /// Force to use DTID (Double-length Triplet ID)
    #[arg(short, long, visible_alias = "dtid")]
    double: bool,

    /// Force to use TTID (Triple-length Triplet ID)
    #[arg(short, long, visible_alias = "ttid")]
    triple: bool,
    
    /// Force to use QTID (Quadruple-length Triplet ID)
    #[arg(short, long, visible_alias = "qtid")]
    quadruple: bool,
}