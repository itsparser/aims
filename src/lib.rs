#![cfg_attr(docsrs, feature(doc_cfg))]
// #![warn(missing_docs)]
#![deny(
    missing_debug_implementations,
    clippy::print_stderr,
    clippy::print_stdout
)]

//!
//! # AIMS
//!
//! #### AIMS - Authentication and Identity Management Syncronizer is a tool support the Oauth, SAML and Directory Sync from IDP / SCIM.
//!
//! ## Getting Started
//!
//! [![GitHub stars](https://img.shields.io/github/stars/itsparser/aims.svg?style=social&label=Star&maxAge=1)](https://github.com/itsparser/aims/stargazers/)
//! If you like what we do, consider starring, commenting, sharing and contributing!
//!
//! [![Discord](https://img.shields.io/discord/753515280423977011?label=Discord)](https://discord.gg/XFEecADgXZ)
//! Join our Discord server to chat with others in the Workfoxes community!
//!
#![doc()]

pub mod oauth2;
pub mod scim;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
