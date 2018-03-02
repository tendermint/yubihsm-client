//! Object attributes specifying which operations are allowed to be performed

// Apparently bitflags isn't clippy-safe
#![allow(unknown_lints, redundant_field_names, suspicious_arithmetic_impl)]

use byteorder::{BigEndian, ByteOrder};
use failure::Error;
use std::slice;

bitflags! {
    /// Object attributes specifying which operations are allowed to be performed
    pub struct Capability: u64 {
        /// asymmetric_decrypt_ecdh: perform ECDH operation
        const ASYMMETRIC_DECRYPT_ECDH = 0x800;

        /// asymmetric_decrypt_oaep: perform RSA-OAEP decryption
        const ASYMMETRIC_DECRYPT_OAEP = 0x400;

        /// asymmetric_decrypt_pkcs: perform RSA-PKCS1v1.5 decryption
        const ASYMMETRIC_DECRYPT_PKCS = 0x200;

        /// asymmetric_gen: generate asymmetric objects
        const ASYMMETRIC_GEN = 0x10;

        /// asymmetric_sign_ecdsa: compute ECDSA digital signature
        const ASYMMETRIC_SIGN_ECDSA = 0x80;

        /// asymmetric_sign_eddsa: compute EdDSA (i.e. Ed25519) digital signature
        const ASYMMETRIC_SIGN_EDDSA = 0x100;

        /// asymmetric_sign_pkcs: compute RSA-PKCS1v1.5 digital signature
        const ASYMMETRIC_SIGN_PKCS = 0x20;

        /// asymmetric_sign_pss: compute RSA-PSS digital signature
        const ASYMMETRIC_SIGN_PSS = 0x40;

        /// attest: create attestation (i.e. X.509 certificate) about an asymmetric object
        const ATTEST = 0x4_0000_0000;

        /// audit: read the log store
        const AUDIT = 0x100_0000;

        /// delete_asymmetric: delete asymmetric key objects
        const DELETE_ASYMMETRIC = 0x200_0000_0000;

        /// delete_authkey: delete AuthKey objects
        const DELETE_AUTHKEY = 0x100_0000_0000;

        /// delete_hmac_key: delete HMACKey objects
        const DELETE_HMACKEY = 0x800_0000_0000;

        /// delete_opaque: delete opaque objects
        const DELETE_OPAQUE = 0x80_0000_0000;

        /// delete_otp_aead_key: delete OTPAEADKey objects
        const DELETE_OTP_AEAD_KEY = 0x2000_0000_0000;

        /// delete_template: delete template objects
        const DELETE_TEMPLATE = 0x1000_0000_0000;

        /// delete_wrap_key: delete WrapKey objects
        const DELETE_WRAPKEY = 0x400_0000_0000;

        /// export_under_wrap: mark an object as exportable under keywrap
        const EXPORT_UNDER_WRAP = 0x1_0000;

        /// export_wrapped: export objects under keywrap
        const EXPORT_WRAPPED = 0x1000;

        /// generate_otp_aead_key: generate OTPAEADKey objects
        const GENERATE_OTP_AEAD_KEY = 0x10_0000_0000;

        /// generate_wrapkey: generate wrapkey objects
        const GENERATE_WRAPKEY = 0x8000;

        /// get_opaque: read opaque objects
        const GET_OPAQUE = 0x1;

        /// get_option: read device-global options
        const GET_OPTION = 0x4_0000;

        /// get_randomness: extract random bytes
        const GET_RANDOMNESS = 0x8_0000;

        /// get_template: read template objects
        const GET_TEMPLATE = 0x400_0000;

        /// hmackey_generate: generate HMACKey objects
        const HMACKEY_GENERATE = 0x20_0000;

        /// hmac_data: compute HMAC for data
        const HMAC_DATA = 0x40_0000;

        /// hmac_verify: verify HMAC for data
        const HMAC_VERIFY = 0x80_0000;

        /// import_wrapped: import keywrapped objects
        const IMPORT_WRAPPED = 0x2000;

        /// otp_aead_create: create an OTP AEAD
        const OTP_AEAD_CREATE = 0x4000_0000;

        /// otp_aead_random: create an OTP AEAD from random data
        const OTP_AEAD_RANDOM = 0x8000_0000;

        /// otp_aead_rewrap_from: rewrap AEADs from one OTPAEADKey Object to another
        const OTP_AEAD_REWRAP_FROM = 0x1_0000_0000;

        /// otp_aead_rewrap_to: rewrap AEADs to one OTPAEADKey Object from another
        const OTP_AEAD_REWRAP_TO = 0x2_0000_0000;

        /// otp_decrypt: decrypt OTP
        const OTP_DECRYPT = 0x2000_0000;

        /// put_asymmetric: write asymmetric objects
        const PUT_ASYMMETRIC =  0x8;

        /// put_authkey: write AuthKey objects
        const PUT_AUTHKEY = 0x4;

        /// put_hmackey: write HMACKey objects
        const PUT_HMACKEY = 0x10_0000;

        /// put_opaque: Write Opaque Objects
        const PUT_OPAQUE = 0x2;

        /// put_option: write device-global options
        const PUT_OPTION = 0x2_0000;

        /// put_otp_aead_key: write OTPAEADKey objects
        const PUT_OTP_AEAD_KEY = 0x8_0000_0000;

        /// put_template: write template objects
        const PUT_TEMPLATE = 0x800_0000;

        /// put_wrapkey: write WrapKey objects
        const PUT_WRAPKEY = 0x4000;

        /// reset: factory reset the device
        const RESET = 0x1000_0000;

        /// ssh_certify: sign SSH certificates
        const SSH_CERTIFY = 0x200_0000;

        /// unwrap_data: unwrap user-provided data
        const UNWRAP_DATA = 0x40_0000_0000;

        /// wrap_data: wrap user-provided data
        const WRAP_DATA = 0x20_0000_0000;
    }
}

