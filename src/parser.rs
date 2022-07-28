use crate::{byte_sizes::*, ByteFormat};

/// Call `calc_bytes` and pass in the size of the bytes as a `f32`
/// in order to format the size of a file into bytes
pub fn calc_bytes(size: f32) -> String {
    if size < KiB {
        byte_to_string(size, ByteFormat::B)
    } else if size >= KiB && size < MiB {
        let size_result = size / KiB;

        byte_to_string(size_result, ByteFormat::KiB)
    } else if size >= MiB && size < GiB {
        let size_result = size / MiB;

        byte_to_string(size_result, ByteFormat::MiB)
    } else if size >= GiB && size < TiB {
        let size_result = size / GiB;

        byte_to_string(size_result, ByteFormat::GiB)
    } else if size >= TiB && size < PiB {
        let size_result = size / TiB;

        byte_to_string(size_result, ByteFormat::TiB)
    } else if size >= PiB && size < EiB {
        let size_result = size / PiB;

        byte_to_string(size_result, ByteFormat::PiB)
    } else {
        let size_result = size / EiB;

        byte_to_string(size_result, ByteFormat::EiB)
    }
}

fn byte_to_string(size_result: f32, format: ByteFormat) -> String {
    let mut formatted = String::new();
    formatted.push_str(&format_args!("{:.2?}", &size_result).to_string());
    formatted.push_str(&format.to_string());

    formatted
}
