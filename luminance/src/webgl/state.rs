//! Graphics state.

use crate::gl::blending::{BlendingState, Equation, Factor};
pub use crate::gl::state::{GraphicsState, StateQueryError};
use crate::gl::state::TLS_ACQUIRE_GFX_STATE;
use crate::webgl::webgl::WebGL2RenderingContext;

impl GraphicsState {
  /// Create a new [`GraphicsState`].
  ///
  /// > Note: keep in mind you can create only one per thread. However, if you’re building without
  /// > standard library, this function will always return successfully. You have to take extra care
  /// > in this case.
  pub fn new(mut ctx: WebGL2RenderingContext) -> Result<Self, StateQueryError> {
    TLS_ACQUIRE_GFX_STATE.with(|rc| {
      let mut inner = rc.borrow_mut();

      match *inner {
        Some(_) => {
          inner.take();
          Self::get_from_context(&mut ctx)
        }

        None => Err(StateQueryError::UnavailableGraphicsState),
      }
    })
  }

  /// Get a [`GraphicsContext`] from the current WebGL2 context.
  fn get_from_context(ctx: &mut WebGL2RenderingContext) -> Result<Self, StateQueryError> {
    unimplemented!()
  }

  /// Get blending state.
  fn get_ctx_blending_state(
    ctx: &WebGL2RenderingContext
  ) -> Result<BlendingState, StateQueryError> {
    let enabled = ctx.is_enabled(WebGL2RenderingContext::BLEND);

    if enabled {
      Ok(BlendingState::On)
    } else {
      Ok(BlendingState::Off)
    }
  }

  fn get_ctx_blending_equation(
    ctx: &WebGL2RenderingContext
  ) -> Result<Equation, StateQueryError> {
    let data = ctx.get_parameter(WebGL2RenderingContext::BLEND_EQUATION_RGB);

    let data = data as GLenum;
    match data {
      gl::FUNC_ADD => Ok(Equation::Additive),
      gl::FUNC_SUBTRACT => Ok(Equation::Subtract),
      gl::FUNC_REVERSE_SUBTRACT => Ok(Equation::ReverseSubtract),
      gl::MIN => Ok(Equation::Min),
      gl::MAX => Ok(Equation::Max),
      _ => Err(StateQueryError::UnknownBlendingEquation(data)),
    }
  }

  unsafe fn get_ctx_blending_factors() -> Result<(Factor, Factor), StateQueryError> {
    let mut src = gl::ONE as GLint;
    let mut dst = gl::ZERO as GLint;

    gl::GetIntegerv(gl::BLEND_SRC_RGB, &mut src);
    gl::GetIntegerv(gl::BLEND_DST_RGB, &mut dst);

    let src_k = Self::from_gl_blending_factor(src as GLenum).map_err(StateQueryError::UnknownBlendingSrcFactor)?;
    let dst_k = Self::from_gl_blending_factor(dst as GLenum).map_err(StateQueryError::UnknownBlendingDstFactor)?;

    Ok((src_k, dst_k))
  }
}
