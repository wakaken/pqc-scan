//! Bounded, non-decrypting RFC 7292 public-certificate subset. No bag payloads
//! other than X.509 certificates are decoded, hashed, copied or exported.
use super::{parse_certificate_der, LocatedCertificate};
use pqc_scan_core::DetectorStatus;
use x509_parser::asn1_rs::{Any, Class, FromDer, Tag};

const MAX_INPUT: usize = 2 * 1024 * 1024;
const MAX_BAGS: usize = 256;
const MAX_DEPTH: usize = 8;
const DATA: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 7, 1];
const CERT_TYPE: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 9, 22, 1];
const BAG_PREFIX: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 12, 10, 1];

type Parsed<T> = Result<T, ()>;

pub(super) struct Inspection {
    pub objects: Vec<LocatedCertificate>,
    pub status: DetectorStatus,
    pub reason: &'static str,
}

#[derive(Default)]
struct State {
    objects: Vec<LocatedCertificate>,
    input_base: usize,
    bags: usize,
    contents: usize,
    attribute_items: usize,
    malformed: bool,
    unsupported: bool,
    encrypted: bool,
    limited: bool,
}

// Any::from_der only frames the current definite-length element. It does not
// recursively decode opaque key/encrypted contents. Exact class and constructed
// bit checks prevent an inner ASN.1 tag from masquerading as an OCTET STRING.
fn take<'a>(input: &mut &'a [u8], class: Class, tag: Tag, constructed: bool) -> Parsed<&'a [u8]> {
    let (rest, any) = Any::from_der(input).map_err(|_| ())?;
    if any.class() != class || any.tag() != tag || any.header.constructed() != constructed {
        return Err(());
    }
    *input = rest;
    Ok(any.data)
}
fn universal<'a>(input: &mut &'a [u8], tag: Tag, constructed: bool) -> Parsed<&'a [u8]> {
    take(input, Class::Universal, tag, constructed)
}
fn exact(input: &[u8], tag: Tag, constructed: bool) -> Parsed<&[u8]> {
    let mut rest = input;
    let value = universal(&mut rest, tag, constructed)?;
    if !rest.is_empty() {
        return Err(());
    }
    Ok(value)
}
fn explicit<'a>(input: &mut &'a [u8]) -> Parsed<&'a [u8]> {
    take(input, Class::ContextSpecific, Tag(0), true)
}

pub(super) fn inspect(bytes: &[u8]) -> Inspection {
    let mut state = State {
        input_base: bytes.as_ptr() as usize,
        ..State::default()
    };
    if bytes.len() > MAX_INPUT {
        state.limited = true;
    } else if pfx(bytes, &mut state).is_err() {
        state.malformed = true;
    }
    let status = if state.limited {
        DetectorStatus::Truncated
    } else if state.malformed || (!state.objects.is_empty() && state.unsupported) {
        DetectorStatus::Partial
    } else if state.unsupported {
        DetectorStatus::Unsupported
    } else {
        DetectorStatus::Complete
    };
    let reason = if state.limited {
        "inspection_limit_reached"
    } else if state.malformed {
        "malformed_structure"
    } else if state.encrypted {
        "encrypted_contents_unsupported"
    } else if state.unsupported {
        "bag_or_content_type_unsupported"
    } else {
        "unencrypted_certificates_inspected"
    };
    Inspection {
        objects: state.objects,
        status,
        reason,
    }
}

fn pfx(bytes: &[u8], state: &mut State) -> Parsed<()> {
    let mut fields = exact(bytes, Tag::Sequence, true)?;
    if universal(&mut fields, Tag::Integer, false)? != [3] {
        return Err(());
    }
    let auth = universal(&mut fields, Tag::Sequence, true)?;
    if let Some(auth) = content(auth, state)? {
        let mut contents = exact(auth, Tag::Sequence, true)?;
        while !contents.is_empty() {
            state.contents += 1;
            if state.contents > MAX_BAGS {
                state.limited = true;
                break;
            }
            let info = universal(&mut contents, Tag::Sequence, true)?;
            match content(info, state) {
                Ok(Some(safe)) => {
                    if safe_contents(safe, state, 0).is_err() {
                        state.malformed = true;
                    }
                }
                Ok(None) => {}
                Err(()) => state.malformed = true,
            }
        }
    }
    // Recover public certificates before checking optional integrity metadata.
    // Its presence never implies MAC or trust verification.
    if !fields.is_empty() {
        let mac = universal(&mut fields, Tag::Sequence, true)?;
        validate_mac(mac)?;
    }
    if !fields.is_empty() {
        return Err(());
    }
    Ok(())
}

