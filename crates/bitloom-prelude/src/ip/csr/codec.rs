//! Lossless, versioned specialization identity. No hash or captured host state.
use super::*;
use std::collections::BTreeMap;

// Schema v1 numeric codes are independent of Rust enum declaration order.
fn access_code(access: CsrAccess) -> u32 {
    match access {
        CsrAccess::Rw => 0,
        CsrAccess::Ro => 1,
        CsrAccess::Wo => 2,
        CsrAccess::W1c => 3,
    }
}
fn owner_code(owner: CsrOwner) -> u32 {
    match owner {
        CsrOwner::Leaf => 0,
        CsrOwner::External => 1,
        CsrOwner::None => 2,
    }
}

pub(super) fn encode(block: &CsrBlock) -> Vec<(String, u32)> {
    fn string(out: &mut Vec<(String, u32)>, key: &str, value: &str) {
        out.push((format!("{key}.len"), value.len() as u32));
        for (i, b) in value.bytes().enumerate() {
            out.push((format!("{key}.{i}"), b.into()));
        }
    }
    let mut out = vec![
        ("schema".into(), 1),
        ("count".into(), block.registers.len() as u32),
    ];
    string(&mut out, "name", &block.name);
    for (i, r) in block.registers.iter().enumerate() {
        let key = format!("r{i}");
        string(&mut out, &format!("{key}.name"), &r.name);
        for (suffix, value) in [
            ("offset", r.offset),
            ("reset", r.reset),
            ("access", access_code(r.access)),
            ("owner", owner_code(r.owner)),
            ("read_reject", r.read_reject as u32),
            ("write_reject", r.write_reject as u32),
            ("event", r.event.is_some() as u32),
            ("count", r.fields.len() as u32),
        ] {
            out.push((format!("{key}.{suffix}"), value));
        }
        if let Some(event) = &r.event {
            string(&mut out, &format!("{key}.event_name"), event);
        }
        for (j, f) in r.fields.iter().enumerate() {
            let key = format!("{key}.f{j}");
            string(&mut out, &format!("{key}.name"), &f.name);
            for (suffix, value) in [
                ("mask", f.mask as u32),
                ("reset", f.reset),
                ("access", access_code(f.access)),
            ] {
                out.push((format!("{key}.{suffix}"), value));
            }
        }
    }
    out
}
struct Reader(BTreeMap<String, u32>);
impl Reader {
    fn word(&mut self, key: &str) -> Result<u32, Diagnostics> {
        self.0
            .remove(key)
            .ok_or_else(|| error(format!("missing codec key {key}")))
    }
    fn count(&mut self, key: &str) -> Result<usize, Diagnostics> {
        let n = self.word(key)? as usize;
        // Each element consumes at least one remaining key. Check before loops
        // and allocations so hostile counts cannot exhaust memory or CPU.
        if n > self.0.len() {
            return Err(error(format!("invalid codec count {key}")));
        }
        Ok(n)
    }
    fn flag(&mut self, key: &str) -> Result<bool, Diagnostics> {
        match self.word(key)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(error(format!("invalid codec flag {key}"))),
        }
    }
    fn string(&mut self, key: &str) -> Result<String, Diagnostics> {
        let n = self.count(&format!("{key}.len"))?;
        let mut bytes = Vec::with_capacity(n);
        for i in 0..n {
            bytes.push(
                u8::try_from(self.word(&format!("{key}.{i}"))?)
                    .map_err(|_| error("codec byte exceeds 255"))?,
            );
        }
        String::from_utf8(bytes).map_err(|_| error("invalid codec UTF-8"))
    }
    fn access(&mut self, key: &str) -> Result<CsrAccess, Diagnostics> {
        match self.word(key)? {
            0 => Ok(CsrAccess::Rw),
            1 => Ok(CsrAccess::Ro),
            2 => Ok(CsrAccess::Wo),
            3 => Ok(CsrAccess::W1c),
            _ => Err(error("invalid codec access")),
        }
    }
}
pub(super) fn decode(params: &[(String, u32)]) -> Result<CsrBlock, Diagnostics> {
    let mut reader = Reader(BTreeMap::new());
    for (key, value) in params {
        if reader.0.insert(key.clone(), *value).is_some() {
            return Err(error("duplicate codec key"));
        }
    }
    if reader.word("schema")? != 1 {
        return Err(error("unknown CSR codec schema"));
    }
    let count = reader.count("count")?;
    let name = reader.string("name")?;
    let mut registers = Vec::with_capacity(count);
    for i in 0..count {
        let key = format!("r{i}");
        let name = reader.string(&format!("{key}.name"))?;
        let offset = reader.word(&format!("{key}.offset"))?;
        let reset = reader.word(&format!("{key}.reset"))?;
        let access = reader.access(&format!("{key}.access"))?;
        let owner = match reader.word(&format!("{key}.owner"))? {
            0 => CsrOwner::Leaf,
            1 => CsrOwner::External,
            2 => CsrOwner::None,
            _ => return Err(error("invalid codec owner")),
        };
        let read_reject = reader.flag(&format!("{key}.read_reject"))?;
        let write_reject = reader.flag(&format!("{key}.write_reject"))?;
        let event = if reader.flag(&format!("{key}.event"))? {
            Some(reader.string(&format!("{key}.event_name"))?)
        } else {
            None
        };
        let count = reader.count(&format!("{key}.count"))?;
        let mut fields = Vec::with_capacity(count);
        for j in 0..count {
            let key = format!("{key}.f{j}");
            fields.push(CsrField {
                name: reader.string(&format!("{key}.name"))?,
                mask: reader.word(&format!("{key}.mask"))? as u64,
                reset: reader.word(&format!("{key}.reset"))?,
                access: reader.access(&format!("{key}.access"))?,
            });
        }
        registers.push(CsrRegister {
            name,
            offset,
            reset,
            access,
            owner,
            event,
            read_reject,
            write_reject,
            fields,
        });
    }
    if !reader.0.is_empty() {
        return Err(error("unexpected CSR codec keys"));
    }
    CsrBlock { name, registers }.canonical()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> CsrBlock {
        CsrBlock {
            name: "Codec".into(),
            registers: vec![CsrRegister {
                name: "flags".into(),
                offset: 4,
                reset: 0,
                access: CsrAccess::W1c,
                owner: CsrOwner::Leaf,
                event: Some("hw_flags".into()),
                read_reject: true,
                write_reject: true,
                fields: vec![CsrField {
                    name: "bits".into(),
                    mask: 0x80000001,
                    reset: 0,
                    access: CsrAccess::W1c,
                }],
            }],
        }
    }
    #[test]
    fn csr_codec_roundtrip_and_key_order() {
        let expected = sample();
        let mut params = encode(&expected);
        assert_eq!(decode(&params).unwrap(), expected);
        params.reverse();
        assert_eq!(decode(&params).unwrap(), expected);
    }
    #[test]
    fn csr_codec_rejects_every_missing_duplicate_and_extra_key() {
        let params = encode(&sample());
        for i in 0..params.len() {
            let mut missing = params.clone();
            missing.remove(i);
            assert!(decode(&missing).is_err(), "missing {}", params[i].0);
            let mut duplicate = params.clone();
            duplicate.push(params[i].clone());
            assert!(decode(&duplicate).is_err());
        }
        let mut extra = params;
        extra.push(("surprise".into(), 0));
        assert!(decode(&extra).is_err());
    }
    #[test]
    fn csr_codec_rejects_malformed_values_and_revalidates() {
        for (key, values) in [
            ("schema", vec![0, 2, u32::MAX]),
            ("count", vec![0, 2, u32::MAX]),
            ("name.len", vec![0, 1, u32::MAX]),
            ("name.0", vec![0, 255, 256, u32::MAX]),
            ("r0.access", vec![4, u32::MAX]),
            ("r0.owner", vec![1, 2, 3, u32::MAX]),
            ("r0.count", vec![0, 2, u32::MAX]),
            ("r0.event", vec![0, 2]),
            ("r0.read_reject", vec![2]),
            ("r0.write_reject", vec![2]),
            ("r0.offset", vec![1, 65536]),
            ("r0.reset", vec![1]),
            ("r0.f0.mask", vec![0]),
            ("r0.f0.reset", vec![1]),
            ("r0.f0.access", vec![0, 4]),
        ] {
            for value in values {
                let mut params = encode(&sample());
                params.iter_mut().find(|p| p.0 == key).unwrap().1 = value;
                assert!(decode(&params).is_err(), "{key}={value}");
            }
        }
        let mut params = encode(&sample());
        params.iter_mut().find(|p| p.0 == "r0.f0.mask").unwrap().0 = "r0.f1.mask".into();
        assert!(decode(&params).is_err());
    }
}
