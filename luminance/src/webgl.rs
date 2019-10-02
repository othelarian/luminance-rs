//! WebGL module provider.
//!
//! This module provides WebGL types and functions that are used to implement the rest of this
//! crate.

mod inner {
  #![cfg_attr(feature = "web", allow(unused_parens, non_camel_case_types))]
  #![allow(missing_docs)]

  include!(concat!(env!("OUT_DIR"), "/webgl_stdweb.rs"));
}

// some types and functions can be shared from the OpenGL implementation as they’re agnostic enough
pub use crate::gl::blending;
pub use crate::gl::depth_test;
pub use crate::gl::face_culling;
pub use crate::gl::linear;
pub use crate::gl::render_state;
pub use crate::gl::vertex;
pub use crate::gl::vertex_restart;
