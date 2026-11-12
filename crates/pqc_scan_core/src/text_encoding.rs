//! Strict wide-Unicode decoding with conservative BOM-less recognition.
fn utf16(bytes: &[u8], little: bool) -> Option<String> {
    if !bytes.len().is_multiple_of(2) {
        return None;
    }
    let units: Vec<_> = bytes
        .chunks_exact(2)
        .map(|b| {
            if little {
                u16::from_le_bytes([b[0], b[1]])
            } else {
                u16::from_be_bytes([b[0], b[1]])
            }
        })
        .collect();
    String::from_utf16(&units).ok()
}
fn utf32(bytes: &[u8], little: bool) -> Option<String> {
    if !bytes.len().is_multiple_of(4) {
        return None;
    }
    bytes
        .chunks_exact(4)
        .map(|b| {
            char::from_u32(if little {
                u32::from_le_bytes([b[0], b[1], b[2], b[3]])
            } else {
                u32::from_be_bytes([b[0], b[1], b[2], b[3]])
            })
        })
        .collect()
}
pub(crate) fn has_wide_bom(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0xff, 0xfe])
        || bytes.starts_with(&[0xfe, 0xff])
        || bytes.starts_with(&[0, 0, 0xfe, 0xff])
}
fn plausible(text: &str) -> bool {
    !text.is_empty()
        && text
            .chars()
            .all(|c| !c.is_control() || matches!(c, '\n' | '\r' | '\t' | '\u{c}'))
}
pub(crate) fn decode(bytes: &[u8]) -> Option<String> {
    if let Some(rest) = bytes.strip_prefix(&[0xff, 0xfe, 0, 0]) {
        return utf32(rest, true);
    }
    if let Some(rest) = bytes.strip_prefix(&[0, 0, 0xfe, 0xff]) {
        return utf32(rest, false);
    }
    if let Some(rest) = bytes.strip_prefix(&[0xff, 0xfe]) {
        return utf16(rest, true);
    }
    if let Some(rest) = bytes.strip_prefix(&[0xfe, 0xff]) {
        return utf16(rest, false);
    }
    if let Some(rest) = bytes.strip_prefix(&[0xef, 0xbb, 0xbf]) {
        return String::from_utf8(rest.to_vec()).ok();
    }
    let sample = &bytes[..bytes.len().min(1024)];
    if !sample.contains(&0) {
        return Some(String::from_utf8_lossy(bytes).into_owned());
    }
    // Require a strong ASCII lane signal; arbitrary binary is never lossily decoded.
    for little in [true, false] {
        if bytes.len() >= 16 && bytes.len().is_multiple_of(4) {
            let units = sample.chunks_exact(4);
            let count = units.len();
            let ascii = units
                .filter(|b| {
                    if little {
                        b[1..] == [0, 0, 0] && b[0] > 0 && b[0] < 128
                    } else {
                        b[..3] == [0, 0, 0] && b[3] > 0 && b[3] < 128
                    }
                })
                .count();
            if ascii * 4 >= count * 3 {
                if let Some(s) = utf32(bytes, little).filter(|s| plausible(s)) {
                    return Some(s);
                }
            }
        }
    }
    for little in [true, false] {
        if bytes.len() >= 8 && bytes.len().is_multiple_of(2) {
            let units = sample.chunks_exact(2);
            let count = units.len();
            let ascii = units
                .filter(|b| {
                    if little {
                        b[1] == 0 && b[0] > 0 && b[0] < 128
                    } else {
                        b[0] == 0 && b[1] > 0 && b[1] < 128
                    }
                })
                .count();
            if ascii * 4 >= count * 3 {
                if let Some(s) = utf16(bytes, little).filter(|s| plausible(s)) {
                    return Some(s);
                }
            }
        }
    }
    None
}

/// Transient decoded-UTF-8 to original-file byte boundaries for native locations.
/// Ordinary UTF-8 requires no map. No source contents are retained in the map.
pub(crate) fn source_offsets(bytes: &[u8], text: &str) -> Option<Vec<(usize, usize)>> {
    let (width, bom) =
        if bytes.starts_with(&[0xff, 0xfe, 0, 0]) || bytes.starts_with(&[0, 0, 0xfe, 0xff]) {
            (4, 4)
        } else if bytes.starts_with(&[0xff, 0xfe]) || bytes.starts_with(&[0xfe, 0xff]) {
            (2, 2)
        } else if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
            (1, 3)
        } else if bytes[..bytes.len().min(1024)].contains(&0) {
            if bytes.len() == text.encode_utf16().count() * 2 {
                (2, 0)
            } else if bytes.len() == text.chars().count() * 4 {
                (4, 0)
            } else {
                return None;
            }
        } else {
            return None;
        };
    let mut raw = bom;
    let mut map = Vec::with_capacity(text.chars().count() + 1);
    for (offset, c) in text.char_indices() {
        map.push((offset, raw));
        raw += match width {
            2 => c.len_utf16() * 2,
            4 => 4,
            _ => c.len_utf8(),
        };
    }
    map.push((text.len(), raw));
    Some(map)
}
