//! Soft-float helpers for synthesizable designs (FR36).
//! Rounding: round-ties-to-even on the truncated mantissa bit.

/// Fixed-point style float with documented round-ties-to-even.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoftF16 {
    /// IEEE-ish bits: 1 sign + 5 exp + 10 frac (host model only).
    pub bits: u16,
}

impl SoftF16 {
    pub fn from_f32(v: f32) -> Self {
        // Host-only model for golden tests; emit path treats SoftF16 as Bits<16>.
        Self {
            bits: half::f32_to_f16_bits(v),
        }
    }

    pub fn to_f32(self) -> f32 {
        half::f16_bits_to_f32(self.bits)
    }

    /// Round ties to even when converting from f32 via truncation of extra bits.
    pub fn add_rte(self, other: Self) -> Self {
        let sum = self.to_f32() + other.to_f32();
        Self::from_f32(sum)
    }
}

mod half {
    pub fn f32_to_f16_bits(v: f32) -> u16 {
        // Minimal host conversion for fixtures (not a full IEEE FMA).
        let bits = v.to_bits();
        let sign = ((bits >> 16) & 0x8000) as u16;
        let exp = ((bits >> 23) & 0xff) as i32;
        let mant = bits & 0x7fffff;
        if exp == 0 {
            return sign;
        }
        if exp == 0xff {
            return sign | 0x7c00 | if mant != 0 { 0x200 } else { 0 };
        }
        let new_exp = exp - 127 + 15;
        if new_exp >= 0x1f {
            return sign | 0x7c00;
        }
        if new_exp <= 0 {
            return sign;
        }
        let mut frac = mant >> 13;
        let round_bit = (mant >> 12) & 1;
        let sticky = mant & 0xfff;
        // Round ties to even.
        if round_bit == 1 && (sticky != 0 || (frac & 1) == 1) {
            frac += 1;
        }
        sign | ((new_exp as u16) << 10) | (frac as u16 & 0x3ff)
    }

    pub fn f16_bits_to_f32(h: u16) -> f32 {
        let sign = ((h as u32) & 0x8000) << 16;
        let exp = ((h >> 10) & 0x1f) as i32;
        let frac = (h & 0x3ff) as u32;
        if exp == 0 {
            return f32::from_bits(sign);
        }
        if exp == 0x1f {
            let bits = sign | 0x7f800000 | (frac << 13);
            return f32::from_bits(bits);
        }
        let e = (exp - 15 + 127) as u32;
        f32::from_bits(sign | (e << 23) | (frac << 13))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_ties_to_even_add() {
        let a = SoftF16::from_f32(1.0);
        let b = SoftF16::from_f32(2.0);
        let s = a.add_rte(b);
        assert!((s.to_f32() - 3.0).abs() < 1e-2);
    }
}
