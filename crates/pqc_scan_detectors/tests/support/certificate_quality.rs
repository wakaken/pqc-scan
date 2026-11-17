use super::*;
fn tlv(tag: u8, body: &[u8]) -> Vec<u8> {
    let mut out = vec![tag];
    if body.len() < 128 {
        out.push(body.len() as u8);
    } else {
        let n = u16::try_from(body.len()).unwrap();
        out.extend_from_slice(&[0x82, (n >> 8) as u8, n as u8]);
    }
    out.extend_from_slice(body);
    out
}
fn fields(input: &[u8]) -> Vec<Vec<u8>> {
    let (_, a) = Any::from_der(input).unwrap();
    let mut rest = a.as_bytes();
    let mut out = vec![];
    while !rest.is_empty() {
        let (r, _) = Any::from_der(rest).unwrap();
        out.push(rest[..rest.len() - r.len()].to_vec());
        rest = r;
    }
    out
}
const RSA: &[u8] = include_bytes!("../fixtures/certificates/rsa2048.der");
fn spki(oid: &[u8], parameters: Option<Vec<u8>>, bits: &[u8]) -> Vec<u8> {
    let mut alg = tlv(6, oid);
    if let Some(p) = parameters {
        alg.extend(p);
    }
    tlv(
        0x30,
        &[tlv(0x30, &alg), tlv(3, &[vec![0], bits.to_vec()].concat())].concat(),
    )
}

#[test]
fn sequence_and_certificate_envelopes_reject_trailing_duplicate_and_wrong_class_fields() {
    for der in [
        tlv(0x70, &[]),
        tlv(0x10, &[]),
        tlv(0x30, &[5, 0, 5, 0]),
        [tlv(0x30, &[]), vec![5, 0]].concat(),
    ] {
        assert!(sequence_fields(&der, 1).is_err());
    }
    assert!(validate_certificate_layout(&tlv(0x30, &[])).is_err());
    let mut outer = fields(RSA);
    let original = outer.clone();
    let tbs = fields(&outer[0]);
    outer[0] = tlv(0x30, &tbs[..3].concat());
    assert!(validate_certificate_layout(&tlv(0x30, &outer.concat())).is_err());
    let mut bad = tbs.clone();
    bad[0] = tlv(0xa0, &[2, 1, 2, 5, 0]);
    outer = original.clone();
    outer[0] = tlv(0x30, &bad.concat());
    assert!(validate_certificate_layout(&tlv(0x30, &outer.concat())).is_err());
    let mut bad = tbs.clone();
    bad[4] = tlv(0x30, &tlv(0x17, b"200101000000Z"));
    outer = original.clone();
    outer[0] = tlv(0x30, &bad.concat());
    assert!(validate_certificate_layout(&tlv(0x30, &outer.concat())).is_err());
    let mut bad = tbs[..7].to_vec();
    bad.push(tlv(0x82, &[0]));
    bad.push(tlv(0x81, &[0]));
    outer = original.clone();
    outer[0] = tlv(0x30, &bad.concat());
    assert!(validate_certificate_layout(&tlv(0x30, &outer.concat())).is_err());
    // An X.509 v1 certificate lacks the explicit v3 field; it is still a valid envelope.
    outer = original;
    outer[0] = tlv(0x30, &tbs[1..7].concat());
    assert!(validate_certificate_layout(&tlv(0x30, &outer.concat())).is_ok());
    let extra = [RSA, &[5, 0]].concat();
    assert!(parse_certificate_der(&extra).is_err());
}

#[test]
fn signature_disagreement_and_nonzero_bit_padding_never_claim_complete() {
    let outer = fields(RSA);
    let mut bad = outer.clone();
    bad[1] = tlv(0x30, &tlv(6, &[0x2b, 0x65, 0x70]));
    assert_eq!(
        parse_certificate_der(&tlv(0x30, &bad.concat()))
            .unwrap()
            .status,
        DetectorStatus::Partial
    );
    let (_, bit) = Any::from_der(&outer[2]).unwrap();
    let mut bits = bit.as_bytes().to_vec();
    bits[0] = 1;
    let last = bits.len() - 1;
    bits[last] &= 0xfe;
    let mut bad = outer;
    bad[2] = tlv(3, &bits);
    assert_eq!(
        parse_certificate_der(&tlv(0x30, &bad.concat()))
            .unwrap()
            .status,
        DetectorStatus::Partial
    );
}

#[test]
fn rsa_algorithm_parameters_and_trailing_integers_preserve_partial_status() {
    let rsa_oid = [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 1, 1];
    let pss_oid = [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 1, 10];
    let outer = fields(RSA);
    let tbs = fields(&outer[0]);
    let oldspki = fields(&tbs[6]);
    let (_, b) = Any::from_der(&oldspki[1]).unwrap();
    let bits = &b.as_bytes()[1..];
    for (oid, params, expected) in [
        (rsa_oid, None, DetectorStatus::Complete),
        (rsa_oid, Some(vec![5, 0]), DetectorStatus::Complete),
        (rsa_oid, Some(tlv(6, &[0x2a, 3])), DetectorStatus::Partial),
        (pss_oid, None, DetectorStatus::Complete),
        (pss_oid, Some(tlv(0x30, &[])), DetectorStatus::Complete),
        (pss_oid, Some(vec![5, 0]), DetectorStatus::Partial),
    ] {
        let a = parse_public_key_der(&spki(&oid, params, bits)).unwrap();
        assert_eq!(a.status, expected);
        if expected == DetectorStatus::Partial {
            assert!(a.key_bits.is_none());
        }
    }
    let trailing = [bits, &[5, 0]].concat();
    let a = parse_public_key_der(&spki(&rsa_oid, None, &trailing)).unwrap();
    assert_eq!(a.status, DetectorStatus::Partial);
    assert!(a.key_bits.is_none());
}

