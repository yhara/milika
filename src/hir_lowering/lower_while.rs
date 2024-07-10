//! Lower `while` to recursive function call.
//!
//! ## Example
//!
//! ```
//! # Before
//! fun foo() {
//!   a...
//!   while i < 3 {
//!     b...
//!     if c { return d } // Early return
//!     e...
//!   }
//!   f...
//! }
//!
//! # After
//! fun foo($env, $cont) {
//!   a...
//!   return foo_while1($env, $cont...)
//! }
//! fun foo_while1($env, $cont...)
//!   if i < 3 {
//!     b...
//!     if c { return $cont(d) }
//!     e...
//!     return foo_while1($env, $cont...)
//!   } else {
//!     return foo_while1e($env, $cont)
//!   }
//! }
//! fun foo_while1e($env, $cont) {
//!   f...
//! }
//! ```