fn content<'a>(mut fields: &'a [u8], state: &mut State) -> Parsed<Option<&'a [u8]>> {
    let oid = universal(&mut fields, Tag::Oid, false)?;
    let value = if fields.is_empty() {
        None
    } else {
        Some(explicit(&mut fields)?)
    };
    if !fields.is_empty() {
        return Err(());
    }
    if oid != DATA {
        state.unsupported = true;
        // encryptedData and envelopedData are deliberately opaque.
        state.encrypted |= matches!(oid, [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 7, 3 | 6]);
        return Ok(None);
    }
    Ok(Some(exact(value.ok_or(())?, Tag::OctetString, false)?))
}

fn safe_contents(bytes: &[u8], state: &mut State, depth: usize) -> Parsed<()> {
    if depth > MAX_DEPTH {
        state.limited = true;
        return Ok(());
    }
    let mut bags = exact(bytes, Tag::Sequence, true)?;
    while !bags.is_empty() {
        if state.bags >= MAX_BAGS {
            state.limited = true;
            break;
        }
        state.bags += 1;
        let bag = universal(&mut bags, Tag::Sequence, true)?;
        if safe_bag(bag, state, depth).is_err() {
            state.malformed = true;
        }
        if state.limited {
            break;
        }
    }
    Ok(())
}

fn safe_bag(mut fields: &[u8], state: &mut State, depth: usize) -> Parsed<()> {
    let oid = universal(&mut fields, Tag::Oid, false)?;
    let value = explicit(&mut fields)?;
    if !fields.is_empty() {
        // Attribute values (friendly names, local IDs) are never exported.
        let attributes = universal(&mut fields, Tag::Set, true)?;
        validate_attributes(attributes, state)?;
    }
    if !fields.is_empty() {
        return Err(());
    }
    let kind = oid.strip_prefix(BAG_PREFIX).and_then(|s| match s {
        [kind] => Some(*kind),
        _ => None,
    });
    match kind {
        Some(3) => {
            let mut cert = exact(value, Tag::Sequence, true)?;
            let kind = universal(&mut cert, Tag::Oid, false)?;
            let value = explicit(&mut cert)?;
            if !cert.is_empty() {
                return Err(());
            }
            if kind != CERT_TYPE {
                state.unsupported = true;
                return Ok(());
            }
            let der = exact(value, Tag::OctetString, false)?;
            let attributes = parse_certificate_der(der).map_err(|_| ())?;
            state.objects.push(LocatedCertificate {
                attributes,
                line: 1,
                column: 1,
                object_index: state.bags,
                byte_offset: der.as_ptr() as usize - state.input_base,
                pkcs12: true,
            });
        }
        Some(6) => safe_contents(value, state, depth + 1)?,
        _ => state.unsupported = true,
    }
    Ok(())
}

fn validate_mac(mut fields: &[u8]) -> Parsed<()> {
    let mut digest = universal(&mut fields, Tag::Sequence, true)?;
    let mut algorithm = universal(&mut digest, Tag::Sequence, true)?;
    universal(&mut algorithm, Tag::Oid, false)?;
    if !algorithm.is_empty() {
        let (rest, _) = Any::from_der(algorithm).map_err(|_| ())?;
        if !rest.is_empty() {
            return Err(());
        }
    }
    universal(&mut digest, Tag::OctetString, false)?;
    if !digest.is_empty() {
        return Err(());
    }
    universal(&mut fields, Tag::OctetString, false)?;
    if !fields.is_empty() {
        universal(&mut fields, Tag::Integer, false)?;
    }
    if !fields.is_empty() {
        return Err(());
    }
    Ok(())
}

// Validate only Attribute framing; unknown attribute values stay opaque.
fn validate_attributes(mut attributes: &[u8], state: &mut State) -> Parsed<()> {
    while !attributes.is_empty() {
        let mut attribute = universal(&mut attributes, Tag::Sequence, true)?;
        universal(&mut attribute, Tag::Oid, false)?;
        let mut values = universal(&mut attribute, Tag::Set, true)?;
        if !attribute.is_empty() || values.is_empty() {
            return Err(());
        }
        while !values.is_empty() {
            state.attribute_items += 1;
            if state.attribute_items > 4096 {
                state.limited = true;
                return Ok(());
            }
            let (rest, _) = Any::from_der(values).map_err(|_| ())?;
            values = rest;
        }
    }
    Ok(())
}
