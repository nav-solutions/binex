use std::f32::consts::PI as Pi32;

use crate::{utils::Utils, Error};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GpsEphemeris {
    /// Satellite ID #
    pub satellite_id: u8,

    /// Time of Issue of Ephemeris (in seconds of week)
    pub toe: u16,

    /// Week counter
    pub tow: i32,

    /// Time of clock (in seconds of week)
    pub toc: i32,

    /// Total group delay (in seconds)
    pub tgd: f32,

    /// Message IODC
    pub iodc: i32,

    /// Satellite clock offset (in seconds)
    pub clock_offset: f32,

    /// Satellite clock drift (in seconds per second)
    pub clock_drift: f32,

    /// Satellite clock drift rate (in seconds per s²)
    pub clock_drift_rate: f32,

    /// Message IODE
    pub iode: i32,

    /// Delta n (in radians per second)
    pub delta_n_rad_s: f32,

    /// Mean anomaly at reference time (in radians)
    pub m0_rad: f64,

    /// Orbit eccentricity
    pub e: f64,

    /// Square root of semi-major axis
    pub sqrt_a_sqrt_m: f64,

    /// Ci (sine term, in radians)
    pub cis_rad: f32,

    /// Ci (cosine term, in radians)
    pub cic_rad: f32,

    /// Cr (cosine term, in meters)
    pub crc_m: f32,

    /// Cr (sine term, in meters)
    pub crs_rad: f32,

    /// Cu (sine term, in radians)
    pub cus_rad: f32,

    /// Cu (cosine term, in radians)
    pub cuc_rad: f32,

    /// Longitude of ascending node (in radians)
    pub omega_0_rad: f64,

    /// Argument of perigee (in radians)
    pub omega_rad: f64,

    /// Inclination angle at reference time (in radians)
    pub i0_rad: f64,

    /// Rate of right ascention (in radians per second)
    pub omega_dot_rad_s: f32,

    /// Rate of inclination (in radians per second)
    pub i_dot_rad_s: f32,

    /// User Range Accuracy (URA, in meters)
    pub ura_m: f32,

    /// Satellite health flag
    pub health: u16,

    pub uint2: u16,
}

impl GpsEphemeris {
    /// Returns total number of bytes required to encode this [GpsEphemeris]
    pub const fn encoding_size() -> usize {
        128
    }

