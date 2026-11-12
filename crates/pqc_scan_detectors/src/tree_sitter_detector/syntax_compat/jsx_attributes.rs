//! Recover quoted JSX attribute values even when entity recovery loses the tag.
//! Only ampersands in complete quoted attributes of a closed opening tag change.
fn name_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b"_$:.-".contains(&b)
}
fn quoted(source: &[u8], i: &mut usize, escape: bool) -> bool {
    let quote = source[*i];
    *i += 1;
    while *i < source.len() {
        let b = source[*i];
        *i += 1;
        if b == quote {
            return true;
        }
        if escape && b == b'\\' {
            *i = (*i + 1).min(source.len());
        }
    }
    false
}
fn braced(source: &[u8], i: &mut usize) -> bool {
    let mut depth = 0;
    while *i < source.len() {
        match source[*i] {
            b'\'' | b'"' | b'`' => {
                if !quoted(source, i, true) {
                    return false;
                }
            }
            b'/' if source[*i..].starts_with(b"//") => {
                while *i < source.len() && source[*i] != b'\n' {
                    *i += 1;
                }
            }
            b'/' if source[*i..].starts_with(b"/*") => {
                *i += 2;
                while *i < source.len() && !source[*i..].starts_with(b"*/") {
                    *i += 1;
                }
                if *i == source.len() {
                    return false;
                }
                *i += 2;
            }
            b'{' => {
                depth += 1;
                *i += 1;
            }
            b'}' => {
                depth -= 1;
                *i += 1;
                if depth == 0 {
                    return true;
                }
            }
            _ => *i += 1,
        }
    }
    false
}
fn tag(source: &[u8], start: usize, i: &mut usize, amps: &mut Vec<usize>) -> bool {
    *i = start + 1;
    if !source
        .get(*i)
        .is_some_and(|b| b.is_ascii_alphabetic() || matches!(*b, b'_' | b'$'))
    {
        return false;
    }
    while source.get(*i).is_some_and(|b| name_byte(*b)) {
        *i += 1;
    }
    loop {
        let before = *i;
        while source.get(*i).is_some_and(u8::is_ascii_whitespace) {
            *i += 1;
        }
        if source.get(*i) == Some(&b'>') {
            *i += 1;
            return true;
        }
        if source.get(*i..*i + 2) == Some(b"/>") {
            *i += 2;
            return true;
        }
        if *i == before {
            return false;
        }
        if source.get(*i) == Some(&b'{') {
            if !braced(source, i) {
                return false;
            }
            continue;
        }
        let begin = *i;
        while source.get(*i).is_some_and(|b| name_byte(*b)) {
            *i += 1;
        }
        if *i == begin {
            return false;
        }
        let name_end = *i;
        while source.get(*i).is_some_and(u8::is_ascii_whitespace) {
            *i += 1;
        }
        if source.get(*i) != Some(&b'=') {
            *i = name_end;
            continue;
        }
        *i += 1;
        while source.get(*i).is_some_and(u8::is_ascii_whitespace) {
            *i += 1;
        }
        if source.get(*i) == Some(&b'{') {
            if !braced(source, i) {
                return false;
            }
            continue;
        }
        if !source.get(*i).is_some_and(|b| matches!(*b, b'\'' | b'"')) {
            return false;
        }
        let content = *i + 1;
        if !quoted(source, i, false) {
            return false;
        }
        for (offset, b) in source[content..*i - 1].iter().enumerate() {
            if *b == b'&' {
                amps.push(content + offset);
            }
        }
    }
}
pub(super) fn normalize(
    text: &str,
    protected: &[bool],
    bytes: &mut [u8],
    remaining: &mut usize,
) -> bool {
    if !text.contains('&') {
        return true;
    }
    for (start, _) in text.match_indices('<') {
        if protected[start] {
            continue;
        }
        let mut i = start;
        let mut amps = Vec::new();
        let valid = tag(text.as_bytes(), start, &mut i, &mut amps);
        let Some(rest) = remaining.checked_sub(i.saturating_sub(start)) else {
            return false;
        };
        *remaining = rest;
        if valid {
            for offset in amps {
                bytes[offset] = b'x';
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn only_complete_quoted_attributes_change_and_work_exhaustion_is_reported() {
        let text = "<A href='/start?first-key=1&second-key=2' other={x && y} />";
        let mut bytes = text.as_bytes().to_vec();
        assert!(super::normalize(
            text,
            &vec![false; text.len()],
            &mut bytes,
            &mut 10000
        ));
        let expected = text.replace("1&second", "1xsecond");
        assert_eq!(bytes, expected.as_bytes());
        let mut bytes = text.as_bytes().to_vec();
        assert!(!super::normalize(
            text,
            &vec![false; text.len()],
            &mut bytes,
            &mut 1
        ));
        for text in ["<A href='a&b' other={ />", "<A href='a&b />"] {
            let mut bytes = text.as_bytes().to_vec();
            assert!(super::normalize(
                text,
                &vec![false; text.len()],
                &mut bytes,
                &mut 10000
            ));
            assert_eq!(bytes, text.as_bytes());
        }
    }
}