#[test]
fn dsa_requires_three_positive_integer_parameters_and_one_complete_public_integer() {
    let oid = [0x2a, 0x86, 0x48, 0xce, 0x38, 4, 1];
    let good = tlv(0x30, &[2, 1, 23, 2, 1, 11, 2, 1, 2]);
    for (params, y, complete) in [
        (Some(good.clone()), vec![2, 1, 4], true),
        (None, vec![2, 1, 4], false),
        (Some(tlv(6, &[0x2a, 3])), vec![2, 1, 4], false),
        (
            Some(tlv(0x30, &[2, 1, 23, 2, 1, 11, 2, 1, 2, 2, 1, 5])),
            vec![2, 1, 4],
            false,
        ),
        (Some(tlv(0x30, &[2, 1, 23, 2, 1, 11])), vec![2, 1, 4], false),
        (
            Some(tlv(0x30, &[2, 1, 23, 2, 1, 11, 4, 1, 2])),
            vec![2, 1, 4],
            false,
        ),
        (Some(good.clone()), vec![2, 1, 4, 5, 0], false),
        (Some(good.clone()), vec![4, 1, 4], false),
        (Some(good), vec![2, 1, 0x80], false),
    ] {
        let a = parse_public_key_der(&spki(&oid, params, &y)).unwrap();
        assert_eq!(
            a.status,
            if complete {
                DetectorStatus::Complete
            } else {
                DetectorStatus::Partial
            }
        );
    }
}

#[test]
fn non_oid_algorithm_parameters_are_reported_as_other_not_as_an_oid() {
    let oid = [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 1, 10];
    for parameters in [tlv(0x30, &[]), tlv(4, &[1])] {
        let parsed =
            parse_public_key_der(&spki(&oid, Some(parameters), &[0x30, 6, 2, 1, 1, 2, 1, 3]))
                .unwrap();
        assert_eq!(parsed.parameters_kind, "other");
        assert!(parsed.parameters_oid.is_none());
    }
}

#[test]
fn curve_oid_bytes_inside_an_octet_string_do_not_become_named_curve_facts() {
    let ec_oid = [0x2a, 0x86, 0x48, 0xce, 0x3d, 2, 1];
    let named_curve = [0x2a, 0x86, 0x48, 0xce, 0x3d, 3, 1, 7];
    let public_point = [vec![4], vec![0; 64]].concat();
    let parsed =
        parse_public_key_der(&spki(&ec_oid, Some(tlv(4, &named_curve)), &public_point)).unwrap();
    assert_eq!(parsed.parameters_kind, "other");
    assert!(parsed.parameters_oid.is_none());
    assert!(parsed.named_curve_oid.is_none());
    assert!(parsed.key_bits.is_none());
    assert_eq!(parsed.status, DetectorStatus::Unsupported);
}

#[test]
fn context_specific_or_constructed_parameters_cannot_impersonate_universal_types() {
    let rsa_oid = [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 1, 1, 1];
    for tag in [0x85, 0x25] {
        let parsed = parse_public_key_der(&spki(
            &rsa_oid,
            Some(tlv(tag, &[])),
            &[0x30, 6, 2, 1, 1, 2, 1, 3],
        ))
        .unwrap();
        assert_eq!(parsed.parameters_kind, "other");
        assert_eq!(parsed.status, DetectorStatus::Partial);
        assert!(parsed.key_bits.is_none());
    }
    let ec_oid = [0x2a, 0x86, 0x48, 0xce, 0x3d, 2, 1];
    for tag in [0x86, 0x26] {
        let parsed = parse_public_key_der(&spki(
            &ec_oid,
            Some(tlv(tag, &[0x2a, 0x86, 0x48, 0xce, 0x3d, 3, 1, 7])),
            &[vec![4], vec![0; 64]].concat(),
        ))
        .unwrap();
        assert!(parsed.named_curve_oid.is_none());
        assert!(parsed.key_bits.is_none());
    }
    let dsa_oid = [0x2a, 0x86, 0x48, 0xce, 0x38, 4, 1];
    for (parameters, y) in [
        (tlv(0xb0, &[2, 1, 23, 2, 1, 11, 2, 1, 2]), vec![2, 1, 4]),
        (tlv(0x30, &[0x82, 1, 23, 2, 1, 11, 2, 1, 2]), vec![2, 1, 4]),
        (tlv(0x30, &[2, 1, 23, 2, 1, 11, 2, 1, 2]), vec![0x82, 1, 4]),
    ] {
        let parsed = parse_public_key_der(&spki(&dsa_oid, Some(parameters), &y)).unwrap();
        assert_eq!(parsed.status, DetectorStatus::Partial);
    }
}
