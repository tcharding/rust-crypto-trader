//! Implements the API for Inedependent Reserve crypto exchange.
//!
//! Private methods require authentication using an API key, signature, and a
//! nonce.

mod private;
mod public;

pub use private::*;
pub use public::*;

// Authentication
//
// All private API methods require authentication. All method
// parameters (except signature) are required to authenticate a
// request. There are three additional parameters which should be
// passed to private API methods:
// - API Key
// - Nonce
// - Signature

// API key
//
// To generate an API Key, go to the Settings page, click "API
// Keys" and then click "generate". Select the level of access to
// grant the key and reenter your password to confirm the creation of
// the key. Ensure that you select the lowest level of access required
// for your usage, the recommended level being Read-only.

// Nonce
//
// The nonce is a 64 bit unsigned integer. The nonce must
// increase with each request made to the API.

// Example: If the nonce is set to 1 in the first request, it must be
// set to at least 2 in the subsequent request. It is not necessary to
// start with 1. A common practice is to use unix time for this
// parameter.

// Signature
//
// Signature is a HMAC-SHA256 encoded message. The message
// is comma-separated string containing the API method URL, and a
// comma separated list of all method parameters (except signature) in
// the form: "parameterName=parameterValue". The HMAC-SHA256 code must
// be generated using the API Secret that was generated with your API
// key. This code must be converted to it's hexadecimal
// representation.