    /// Encodes this [GpsEphemeris] into mutable buffer.
    /// Returns total number of encoded bytes.
    pub(crate) fn encode(&self, big_endian: bool, buf: &mut [u8]) -> Result<usize, Error> {
        let size = Self::encoding_size();
        if buf.len() < size {
            return Err(Error::NotEnoughBytes);
        }

        buf[0] = self.satellite_id;

        let toe = if big_endian {
            self.toe.to_be_bytes()
        } else {
            self.toe.to_le_bytes()
        };

        buf[1..3].copy_from_slice(&toe);

        let tow = if big_endian {
            self.tow.to_be_bytes()
        } else {
            self.tow.to_le_bytes()
        };

        buf[3..7].copy_from_slice(&tow);

        let toc = if big_endian {
            self.toc.to_be_bytes()
        } else {
            self.toc.to_le_bytes()
        };

        buf[7..11].copy_from_slice(&toc);

        let tgd = if big_endian {
            self.tgd.to_be_bytes()
        } else {
            self.tgd.to_le_bytes()
        };

        buf[11..15].copy_from_slice(&tgd);

        let iodc = if big_endian {
            self.iodc.to_be_bytes()
        } else {
            self.iodc.to_le_bytes()
        };

        buf[15..19].copy_from_slice(&iodc);

        let af2 = if big_endian {
            self.clock_drift_rate.to_be_bytes()
        } else {
            self.clock_drift_rate.to_le_bytes()
        };

        buf[19..23].copy_from_slice(&af2);

        let af1 = if big_endian {
            self.clock_drift.to_be_bytes()
        } else {
            self.clock_drift.to_le_bytes()
        };

        buf[23..27].copy_from_slice(&af1);

        let af0 = if big_endian {
            self.clock_offset.to_be_bytes()
        } else {
            self.clock_offset.to_le_bytes()
        };

        buf[27..31].copy_from_slice(&af0);

        let iode = if big_endian {
            self.iode.to_be_bytes()
        } else {
            self.iode.to_le_bytes()
        };

        buf[31..35].copy_from_slice(&iode);

        let delta_n = if big_endian {
            (self.delta_n_rad_s / Pi32).to_be_bytes()
        } else {
            (self.delta_n_rad_s / Pi32).to_le_bytes()
        };

        buf[35..39].copy_from_slice(&delta_n);

        let m0 = if big_endian {
            self.m0_rad.to_be_bytes()
        } else {
            self.m0_rad.to_le_bytes()
        };

        buf[39..47].copy_from_slice(&m0);

        let e = if big_endian {
            self.e.to_be_bytes()
        } else {
            self.e.to_le_bytes()
        };

        buf[47..55].copy_from_slice(&e);

        let sqrt_a = if big_endian {
            self.sqrt_a_sqrt_m.to_be_bytes()
        } else {
            self.sqrt_a_sqrt_m.to_le_bytes()
        };

        buf[55..63].copy_from_slice(&sqrt_a);

        let cic = if big_endian {
            self.cic_rad.to_be_bytes()
        } else {
            self.cic_rad.to_le_bytes()
        };

        buf[63..67].copy_from_slice(&cic);

        let crc = if big_endian {
            self.crc_m.to_be_bytes()
        } else {
            self.crc_m.to_le_bytes()
        };

        buf[67..71].copy_from_slice(&crc);

        let cis = if big_endian {
            self.cis.to_be_bytes()
        } else {
            self.cis.to_le_bytes()
        };

        buf[71..75].copy_from_slice(&cis);

        let crs = if big_endian {
            self.crs_m.to_be_bytes()
        } else {
            self.crs_m.to_le_bytes()
        };

        buf[75..79].copy_from_slice(&crs);

        let cuc = if big_endian {
            self.cuc_rad.to_be_bytes()
        } else {
            self.cuc_rad.to_le_bytes()
        };

        buf[79..83].copy_from_slice(&cuc);

        let cus = if big_endian {
            self.cus_rad.to_be_bytes()
        } else {
            self.cus_rad.to_le_bytes()
        };

        buf[83..87].copy_from_slice(&cus);

        let omega_0_rad = if big_endian {
            self.omega_0_rad.to_be_bytes()
        } else {
            self.omega_0_rad.to_le_bytes()
        };

        buf[87..95].copy_from_slice(&omega_0_rad);

        let omega_rad = if big_endian {
            self.omega_rad.to_be_bytes()
        } else {
            self.omega_rad.to_le_bytes()
        };

        buf[95..103].copy_from_slice(&omega_rad);

        let i0_rad = if big_endian {
            self.i0_rad.to_be_bytes()
        } else {
            self.i0_rad.to_le_bytes()
        };

        buf[103..111].copy_from_slice(&i0_rad);

        let omega_dot_rad_s = if big_endian {
            (self.omega_dot_rad_s / Pi32).to_be_bytes()
        } else {
            (self.omega_dot_rad_s / Pi32).to_le_bytes()
        };

        buf[111..115].copy_from_slice(&omega_dot_rad_s);

        let i_dot_rad_s = if big_endian {
            (self.i_dot_rad_s / Pi32).to_be_bytes()
        } else {
            (self.i_dot_rad_s / Pi32).to_le_bytes()
        };

        buf[115..119].copy_from_slice(&i_dot_rad_s);

        let ura_m = if big_endian {
            (self.ura_m / 0.1).to_be_bytes()
        } else {
            (self.ura_m / 0.1).to_le_bytes()
        };

        buf[119..123].copy_from_slice(&ura_m);

        let health = if big_endian {
            self.health.to_be_bytes()
        } else {
            self.health.to_le_bytes()
        };

        buf[123..125].copy_from_slice(&sv_health);

        let uint2 = if big_endian {
            self.uint2.to_be_bytes()
        } else {
            self.uint2.to_le_bytes()
        };

        buf[125..127].copy_from_slice(&uint2);
        Ok(size)
    }

