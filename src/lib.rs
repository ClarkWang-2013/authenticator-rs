/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
mod util;

#[cfg(any(target_os = "linux"))]
extern crate libudev;

#[cfg(any(target_os = "freebsd"))]
extern crate devd_rs;

#[cfg(any(target_os = "macos"))]
extern crate core_foundation;

extern crate boxfnonce;
extern crate libc;
#[macro_use]
extern crate log;
extern crate base64;
extern crate byteorder;
extern crate rand;
extern crate runloop;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate nom;
extern crate serde;
extern crate serde_bytes;
extern crate serde_cbor;
#[macro_use]
extern crate serde_derive;
extern crate cose;
extern crate hmac;
extern crate pretty_hex;
extern crate serde_json;
extern crate sha2;
// TODO(baloo): this has to be replaced by nss at some point
extern crate openssl;

#[cfg(test)]
#[macro_use]
extern crate hex_literal;

mod consts;
mod statemachine;
//mod u2fprotocol;
//mod u2ftypes;
mod ctap;

pub mod ctap2;
mod transport;

mod manager;
pub use manager::FidoManager;
#[allow(deprecated)]
pub use manager::U2FManager;

//mod capi;
//pub use capi::*;

// Keep this in sync with the constants in u2fhid-capi.h.
bitflags! {
    pub struct RegisterFlags: u64 {
        const REQUIRE_RESIDENT_KEY        = 1;
        const REQUIRE_USER_VERIFICATION   = 2;
        const REQUIRE_PLATFORM_ATTACHMENT = 4;
    }
}
bitflags! {
    pub struct SignFlags: u64 {
        const REQUIRE_USER_VERIFICATION = 1;
    }
}
bitflags! {
    pub struct AuthenticatorTransports: u8 {
        const USB = 1;
        const NFC = 2;
        const BLE = 4;
    }
}

#[derive(Clone)]
pub struct KeyHandle {
    pub credential: Vec<u8>,
    pub transports: AuthenticatorTransports,
}

pub type AppId = Vec<u8>;
pub type RegisterResult = Vec<u8>;
pub type SignResult = (AppId, Vec<u8>, Vec<u8>);

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Unknown = 1,
    NotSupported = 2,
    InvalidState = 3,
    ConstraintError = 4,
    NotAllowed = 5,
}

#[cfg(fuzzing)]
pub use consts::*;
#[cfg(fuzzing)]
pub use u2fprotocol::*;
#[cfg(fuzzing)]
pub use u2ftypes::*;
