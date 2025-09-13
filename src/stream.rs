//! BINEX Stream representation
use crate::prelude::{ClosedSourceMeta, Message, Meta};

/// List of official [Message] [Provider]s.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Provider {
    /// [Provider::JPL] custom frame prototype.
    JPL,

    /// [Provideer::IGS] custom frame prototype.
    IGS,

    /// Colorado University (CU Boulder) custom frame prototype.
    ColoradoUniversity,

    /// [Provider::NRCAN] (Canada) custom frame prototype.
    NRCAN,

    /// UCAR COSMIC (<https://www.cosmic.ucar.edu>) custom frame prototype.
    UCAR,

    /// GPS Solutions Inc. custom frame prototype.
    GpsSolutions,

    /// Astech Precision custom frame prototype.
    Ashtech,

    /// Topcon Positioning Systems custom frame prototype.
    Topcon,
}

impl Provider {
    /// Tries to match an official frame [Provider] from parsed MID
    pub(crate) fn match_any(mid: u32) -> Option<Self> {
        if mid >= 0x80 && mid <= 0x87 {
            Some(Self::UCAR)
        } else if mid >= 0x88 && mid <= 0xa7 {
            Some(Self::Ashtech)
        } else if mid >= 0xa8 && mid <= 0xaf {
            Some(Self::Topcon)
        } else if mid >= 0xb0 && mid <= 0xb3 {
            Some(Self::GPSSolutions)
        } else if mid >= 0xb4 && mid <= 0xb7 {
            Some(Self::NRCAN)
        } else if mid >= 0xb8 && mid <= 0xbf {
            Some(Self::JPL)
        } else if mid >= 0xc0 && mid <= 0xc3 {
            Some(Self::ColoradoUniversity)
        } else {
            None
        }
    }
}

/// [ClosedSourceElement] describes a frame that we can encode but not fully interprete,
/// because it is either not open source, or still being prototyped.
/// This particular [StreamElement] can be either a part of a continuous serie or self sustainable.
pub struct ClosedSourceElement<'a> {
    /// [ClosedSourceMeta]
    pub closed_meta: ClosedSourceMeta,

    /// Raw data starting at first byte of undisclosed payload.
    pub raw: &'a [u8],
}

impl<'a> ClosedSourceElement<'a> {
    /// Interpret this [ClosedSourceElement] using custom undisclosed method.
    pub fn interpret(&self, f: &dyn Fn(&[u8])) {
        f(&self.raw[..self.closed_meta.size])
    }

    /// Returns reference to raw data "as is", since frame interpration has not been disclosed yet.
    pub fn raw(&self) -> &'a [u8] {
        &self.raw[..self.closed_meta.size]
    }
}

/// [StreamElement] represents one element of a continuous BINEX stream.
pub enum StreamElement<'a> {
    /// Fully open source BINEX [Message] we can encode, decode and interpret.
    OpenSource(Message),

    /// Non disclosed [ClosedSourceElement] that may be part of a continuous serie of elements.
    /// Each chunk of the serie is internally limited to 4096 bytes.
    /// Encoding and decoding is still feasible, but we cannot interpret the payload.
    /// This is most-often used for frames being prototyped.
    ClosedSource(ClosedSourceElement<'a>),
}

impl<'a> From<Message> for StreamElement<'a> {
    /// Creates an open source [Message] wrapper.
    fn from(msg: Message) -> Self {
        Self::OpenSource(msg)
    }
}

impl<'a> StreamElement<'a> {
    /// Creates a new open source [Message] ready to be encoded
    pub fn new_open_source(msg: Message) -> Self {
        Self::OpenSource(msg)
    }

    /// Creates a new self-sustained closed source [StreamElement] provided by desired [Provider].
    /// ## Inputs
    /// - meta: [Meta] data of this prototype
    /// - provider: [Provider] of this prototype.
    /// - mid: message ID
    /// - mlen: total payload length (bytes)
    /// - raw: chunk we can encode, decode but not fully interprate   
    /// - size: size of this chunk
    pub fn new_prototype(
        open_meta: Meta,
        provider: Provider,
        mid: u32,
        mlen: usize,
        raw: &'a [u8],
        size: usize,
    ) -> Self {
        Self::ClosedSource(ClosedSourceElement {
            raw,
            closed_meta: ClosedSourceMeta {
                mid,
                mlen,
                open_meta,
                provider,
                offset: 0,
                size,
            },
        })
    }

    /// Add one closed source [StreamElement]s provided by desired [Provider::JPL].
    /// While we can encode this into a BINEX stream, only this organization can fully interprate the resulting stream.
    /// ## Inputs
    /// - meta: [Meta] of this message prototype
    /// - mid: message ID
    /// - mlen: total payload length (bytes)
    /// - raw: content we can encode, decode but not interprate
    /// - total: total size of the closed source Message (bytewise).
    /// It's either equal to [Meta::mlen] if this prototype if self sustainable,
    /// or larger, in case this prototype is only one element of a serie.
    pub fn jpl_prototype(meta: Meta, mid: u32, mlen: usize, raw: &'a [u8], size: usize) -> Self {
        Self::new_prototype(meta, Provider::JPL, mid, mlen, raw, size)
    }