    /// Decodes a [GpsEphemeris] frame from read-only buffer.
    pub fn decode(big_endian: bool, buf: &[u8]) -> Result<Self, Error> {
        if buf.len() < Self::encoding_size() {
            return Err(Error::NotEnoughBytes);
        }
        // 1. PRN
        let satellite_id = buf[0];
        // 2. TOE
        let toe = Utils::decode_u16(big_endian, &buf[1..3])?;
        // 3. TOW
        let tow = Utils::decode_i32(big_endian, &buf[3..7])?;
        // 4. TOC
        let toc = Utils::decode_i32(big_endian, &buf[7..11])?;
        // 4. TGD
        let tgd = Utils::decode_f32(big_endian, &buf[11..15])?;
        // 5. IODC
        let iodc = Utils::decode_i32(big_endian, &buf[15..19])?;
        // 6. Af2
        let af2 = Utils::decode_f32(big_endian, &buf[19..23])?;
        // 7. Af1
        let af1 = Utils::decode_f32(big_endian, &buf[23..27])?;
        // 8. Af0
        let af0 = Utils::decode_f32(big_endian, &buf[27..31])?;
        // 9: IODE
        let iode = Utils::decode_i32(big_endian, &buf[31..35])?;
        // 10: delta_n
        let delta_n_rad_s = Utils::decode_f32(big_endian, &buf[35..39])? * Pi32;
        // 11: m0
        let m0_rad = Utils::decode_f64(big_endian, &buf[39..47])?;
        // 12: e
        let e = Utils::decode_f64(big_endian, &buf[47..55])?;
        // 13: sqrt_a
        let sqrt_a_sqrt_m = Utils::decode_f64(big_endian, &buf[55..63])?;
        // 14: cic
        let cic_rad = Utils::decode_f32(big_endian, &buf[63..67])?;
        // 15: crc
        let crc_m = Utils::decode_f32(big_endian, &buf[67..71])?;
        // 16: cis
        let cis_rad = Utils::decode_f32(big_endian, &buf[71..75])?;
        // 17: crs
        let crs_m = Utils::decode_f32(big_endian, &buf[75..79])?;
        // 18: cuc
        let cuc_rad = Utils::decode_f32(big_endian, &buf[79..83])?;
        // 19: cus
        let cus_rad = Utils::decode_f32(big_endian, &buf[83..87])?;
        // 20: omega0
        let omega_0_rad = Utils::decode_f64(big_endian, &buf[87..95])?;
        // 21: omega
        let omega_rad = Utils::decode_f64(big_endian, &buf[95..103])?;
        // 22: i0
        let i0_rad = Utils::decode_f64(big_endian, &buf[103..111])?;
        // 23: omega_dot
        let omega_dot_rad_s = Utils::decode_f32(big_endian, &buf[111..115])? * Pi32;
        // 24: idot
        let i_dot_rad_s = Utils::decode_f32(big_endian, &buf[115..119])? * Pi32;
        // 25: ura
        let ura_m = Utils::decode_f32(big_endian, &buf[119..123])? * 0.1;
        // 26: health
        let health = Utils::decode_u16(big_endian, &buf[123..125])?;
        // 27: uint2
        let uint2 = Utils::decode_u16(big_endian, &buf[125..127])?;

        Ok(Self {
            satellite_id,
            toe,
            tow,
            toc,
            tgd,
            iodc,
            iode,
            clock_offset: af0,
            clock_drift: af1,
            clock_drift_rate: af2,
            delta_n_rad_s,
            m0_rad,
            e,
            sqrt_a_sqrt_m,
            cic_rad,
            crc_m,
            cis_rad,
            crs_m,
            cuc_rad,
            cus_rad,
            omega_rad,
            omega_0_rad,
            i0_rad,
            i_dot_rad_s,
            omega_dot_rad_s,
            ura_m,
            health,
            uint2,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eph_x00_x01_error() {
        let buf = [0; 100];
        assert!(GpsEphemeris::decode(true, &buf).is_err());
    }

    #[test]
    fn gps_ephemeris() {
        for big_endian in [true, false] {
            let buf = [0; 128];

            let eph = GpsEphemeris::decode(big_endian, &buf).unwrap();

            // test mirror
            let mut target = [0; 100];
            assert!(eph.encode(big_endian, &mut target).is_err());

            let mut target = [0; 128];
            let size = eph.encode(big_endian, &mut target).unwrap();
            assert_eq!(size, 128);
            assert_eq!(buf, target);

            let eph = GpsEphemeris {
                satellite_id: 10,
                toe: 1000,
                tow: 120,
                toc: 130,
                tgd: 10.0,
                iodc: 24,
                clock_offset: 123.0,
                clock_drift_rate: 130.0,
                clock_drift: 150.0,
                sqrt_a_sqrt_m: 56.0,
                iode: -2000,
                delta_n_rad_s: 12.0,
                m0_rad: 0.1,
                e: 0.2,
                cic_rad: 0.3,
                crc_m: 0.4,
                cis_rad: 0.5,
                crs_m: 0.6,
                cuc_rad: 0.7,
                cus_rad: 0.8,
                omega_0_rad: 0.9,
                omega_rad: 59.0,
                i0_rad: 61.0,
                omega_dot_rad_s: 62.0,
                i_dot_rad_s: 74.0,
                ura_m: 75.0,
                sv_health: 16,
                uint2: 17,
            };

            let mut target = [0; 153];
            eph.encode(big_endian, &mut target).unwrap();

            let decoded = GpsEphemeris::decode(big_endian, &target).unwrap();
            assert_eq!(eph, decoded);
        }
    }
}
