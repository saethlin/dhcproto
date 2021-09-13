#![warn(
    missing_debug_implementations,
    // missing_docs, // some variants still missing docs
    missing_copy_implementations,
    rust_2018_idioms,
    unreachable_pub,
    non_snake_case,
    non_upper_case_globals
)]
#![allow(clippy::cognitive_complexity)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
//! # dhcproto
//!
//!  A DHCP parser and encoder for DHCPv4 and DHCPv6. `dhcproto` aims to be a functionally complete DHCP implementation.
//!
//! ## DHCPv4
//!
//! ```rust
//! use dhcproto::v4::{Message, Encoder, Decoder, Decodable, Encodable};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!        // decode
//!        let bytes = dhcp_offer();
//!        let msg = Message::decode(&mut Decoder::new(&bytes))?;
//!        // now encode
//!        let mut buf = Vec::new();
//!        let mut e = Encoder::new(&mut buf);
//!        msg.encode(&mut e)?;
//! # Ok(())
//! }
//!     #   fn dhcp_offer() -> Vec<u8> {
//!     #   vec![
//!     #       0x02, 0x01, 0x06, 0x00, 0x00, 0x00, 0x15, 0x5c, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0xc0, 0xa8, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0xcc, 0x00, 0x0a, 0xc4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0x82,
//!     #       0x53, 0x63, 0x35, 0x01, 0x02, 0x36, 0x04, 0xc0, 0xa8, 0x00, 0x01, 0x33, 0x04, 0x00,
//!     #       0x00, 0x00, 0x3c, 0x3a, 0x04, 0x00, 0x00, 0x00, 0x1e, 0x3b, 0x04, 0x00, 0x00, 0x00,
//!     #       0x34, 0x01, 0x04, 0xff, 0xff, 0xff, 0x00, 0x03, 0x04, 0xc0, 0xa8, 0x00, 0x01, 0x06,
//!     #       0x08, 0xc0, 0xa8, 0x00, 0x01, 0xc0, 0xa8, 0x01, 0x01, 0xff, 0x00, 0x00, 0x00, 0x00,
//!     #       0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     #   ]
//!     #    }
//! ```
//!
//! ## DHCPv6
//!
//! ```rust
//! use dhcproto::v6::{Message, Encoder, Decoder, Decodable, Encodable};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!        // decode
//!        let bytes = solicit();
//!        let msg = Message::decode(&mut Decoder::new(&bytes))?;
//!        // now encode
//!        let mut buf = Vec::new();
//!        let mut e = Encoder::new(&mut buf);
//!        msg.encode(&mut e)?;
//! # Ok(())
//! }
//! #    fn solicit() -> Vec<u8> {
//! #        vec![
//! #            0x01, 0x10, 0x08, 0x74, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x01, 0x1c, 0x39,
//! #            0xcf, 0x88, 0x08, 0x00, 0x27, 0xfe, 0x8f, 0x95, 0x00, 0x06, 0x00, 0x04, 0x00, 0x17,
//! #            0x00, 0x18, 0x00, 0x08, 0x00, 0x02, 0x00, 0x00, 0x00, 0x19, 0x00, 0x0c, 0x27, 0xfe,
//! #            0x8f, 0x95, 0x00, 0x00, 0x0e, 0x10, 0x00, 0x00, 0x15, 0x18,
//! #        ]
//! #    }
//! ```

pub use decoder::{Decodable, Decoder};
pub use encoder::{Encodable, Encoder};

pub mod decoder;
pub mod encoder;
pub mod error;
pub mod v4;
pub mod v6;