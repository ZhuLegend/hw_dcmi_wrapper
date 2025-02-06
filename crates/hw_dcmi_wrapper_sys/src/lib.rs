// bindgen generates code that triggers these lints
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]

// We avoid generating layout tests because they cause a large number of
// warnings and according to commentary are not useful. See
// https://github.com/rust-lang/rust-bindgen/issues/1651 for more.
#[cfg(not(feature = "load_dynamic"))]
pub mod bindings;

#[cfg(feature = "load_dynamic")]
pub mod bindings_dyn;

#[cfg(feature = "load_dynamic")]
pub use bindings_dyn as bindings;
