//--------------------------------------------------------------
//
// This stops the dead_code and unused_imports warnings from
// cropping up durring the development of code.  These warnings
// will and should still crop up during a release build
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
//
//--------------------------------------------------------------
//
// During test writing I don't want to be bothered with warnings
// about unused variables.  It's extra noise while I'm trying to
// plan out tests.
#![cfg_attr(test, allow(unused_variables))]

//! # ArgsBar
//! > UI Component / Toolkit for Ergonomic Argument Input

/// # General Development Support
/// - Test Harnesses
/// - Factories
/// - Development Debug Helpers
/// - Scaffolding
#[cfg(test)]
mod support;

/// # Application Data Structures
pub mod models;

/// # Application Event Messages
mod events;
