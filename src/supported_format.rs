/// An enum representing the supported formats
#[derive(Debug)]
pub enum ByteFormat {
    /// `Byte` formatted  as `B`
    B,
    /// `KibiByte` formatted  as `KiB`
    KiB,
    /// `MebiByte` formatted  as `MiB`
    MiB,
    /// `GigiByte` formatted  as `GiB`
    GiB,
    /// `Tebibyte` formatted  as `TiB`
    TiB,
    /// `Pebibyte` formatted  as `PiB`
    PiB,
    /// `Exbibyte` formatted  as `EiB`
    EiB,
}

use core::fmt::Display;

impl Display for ByteFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::B => "B",
            Self::KiB => "KiB",
            Self::MiB => "MiB",
            Self::GiB => "GiB",
            Self::TiB => "TiB",
            Self::PiB => "PiB",
            Self::EiB => "EiB",
        })
    }
}
