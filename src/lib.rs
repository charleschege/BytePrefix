#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]
#![doc = include_str!("../README.md")]

mod byte_sizes;
pub use byte_sizes::*;
mod parser;
pub use parser::*;
mod supported_format;
pub use supported_format::*;

#[cfg(test)]
mod sanity_checks {
    use crate::calc_bytes;

    #[test]
    fn bytes() {
        let size = 1023usize as f32;
        let formatted = calc_bytes(size);

        assert_eq!("1023.00B", formatted.as_str());
    }

    #[test]
    fn kib() {
        let size = 1024usize as f32;
        let formatted = calc_bytes(size);

        assert_eq!("1.00KiB", formatted.as_str());

        let size = (1024 * 1023usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("1023.00KiB", formatted.as_str());

        let size = (1024 * 16usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("16.00KiB", formatted.as_str());
    }

    #[test]
    fn mib() {
        let size = (1024 * 1024usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("1.00MiB", formatted.as_str());

        let size = (1024 * 1024 * 1023usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("1023.00MiB", formatted.as_str());

        let size = (1024 * 1024 * 16usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("16.00MiB", formatted.as_str());
    }

    #[test]
    fn gib() {
        let size = (1024 * 1024 * 1024usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("1.00GiB", formatted.as_str());

        let size = (1024 * 1024 * 1024 * 1023usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("1023.00GiB", formatted.as_str());

        let size = (1024 * 1024 * 1024 * 16usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("16.00GiB", formatted.as_str());
    }

    #[test]
    fn tib() {
        let size = (1024 * 1024 * 1024 * 1024usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("1.00TiB", formatted.as_str());

        let size = (1024 * 1024 * 1024 * 1024 * 1023usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("1023.00TiB", formatted.as_str());

        let size = (1024 * 1024 * 1024 * 1024 * 16usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("16.00TiB", formatted.as_str());
    }

    #[test]
    fn pib() {
        let size = (1024 * 1024 * 1024 * 1024 * 1024usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("1.00PiB", formatted.as_str());

        let size = (1024 * 1024 * 1024 * 1024 * 1024 * 1023usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("1023.00PiB", formatted.as_str());

        let size = (1024 * 1024 * 1024 * 1024 * 1024 * 16usize) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("16.00PiB", formatted.as_str());
    }

    #[test]
    fn eib() {
        let size = (1024 * 1024 * 1024 * 1024 * 1024 * 1024u128) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("1.00EiB", formatted.as_str());

        let size = (1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1023u128) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("1023.00EiB", formatted.as_str());

        let size = (1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 16u128) as f32;
        let formatted = calc_bytes(size);

        assert_eq!("16.00EiB", formatted.as_str());
    }
}
