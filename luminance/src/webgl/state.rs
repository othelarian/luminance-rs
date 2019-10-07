//! Graphics state.

use std::marker::PhantomData;
use stdweb::unstable::TryInto as _;

use crate::gl::blending::{BlendingState, Equation, Factor};
use crate::gl::depth_test::{DepthComparison, DepthTest};
use crate::gl::face_culling::{FaceCullingMode, FaceCullingOrder, FaceCullingState};
use crate::gl::vertex_restart::VertexRestart;
pub use crate::gl::state::{GraphicsState, StateQueryError};
use crate::gl::state::TLS_ACQUIRE_GFX_STATE;
use crate::webgl::webgl::*;

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
  fn get_from_context(ctx: &WebGL2RenderingContext) -> Result<Self, StateQueryError> {
    let blending_state = Self::get_ctx_blending_state(ctx)?;
    let blending_equation = Self::get_ctx_blending_equation(ctx)?;
    let blending_func = Self::get_ctx_blending_factors(ctx)?;
    let depth_test = Self::get_ctx_depth_test(ctx)?;
    let depth_test_comparison = DepthComparison::Less;
    let face_culling_state = Self::get_ctx_face_culling_state(ctx)?;
    let face_culling_order = Self::get_ctx_face_culling_order(ctx)?;
    let face_culling_mode = Self::get_ctx_face_culling_mode(ctx)?;
    let vertex_restart = Self::get_ctx_vertex_restart(ctx)?;
    let current_texture_unit = Self::get_ctx_current_texture_unit(ctx)?;
    let bound_textures = vec![(WebGL2RenderingContext::TEXTURE_2D, 0); 48]; // 48 is the platform minimal requirement
    let bound_uniform_buffers = vec![0; 36]; // 36 is the platform minimal requirement
    let bound_array_buffer = 0;
    let bound_element_array_buffer = 0;
    let bound_draw_framebuffer = Self::get_ctx_bound_draw_framebuffer(ctx)?;
    let bound_vertex_array = Self::get_ctx_bound_vertex_array(ctx)?;
    let current_program = Self::get_ctx_current_program(ctx)?;

    Ok(GraphicsState {
      _a: PhantomData,
      blending_state,
      blending_equation,
      blending_func,
      depth_test,
      depth_test_comparison,
      face_culling_state,
      face_culling_order,
      face_culling_mode,
      vertex_restart,
      current_texture_unit,
      bound_textures,
      bound_uniform_buffers,
      bound_array_buffer,
      bound_element_array_buffer,
      bound_draw_framebuffer,
      bound_vertex_array,
      current_program,
    })
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
    let data: GLenum = ctx.get_parameter(WebGL2RenderingContext::BLEND_EQUATION_RGB).try_into().unwrap();

    match data {
      WebGL2RenderingContext::FUNC_ADD => Ok(Equation::Additive),
      WebGL2RenderingContext::FUNC_SUBTRACT => Ok(Equation::Subtract),
      WebGL2RenderingContext::FUNC_REVERSE_SUBTRACT => Ok(Equation::ReverseSubtract),
      WebGL2RenderingContext::MIN => Ok(Equation::Min),
      WebGL2RenderingContext::MAX => Ok(Equation::Max),
      _ => Err(StateQueryError::UnknownBlendingEquation(data)),
    }
  }

  fn get_ctx_blending_factors(
    ctx: &WebGL2RenderingContext
  ) -> Result<(Factor, Factor), StateQueryError> {
    let src: GLint = ctx.get_parameter(WebGL2RenderingContext::BLEND_SRC_RGB).try_into().unwrap();
    let dst: GLint = ctx.get_parameter(WebGL2RenderingContext::BLEND_DST_RGB).try_into().unwrap();

    let src_k = Self::from_gl_blending_factor(src as GLenum).map_err(StateQueryError::UnknownBlendingSrcFactor)?;
    let dst_k = Self::from_gl_blending_factor(dst as GLenum).map_err(StateQueryError::UnknownBlendingDstFactor)?;

    Ok((src_k, dst_k))
  }

  #[inline]
  fn from_gl_blending_factor(factor: GLenum) -> Result<Factor, GLenum> {
    match factor {
      WebGL2RenderingContext::ONE => Ok(Factor::One),
      WebGL2RenderingContext::ZERO => Ok(Factor::Zero),
      WebGL2RenderingContext::SRC_COLOR => Ok(Factor::SrcColor),
      WebGL2RenderingContext::ONE_MINUS_SRC_COLOR => Ok(Factor::SrcColorComplement),
      WebGL2RenderingContext::DST_COLOR => Ok(Factor::DestColor),
      WebGL2RenderingContext::ONE_MINUS_DST_COLOR => Ok(Factor::DestColorComplement),
      WebGL2RenderingContext::SRC_ALPHA => Ok(Factor::SrcAlpha),
      WebGL2RenderingContext::ONE_MINUS_SRC_ALPHA => Ok(Factor::SrcAlphaComplement),
      WebGL2RenderingContext::DST_ALPHA => Ok(Factor::DstAlpha),
      WebGL2RenderingContext::ONE_MINUS_DST_ALPHA => Ok(Factor::DstAlphaComplement),
      WebGL2RenderingContext::SRC_ALPHA_SATURATE => Ok(Factor::SrcAlphaSaturate),
      _ => Err(factor),
    }
  }

  fn get_ctx_depth_test(ctx: &WebGL2RenderingContext) -> Result<DepthTest, StateQueryError> {
    let enabled = ctx.is_enabled(WebGL2RenderingContext::DEPTH_TEST);

    let test = if enabled {
      DepthTest::On
    } else {
      DepthTest::Off
    };

    Ok(test)
  }

  fn get_ctx_face_culling_state(
    ctx: &WebGL2RenderingContext
  ) -> Result<FaceCullingState, StateQueryError> {
    let enabled = ctx.is_enabled(WebGL2RenderingContext::CULL_FACE);

    let state = if enabled {
      FaceCullingState::On
    } else {
      FaceCullingState::Off
    };

    Ok(state)
  }

  fn get_ctx_face_culling_order(
    ctx: &WebGL2RenderingContext
  ) -> Result<FaceCullingOrder, StateQueryError> {
    let order: GLenum = ctx.get_parameter(WebGL2RenderingContext::FRONT_FACE).try_into().unwrap();

    match order {
      WebGL2RenderingContext::CCW => Ok(FaceCullingOrder::CCW),
      WebGL2RenderingContext::CW => Ok(FaceCullingOrder::CW),
      _ => Err(StateQueryError::UnknownFaceCullingOrder(order)),
    }
  }

  fn get_ctx_face_culling_mode(
    ctx: &WebGL2RenderingContext
  ) -> Result<FaceCullingMode, StateQueryError> {
    let mode: GLenum = ctx.get_parameter(WebGL2RenderingContext::CULL_FACE_MODE).try_into().unwrap();

    match mode {
      WebGL2RenderingContext::FRONT => Ok(FaceCullingMode::Front),
      WebGL2RenderingContext::BACK => Ok(FaceCullingMode::Back),
      WebGL2RenderingContext::FRONT_AND_BACK => Ok(FaceCullingMode::Both),
      _ => Err(StateQueryError::UnknownFaceCullingMode(mode)),
    }
  }

  fn get_ctx_vertex_restart(
    _: &WebGL2RenderingContext
  ) -> Result<VertexRestart, StateQueryError> {
    // implementation note: WebGL2 doesn’t allow to enable nor disable primitive restart as it’s
    // always on
    Ok(VertexRestart::On)
  }

  fn get_ctx_current_texture_unit(
    ctx: &WebGL2RenderingContext
  ) -> Result<GLenum, StateQueryError> {
    let active_texture = ctx.get_parameter(WebGL2RenderingContext::TEXTURE0).try_into().unwrap();
    Ok(active_texture)
  }

  fn get_ctx_bound_draw_framebuffer(
    ctx: &WebGL2RenderingContext
  ) -> Result<GLuint, StateQueryError> {
    let bound = ctx.get_parameter(WebGL2RenderingContext::DRAW_FRAMEBUFFER_BINDING).try_into().unwrap();
    Ok(bound)
  }

  fn get_ctx_bound_vertex_array(
    ctx: &WebGL2RenderingContext
  ) -> Result<GLuint, StateQueryError> {
    let bound = ctx.get_parameter(WebGL2RenderingContext::VERTEX_ARRAY_BINDING).try_into().unwrap();
    Ok(bound)
  }

  fn get_ctx_current_program(
    ctx: &WebGL2RenderingContext
  ) -> Result<GLuint, StateQueryError> {
    let used = ctx.get_parameter(WebGL2RenderingContext::CURRENT_PROGRAM).try_into().unwrap();
    Ok(used)
  }
}