impl Capability {
    /// Set of all capabilities
    pub fn iter() -> slice::Iter<'static, Self> {
        // TODO: does bitflags provide this functionality?
        [
            Self::ASYMMETRIC_DECRYPT_ECDH,
            Self::ASYMMETRIC_DECRYPT_OAEP,
            Self::ASYMMETRIC_DECRYPT_PKCS,
            Self::ASYMMETRIC_GEN,
            Self::ASYMMETRIC_SIGN_ECDSA,
            Self::ASYMMETRIC_SIGN_EDDSA,
            Self::ASYMMETRIC_SIGN_PKCS,
            Self::ASYMMETRIC_SIGN_PSS,
            Self::ATTEST,
            Self::AUDIT,
            Self::DELETE_ASYMMETRIC,
            Self::DELETE_AUTHKEY,
            Self::DELETE_HMACKEY,
            Self::DELETE_OPAQUE,
            Self::DELETE_OTP_AEAD_KEY,
            Self::DELETE_TEMPLATE,
            Self::DELETE_WRAPKEY,
            Self::EXPORT_UNDER_WRAP,
            Self::EXPORT_WRAPPED,
            Self::GENERATE_OTP_AEAD_KEY,
            Self::GENERATE_WRAPKEY,
            Self::GET_OPAQUE,
            Self::GET_OPTION,
            Self::GET_RANDOMNESS,
            Self::GET_TEMPLATE,
            Self::HMACKEY_GENERATE,
            Self::HMAC_DATA,
            Self::HMAC_VERIFY,
            Self::IMPORT_WRAPPED,
            Self::OTP_AEAD_CREATE,
            Self::OTP_AEAD_RANDOM,
            Self::OTP_AEAD_REWRAP_FROM,
            Self::OTP_AEAD_REWRAP_TO,
            Self::OTP_DECRYPT,
            Self::PUT_ASYMMETRIC,
            Self::PUT_AUTHKEY,
            Self::PUT_HMACKEY,
            Self::PUT_OPAQUE,
            Self::PUT_OPTION,
            Self::PUT_OTP_AEAD_KEY,
            Self::PUT_TEMPLATE,
            Self::PUT_WRAPKEY,
            Self::RESET,
            Self::SSH_CERTIFY,
            Self::UNWRAP_DATA,
            Self::WRAP_DATA,
        ].iter()
    }

    /// Parse capabilities from a byte serialization
    pub fn parse(bytes: &[u8]) -> Result<Vec<Self>, Error> {
        if bytes.len() != 8 {
            bail!("invalid capability length {} (expected {})", bytes.len(), 8);
        }

        let bitfield = BigEndian::read_u64(bytes);
        let mut result = vec![];

        for capability in Self::iter() {
            if bitfield & capability.bits() != 0 {
                result.push(*capability);
            }
        }

        Ok(result)
    }

    /// Convert an array of Capability objects to a 64-bit integer bitfield
    pub fn bitfield(capabilities: &[Self]) -> u64 {
        capabilities
            .iter()
            .fold(0, |result, capability| result | capability.bits())
    }
}
