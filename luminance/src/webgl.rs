//! WebGL module provider.
//!
//! This module provides WebGL types and functions that are used to implement the rest of this
//! crate.

pub mod webgl {
  #![allow(missing_docs, unused_parens, non_camel_case_types, warnings)]
  include!(concat!(env!("OUT_DIR"), "/webgl_stdweb.rs"));
}

pub mod state;

pub use crate::gl::blending;
pub use crate::gl::depth_test;
pub use crate::gl::face_culling;
pub use crate::gl::linear;
pub use crate::gl::pixel;
pub use crate::gl::render_state;
pub use crate::gl::vertex;
pub use crate::gl::vertex_restart;