    /// Add one closed source [StreamElement]s provided by desired [Provider::JPL].
    /// While we can encode this into a BINEX stream, only this organization can fully interprate the resulting stream.
    /// ## Inputs
    /// - meta: [Meta] of this message prototype
    /// - raw: content we can encode, decode but not interprate
    /// - total: total size of the closed source Message (bytewise).
    /// It's either equal to [Meta::mlen] if this prototype if self sustainable,
    /// or larger, in case this prototype is only one element of a serie.
    pub fn igs_prototype(meta: Meta, mid: u32, mlen: usize, raw: &'a [u8], size: usize) -> Self {
        Self::new_prototype(meta, Provider::IGS, mid, mlen, raw, size)
    }

    /// Add one closed source [StreamElement]s provided by desired [Provider::ColoradoUniversity].
    /// While we can encode this into a BINEX stream, only this organization can fully interprate the resulting stream.
    /// ## Inputs
    /// - meta: [Meta] of this message prototype
    /// - mid: message ID
    /// - mlen: total payload length (bytes)
    /// - raw: content we can encode, decode but not interprate
    /// - total: total size of the closed source Message (bytewise).
    /// It's either equal to [Meta::mlen] if this prototype if self sustainable,
    /// or larger, in case this prototype is only one element of a serie.
    pub fn colorado_university_prototype(
        meta: Meta,
        mid: u32,
        mlen: usize,
        raw: &'a [u8],
        size: usize,
    ) -> Self {
        Self::new_prototype(meta, Provider::ColoradoUniversity, mid, mlen, raw, size)
    }

    /// Add one closed source [StreamElement]s provided by desired [Provider::NRCAN].
    /// While we can encode this into a BINEX stream, only this organization can fully interprate the resulting stream.
    /// ## Inputs
    /// - meta: [Meta] of this message prototype
    /// - mid: message ID
    /// - mlen: total payload length (bytes)
    /// - raw: content we can encode, decode but not interprate
    /// - total: total size of the closed source Message (bytewise).
    /// It's either equal to [Meta::mlen] if this prototype if self sustainable,
    /// or larger, in case this prototype is only one element of a serie.
    pub fn nrcan_prototype(meta: Meta, mid: u32, mlen: usize, raw: &'a [u8], size: usize) -> Self {
        Self::new_prototype(meta, Provider::NRCAN, mid, mlen, raw, size)
    }

    /// Add one closed source [StreamElement]s provided by desired [Provider::UCAR].
    /// While we can encode this into a BINEX stream, only this organization can fully interprate the resulting stream.
    /// ## Inputs
    /// - meta: [Meta] of this message prototype
    /// - mid: message ID
    /// - mlen: total payload length (bytes)
    /// - raw: content we can encode, decode but not interprate
    /// - total: total size of the closed source Message (bytewise).
    /// It's either equal to [Meta::mlen] if this prototype if self sustainable,
    /// or larger, in case this prototype is only one element of a serie.
    pub fn ucar_prototype(meta: Meta, mid: u32, mlen: usize, raw: &'a [u8], size: usize) -> Self {
        Self::new_prototype(meta, Provider::UCAR, mid, mlen, raw, size)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn frame_prototype_providers() {
        for (value, expected) in [
            (0x80, Provider::UCAR),
            (0x81, Provider::UCAR),
            (0x82, Provider::UCAR),
            (0x87, Provider::UCAR),
            (0x88, Provider::AshTech),
            (0x89, Provider::AshTech),
            (0xa7, Provider::AshTech),
            (0xa8, Provider::TopCon),
            (0xa9, Provider::TopCon),
            (0xaf, Provider::TopCon),
            (0xb0, Provider::GpsSolutions),
            (0xb1, Provider::GpsSolutions),
            (0xb2, Provider::GpsSolutions),
            (0xb3, Provider::GpsSolutions),
            (0xb4, Provider::NRCAN),
            (0xb5, Provider::NRCAN),
            (0xb6, Provider::NRCAN),
            (0xb7, Provider::NRCAN),
            (0xb8, Provider::JPL),
            (0xb9, Provider::JPL),
            (0xba, Provider::JPL),
            (0xbe, Provider::JPL),
            (0xbf, Provider::JPL),
            (0xc0, Provider::ColoradoUniversity),
            (0xc1, Provider::ColoradoUniversity),
            (0xc2, Provider::ColoradoUniversity),
            (0xc3, Provider::ColoradoUniversity),
        ] {
            assert_eq!(Provider::match_any(value), Some(expected));
        }

        assert_eq!(Provider::match_any(0x79).is_none());
        assert_eq!(Provider::match_any(0x7f).is_none());
        assert_eq!(Provider::match_any(0xc4.is_none()));
        assert_eq!(Provider::match_any(0xc5.is_none()));
    }
}
