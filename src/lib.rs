#![doc(
    html_logo_url = "https://raw.githubusercontent.com/nav-solutions/.github/master/logos/logo2.jpg"
)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

/*
 * BINEX is part of the nav-solutions framework.
 * Authors: Guillaume W. Bres <guillaume.bressaix@gmail.com> et al.
 * (cf. https://github.com/nav-solutions/binex/graphs/contributors)
 * This framework is shipped under Mozilla Public V2 license.
 */

use thiserror::Error;

mod decoder;
mod message;
mod stream;

pub(crate) mod utils;

pub mod prelude {
    pub use crate::{
        decoder::Decoder,
        message::{
            EphemerisFrame, GALEphemeris, GLOEphemeris, GPSEphemeris, GPSRaw, GeoStringFrame,
            Message, Meta, MonumentGeoMetadata, MonumentGeoRecord, PositionEcef3d, PositionGeo3d,
            Record, SBASEphemeris, Solutions, SolutionsFrame, TemporalSolution, Velocity3d,
            VelocityNED3d,
        },
        stream::{ClosedSourceElement, Provider, StreamElement},
        ClosedSourceMeta, Error,
    };
    // re-export
    pub use hifitime::{Epoch, TimeScale};
}

use crate::message::Meta;
use crate::stream::Provider;

/// [ClosedSourceMeta] helps identify a closed source message we cannot interprate.
#[derive(Debug, Copy, Clone)]
pub struct ClosedSourceMeta {
    /// Message ID "as is"
    pub mid: u32,
    /// Message length (total payload) "as is"
    pub mlen: usize,
    /// Size of chunk.
    /// This library is designed to support all open source messages that are short.
    /// Yet a BINEX (prototype) message may span 2^27 bytes.
    pub size: usize,
    /// [Meta] data that follows the open source protocol.
    pub open_meta: Meta,
    /// [Provider] of this message. Only this organization may fully decode this message.
    pub provider: Provider,
    // payload offset in buffer
    offset: usize,
}

#[derive(Debug)]
pub enum Error {
    /// Not enough bytes available to continue decoding process
    NotEnoughBytes,
    /// I/O error
    IoError,
    /// Missing SYNC byte
    NoSyncByte,
    // InvalidStartofStream,
    /// Library limitation: reversed streams are not supported
    ReversedStream,
    /// Library limitation: enhanced CRC is not supported yet
    EnhancedCrc,
    /// Found an unsupported timescale that we cannot interprate.
    NonSupportedTimescale,
    /// Found unknown message ID
    UnknownMessage,
    /// Error while attempting to interprate UTF-8 (invalid ASCII)
    Utf8Error,
    /// Message is missing CRC field and cannot be verified
    MissingCRC,
    /// Message corrupt: received CRC does not match expected CRC
    CorrupctBadCRC,
    /// Incomplete message: need more data to complete
    IncompleteMessage(usize),
    /// Library limitation: not all open source Messages supported yet
    NonSupportedMesssage(usize),
    /// Library limtation: not all subrecords supported yet
    NonSupportedSubRecord,
    /// Library limtation: should never happen, because this library
    /// will be designed to parse all open source [Message]s.
    /// This may happen as either we're still in development (bad internal design)
    /// or for format that we still do not support (temporarily "ok")
    TooLargeInternalLimitation,
    /// Found closed source message
    ClosedSourceMessage(ClosedSourceMeta),
}
