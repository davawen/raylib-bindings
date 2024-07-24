#![allow(non_snake_case, non_camel_case_types, unused)]
use std::ffi;

pub const RLGL_VERSION: &str = "5.0";
pub const RL_DEFAULT_BATCH_BUFFER_ELEMENTS: i32 = 8192;
/// Default number of batch buffers (multi-buffering)
pub const RL_DEFAULT_BATCH_BUFFERS: i32 = 1;
/// Default number of batch draw calls (by state changes: mode, texture)
pub const RL_DEFAULT_BATCH_DRAWCALLS: i32 = 256;
/// Maximum number of textures units that can be activated on batch drawing (SetShaderValueTexture())
pub const RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS: i32 = 4;
/// Maximum size of Matrix stack
pub const RL_MAX_MATRIX_STACK_SIZE: i32 = 32;
/// Maximum number of shader locations supported
pub const RL_MAX_SHADER_LOCATIONS: i32 = 32;
/// GL_TEXTURE_WRAP_S
pub const RL_TEXTURE_WRAP_S: i32 = 10242;
/// GL_TEXTURE_WRAP_T
pub const RL_TEXTURE_WRAP_T: i32 = 10243;
/// GL_TEXTURE_MAG_FILTER
pub const RL_TEXTURE_MAG_FILTER: i32 = 10240;
/// GL_TEXTURE_MIN_FILTER
pub const RL_TEXTURE_MIN_FILTER: i32 = 10241;
/// GL_NEAREST
pub const RL_TEXTURE_FILTER_NEAREST: i32 = 9728;
/// GL_LINEAR
pub const RL_TEXTURE_FILTER_LINEAR: i32 = 9729;
/// GL_NEAREST_MIPMAP_NEAREST
pub const RL_TEXTURE_FILTER_MIP_NEAREST: i32 = 9984;
/// GL_NEAREST_MIPMAP_LINEAR
pub const RL_TEXTURE_FILTER_NEAREST_MIP_LINEAR: i32 = 9986;
/// GL_LINEAR_MIPMAP_NEAREST
pub const RL_TEXTURE_FILTER_LINEAR_MIP_NEAREST: i32 = 9985;
/// GL_LINEAR_MIPMAP_LINEAR
pub const RL_TEXTURE_FILTER_MIP_LINEAR: i32 = 9987;
/// Anisotropic filter (custom identifier)
pub const RL_TEXTURE_FILTER_ANISOTROPIC: i32 = 12288;
/// Texture mipmap bias, percentage ratio (custom identifier)
pub const RL_TEXTURE_MIPMAP_BIAS_RATIO: i32 = 16384;
/// GL_REPEAT
pub const RL_TEXTURE_WRAP_REPEAT: i32 = 10497;
/// GL_CLAMP_TO_EDGE
pub const RL_TEXTURE_WRAP_CLAMP: i32 = 33071;
/// GL_MIRRORED_REPEAT
pub const RL_TEXTURE_WRAP_MIRROR_REPEAT: i32 = 33648;
/// GL_MIRROR_CLAMP_EXT
pub const RL_TEXTURE_WRAP_MIRROR_CLAMP: i32 = 34626;
/// GL_MODELVIEW
pub const RL_MODELVIEW: i32 = 5888;
/// GL_PROJECTION
pub const RL_PROJECTION: i32 = 5889;
/// GL_TEXTURE
pub const RL_TEXTURE: i32 = 5890;
/// GL_LINES
pub const RL_LINES: i32 = 1;
/// GL_TRIANGLES
pub const RL_TRIANGLES: i32 = 4;
/// GL_QUADS
pub const RL_QUADS: i32 = 7;
/// GL_UNSIGNED_BYTE
pub const RL_UNSIGNED_BYTE: i32 = 5121;
/// GL_FLOAT
pub const RL_FLOAT: i32 = 5126;
/// GL_STREAM_DRAW
pub const RL_STREAM_DRAW: i32 = 35040;
/// GL_STREAM_READ
pub const RL_STREAM_READ: i32 = 35041;
/// GL_STREAM_COPY
pub const RL_STREAM_COPY: i32 = 35042;
/// GL_STATIC_DRAW
pub const RL_STATIC_DRAW: i32 = 35044;
/// GL_STATIC_READ
pub const RL_STATIC_READ: i32 = 35045;
/// GL_STATIC_COPY
pub const RL_STATIC_COPY: i32 = 35046;
/// GL_DYNAMIC_DRAW
pub const RL_DYNAMIC_DRAW: i32 = 35048;
/// GL_DYNAMIC_READ
pub const RL_DYNAMIC_READ: i32 = 35049;
/// GL_DYNAMIC_COPY
pub const RL_DYNAMIC_COPY: i32 = 35050;
/// GL_FRAGMENT_SHADER
pub const RL_FRAGMENT_SHADER: i32 = 35632;
/// GL_VERTEX_SHADER
pub const RL_VERTEX_SHADER: i32 = 35633;
/// GL_COMPUTE_SHADER
pub const RL_COMPUTE_SHADER: i32 = 37305;
/// GL_ZERO
pub const RL_ZERO: i32 = 0;
/// GL_ONE
pub const RL_ONE: i32 = 1;
/// GL_SRC_COLOR
pub const RL_SRC_COLOR: i32 = 768;
/// GL_ONE_MINUS_SRC_COLOR
pub const RL_ONE_MINUS_SRC_COLOR: i32 = 769;
/// GL_SRC_ALPHA
pub const RL_SRC_ALPHA: i32 = 770;
/// GL_ONE_MINUS_SRC_ALPHA
pub const RL_ONE_MINUS_SRC_ALPHA: i32 = 771;
/// GL_DST_ALPHA
pub const RL_DST_ALPHA: i32 = 772;
/// GL_ONE_MINUS_DST_ALPHA
pub const RL_ONE_MINUS_DST_ALPHA: i32 = 773;
/// GL_DST_COLOR
pub const RL_DST_COLOR: i32 = 774;
/// GL_ONE_MINUS_DST_COLOR
pub const RL_ONE_MINUS_DST_COLOR: i32 = 775;
/// GL_SRC_ALPHA_SATURATE
pub const RL_SRC_ALPHA_SATURATE: i32 = 776;
/// GL_CONSTANT_COLOR
pub const RL_CONSTANT_COLOR: i32 = 32769;
/// GL_ONE_MINUS_CONSTANT_COLOR
pub const RL_ONE_MINUS_CONSTANT_COLOR: i32 = 32770;
/// GL_CONSTANT_ALPHA
pub const RL_CONSTANT_ALPHA: i32 = 32771;
/// GL_ONE_MINUS_CONSTANT_ALPHA
pub const RL_ONE_MINUS_CONSTANT_ALPHA: i32 = 32772;
/// GL_FUNC_ADD
pub const RL_FUNC_ADD: i32 = 32774;
/// GL_MIN
pub const RL_MIN: i32 = 32775;
/// GL_MAX
pub const RL_MAX: i32 = 32776;
/// GL_FUNC_SUBTRACT
pub const RL_FUNC_SUBTRACT: i32 = 32778;
/// GL_FUNC_REVERSE_SUBTRACT
pub const RL_FUNC_REVERSE_SUBTRACT: i32 = 32779;
/// GL_BLEND_EQUATION
pub const RL_BLEND_EQUATION: i32 = 32777;
/// GL_BLEND_EQUATION_RGB   // (Same as BLEND_EQUATION)
pub const RL_BLEND_EQUATION_RGB: i32 = 32777;
/// GL_BLEND_EQUATION_ALPHA
pub const RL_BLEND_EQUATION_ALPHA: i32 = 34877;
/// GL_BLEND_DST_RGB
pub const RL_BLEND_DST_RGB: i32 = 32968;
/// GL_BLEND_SRC_RGB
pub const RL_BLEND_SRC_RGB: i32 = 32969;
/// GL_BLEND_DST_ALPHA
pub const RL_BLEND_DST_ALPHA: i32 = 32970;
/// GL_BLEND_SRC_ALPHA
pub const RL_BLEND_SRC_ALPHA: i32 = 32971;
/// GL_BLEND_COLOR
pub const RL_BLEND_COLOR: i32 = 32773;
/// GL_READ_FRAMEBUFFER
pub const RL_READ_FRAMEBUFFER: i32 = 36008;
/// GL_DRAW_FRAMEBUFFER
pub const RL_DRAW_FRAMEBUFFER: i32 = 36009;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_POSITION: i32 = 0;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD: i32 = 1;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_NORMAL: i32 = 2;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_COLOR: i32 = 3;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TANGENT: i32 = 4;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD2: i32 = 5;
/// Dynamic vertex buffers (position + texcoords + colors + indices arrays)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct rlVertexBuffer {
    /// Number of elements in the buffer (QUADS)
    pub elementCount: ffi::c_int,
    /// Vertex position (XYZ - 3 components per vertex) (shader-location = 0)
    pub vertices: *mut ffi::c_float,
    /// Vertex texture coordinates (UV - 2 components per vertex) (shader-location = 1)
    pub texcoords: *mut ffi::c_float,
    /// Vertex normal (XYZ - 3 components per vertex) (shader-location = 2)
    pub normals: *mut ffi::c_float,
    /// Vertex colors (RGBA - 4 components per vertex) (shader-location = 3)
    pub colors: *mut ffi::c_uchar,
    /// Vertex indices (in case vertex data comes indexed) (6 indices per quad)
    pub indices: *mut ffi::c_uint,
    /// OpenGL Vertex Array Object id
    pub vaoId: ffi::c_uint,
    /// OpenGL Vertex Buffer Objects id (5 types of vertex data)
    pub vboId: [ffi::c_uint; 5],
}
/// of those state-change happens (this is done in core module)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct rlDrawCall {
    /// Drawing mode: LINES, TRIANGLES, QUADS
    pub mode: ffi::c_int,
    /// Number of vertex of the draw
    pub vertexCount: ffi::c_int,
    /// Number of vertex required for index alignment (LINES, TRIANGLES)
    pub vertexAlignment: ffi::c_int,
    /// Texture id to be used on the draw -\> Use to create new draw call if changes
    pub textureId: ffi::c_uint,
}
/// rlRenderBatch type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct rlRenderBatch {
    /// Number of vertex buffers (multi-buffering support)
    pub bufferCount: ffi::c_int,
    /// Current buffer tracking in case of multi-buffering
    pub currentBuffer: ffi::c_int,
    /// Dynamic buffer(s) for vertex data
    pub vertexBuffer: *mut rlVertexBuffer,
    /// Draw calls array, depends on textureId
    pub draws: *mut rlDrawCall,
    /// Draw calls counter
    pub drawCounter: ffi::c_int,
    /// Current depth value for next draw
    pub currentDepth: ffi::c_float,
}



#[link(name = "raylib", kind = "static")]
extern "C" {
/// Choose the current matrix to be transformed
pub fn rlMatrixMode(mode: ffi::c_int, );
/// Push the current matrix to stack
pub fn rlPushMatrix();
/// Pop latest inserted matrix from stack
pub fn rlPopMatrix();
/// Reset current matrix to identity matrix
pub fn rlLoadIdentity();
/// Multiply the current matrix by a translation matrix
pub fn rlTranslatef(x: ffi::c_float, y: ffi::c_float, z: ffi::c_float, );
/// Multiply the current matrix by a rotation matrix
pub fn rlRotatef(angle: ffi::c_float, x: ffi::c_float, y: ffi::c_float, z: ffi::c_float, );
/// Multiply the current matrix by a scaling matrix
pub fn rlScalef(x: ffi::c_float, y: ffi::c_float, z: ffi::c_float, );
/// Multiply the current matrix by another matrix
pub fn rlMultMatrixf(matf: *const ffi::c_float, );
pub fn rlFrustum(left: ffi::c_double, right: ffi::c_double, bottom: ffi::c_double, top: ffi::c_double, znear: ffi::c_double, zfar: ffi::c_double, );
pub fn rlOrtho(left: ffi::c_double, right: ffi::c_double, bottom: ffi::c_double, top: ffi::c_double, znear: ffi::c_double, zfar: ffi::c_double, );
/// Set the viewport area
pub fn rlViewport(x: ffi::c_int, y: ffi::c_int, width: ffi::c_int, height: ffi::c_int, );
/// Set clip planes distances
pub fn rlSetClipPlanes(near: ffi::c_double, far: ffi::c_double, );
/// Get cull plane distance near
pub fn rlGetCullDistanceNear() -> ffi::c_double;
/// Get cull plane distance far
pub fn rlGetCullDistanceFar() -> ffi::c_double;
/// Initialize drawing mode (how to organize vertex)
pub fn rlBegin(mode: ffi::c_int, );
/// Finish vertex providing
pub fn rlEnd();
/// Define one vertex (position) - 2 int
pub fn rlVertex2i(x: ffi::c_int, y: ffi::c_int, );
/// Define one vertex (position) - 2 float
pub fn rlVertex2f(x: ffi::c_float, y: ffi::c_float, );
/// Define one vertex (position) - 3 float
pub fn rlVertex3f(x: ffi::c_float, y: ffi::c_float, z: ffi::c_float, );
/// Define one vertex (texture coordinate) - 2 float
pub fn rlTexCoord2f(x: ffi::c_float, y: ffi::c_float, );
/// Define one vertex (normal) - 3 float
pub fn rlNormal3f(x: ffi::c_float, y: ffi::c_float, z: ffi::c_float, );
/// Define one vertex (color) - 4 byte
pub fn rlColor4ub(r: ffi::c_uchar, g: ffi::c_uchar, b: ffi::c_uchar, a: ffi::c_uchar, );
/// Define one vertex (color) - 3 float
pub fn rlColor3f(x: ffi::c_float, y: ffi::c_float, z: ffi::c_float, );
/// Define one vertex (color) - 4 float
pub fn rlColor4f(x: ffi::c_float, y: ffi::c_float, z: ffi::c_float, w: ffi::c_float, );
/// Enable vertex array (VAO, if supported)
pub fn rlEnableVertexArray(vaoId: ffi::c_uint, ) -> bool;
/// Disable vertex array (VAO, if supported)
pub fn rlDisableVertexArray();
/// Enable vertex buffer (VBO)
pub fn rlEnableVertexBuffer(id: ffi::c_uint, );
/// Disable vertex buffer (VBO)
pub fn rlDisableVertexBuffer();
/// Enable vertex buffer element (VBO element)
pub fn rlEnableVertexBufferElement(id: ffi::c_uint, );
/// Disable vertex buffer element (VBO element)
pub fn rlDisableVertexBufferElement();
/// Enable vertex attribute index
pub fn rlEnableVertexAttribute(index: ffi::c_uint, );
/// Disable vertex attribute index
pub fn rlDisableVertexAttribute(index: ffi::c_uint, );
/// Enable attribute state pointer
pub fn rlEnableStatePointer(vertexAttribType: ffi::c_int, buffer: *mut ffi::c_void, );
/// Disable attribute state pointer
pub fn rlDisableStatePointer(vertexAttribType: ffi::c_int, );
/// Select and active a texture slot
pub fn rlActiveTextureSlot(slot: ffi::c_int, );
/// Enable texture
pub fn rlEnableTexture(id: ffi::c_uint, );
/// Disable texture
pub fn rlDisableTexture();
/// Enable texture cubemap
pub fn rlEnableTextureCubemap(id: ffi::c_uint, );
/// Disable texture cubemap
pub fn rlDisableTextureCubemap();
/// Set texture parameters (filter, wrap)
pub fn rlTextureParameters(id: ffi::c_uint, param: ffi::c_int, value: ffi::c_int, );
/// Set cubemap parameters (filter, wrap)
pub fn rlCubemapParameters(id: ffi::c_uint, param: ffi::c_int, value: ffi::c_int, );
/// Enable shader program
pub fn rlEnableShader(id: ffi::c_uint, );
/// Disable shader program
pub fn rlDisableShader();
/// Enable render texture (fbo)
pub fn rlEnableFramebuffer(id: ffi::c_uint, );
/// Disable render texture (fbo), return to default framebuffer
pub fn rlDisableFramebuffer();
/// Get the currently active render texture (fbo), 0 for default framebuffer
pub fn rlGetActiveFramebuffer() -> ffi::c_uint;
/// Activate multiple draw color buffers
pub fn rlActiveDrawBuffers(count: ffi::c_int, );
/// Blit active framebuffer to main framebuffer
pub fn rlBlitFramebuffer(srcX: ffi::c_int, srcY: ffi::c_int, srcWidth: ffi::c_int, srcHeight: ffi::c_int, dstX: ffi::c_int, dstY: ffi::c_int, dstWidth: ffi::c_int, dstHeight: ffi::c_int, bufferMask: ffi::c_int, );
/// Bind framebuffer (FBO)
pub fn rlBindFramebuffer(target: ffi::c_uint, framebuffer: ffi::c_uint, );
/// Enable color blending
pub fn rlEnableColorBlend();
/// Disable color blending
pub fn rlDisableColorBlend();
/// Enable depth test
pub fn rlEnableDepthTest();
/// Disable depth test
pub fn rlDisableDepthTest();
/// Enable depth write
pub fn rlEnableDepthMask();
/// Disable depth write
pub fn rlDisableDepthMask();
/// Enable backface culling
pub fn rlEnableBackfaceCulling();
/// Disable backface culling
pub fn rlDisableBackfaceCulling();
/// Color mask control
pub fn rlColorMask(r: bool, g: bool, b: bool, a: bool, );
/// Set face culling mode
pub fn rlSetCullFace(mode: ffi::c_int, );
/// Enable scissor test
pub fn rlEnableScissorTest();
/// Disable scissor test
pub fn rlDisableScissorTest();
/// Scissor test
pub fn rlScissor(x: ffi::c_int, y: ffi::c_int, width: ffi::c_int, height: ffi::c_int, );
/// Enable wire mode
pub fn rlEnableWireMode();
/// Enable point mode
pub fn rlEnablePointMode();
/// Disable wire mode ( and point ) maybe rename
pub fn rlDisableWireMode();
/// Set the line drawing width
pub fn rlSetLineWidth(width: ffi::c_float, );
/// Get the line drawing width
pub fn rlGetLineWidth() -> ffi::c_float;
/// Enable line aliasing
pub fn rlEnableSmoothLines();
/// Disable line aliasing
pub fn rlDisableSmoothLines();
/// Enable stereo rendering
pub fn rlEnableStereoRender();
/// Disable stereo rendering
pub fn rlDisableStereoRender();
/// Check if stereo render is enabled
pub fn rlIsStereoRenderEnabled() -> bool;
/// Clear color buffer with color
pub fn rlClearColor(r: ffi::c_uchar, g: ffi::c_uchar, b: ffi::c_uchar, a: ffi::c_uchar, );
/// Clear used screen buffers (color and depth)
pub fn rlClearScreenBuffers();
/// Check and log OpenGL error codes
pub fn rlCheckErrors();
/// Set blending mode
pub fn rlSetBlendMode(mode: ffi::c_int, );
/// Set blending mode factor and equation (using OpenGL factors)
pub fn rlSetBlendFactors(glSrcFactor: ffi::c_int, glDstFactor: ffi::c_int, glEquation: ffi::c_int, );
/// Set blending mode factors and equations separately (using OpenGL factors)
pub fn rlSetBlendFactorsSeparate(glSrcRGB: ffi::c_int, glDstRGB: ffi::c_int, glSrcAlpha: ffi::c_int, glDstAlpha: ffi::c_int, glEqRGB: ffi::c_int, glEqAlpha: ffi::c_int, );
/// Initialize rlgl (buffers, shaders, textures, states)
pub fn rlglInit(width: ffi::c_int, height: ffi::c_int, );
/// De-initialize rlgl (buffers, shaders, textures)
pub fn rlglClose();
/// Load OpenGL extensions (loader function required)
pub fn rlLoadExtensions(loader: *mut ffi::c_void, );
/// Get current OpenGL version
pub fn rlGetVersion() -> ffi::c_int;
/// Set current framebuffer width
pub fn rlSetFramebufferWidth(width: ffi::c_int, );
/// Get default framebuffer width
pub fn rlGetFramebufferWidth() -> ffi::c_int;
/// Set current framebuffer height
pub fn rlSetFramebufferHeight(height: ffi::c_int, );
/// Get default framebuffer height
pub fn rlGetFramebufferHeight() -> ffi::c_int;
/// Get default texture id
pub fn rlGetTextureIdDefault() -> ffi::c_uint;
/// Get default shader id
pub fn rlGetShaderIdDefault() -> ffi::c_uint;
/// Get default shader locations
pub fn rlGetShaderLocsDefault() -> *mut ffi::c_int;
/// Load a render batch system
pub fn rlLoadRenderBatch(numBuffers: ffi::c_int, bufferElements: ffi::c_int, ) -> rlRenderBatch;
/// Unload render batch system
pub fn rlUnloadRenderBatch(batch: rlRenderBatch, );
/// Draw render batch data (Update-\>Draw-\>Reset)
pub fn rlDrawRenderBatch(batch: *mut rlRenderBatch, );
/// Set the active render batch for rlgl (NULL for default internal)
pub fn rlSetRenderBatchActive(batch: *mut rlRenderBatch, );
/// Update and draw internal render batch
pub fn rlDrawRenderBatchActive();
/// Check internal buffer overflow for a given number of vertex
pub fn rlCheckRenderBatchLimit(vCount: ffi::c_int, ) -> bool;
/// Set current texture for render batch and check buffers limits
pub fn rlSetTexture(id: ffi::c_uint, );
/// Load vertex array (vao) if supported
pub fn rlLoadVertexArray() -> ffi::c_uint;
/// Load a vertex buffer object
pub fn rlLoadVertexBuffer(buffer: *const ffi::c_void, size: ffi::c_int, dynamic: bool, ) -> ffi::c_uint;
/// Load vertex buffer elements object
pub fn rlLoadVertexBufferElement(buffer: *const ffi::c_void, size: ffi::c_int, dynamic: bool, ) -> ffi::c_uint;
/// Update vertex buffer object data on GPU buffer
pub fn rlUpdateVertexBuffer(bufferId: ffi::c_uint, data: *const ffi::c_void, dataSize: ffi::c_int, offset: ffi::c_int, );
/// Update vertex buffer elements data on GPU buffer
pub fn rlUpdateVertexBufferElements(id: ffi::c_uint, data: *const ffi::c_void, dataSize: ffi::c_int, offset: ffi::c_int, );
/// Unload vertex array (vao)
pub fn rlUnloadVertexArray(vaoId: ffi::c_uint, );
/// Unload vertex buffer object
pub fn rlUnloadVertexBuffer(vboId: ffi::c_uint, );
/// Set vertex attribute data configuration
pub fn rlSetVertexAttribute(index: ffi::c_uint, compSize: ffi::c_int, r#type: ffi::c_int, normalized: bool, stride: ffi::c_int, offset: ffi::c_int, );
/// Set vertex attribute data divisor
pub fn rlSetVertexAttributeDivisor(index: ffi::c_uint, divisor: ffi::c_int, );
/// Set vertex attribute default value, when attribute to provided
pub fn rlSetVertexAttributeDefault(locIndex: ffi::c_int, value: *const ffi::c_void, attribType: ffi::c_int, count: ffi::c_int, );
/// Draw vertex array (currently active vao)
pub fn rlDrawVertexArray(offset: ffi::c_int, count: ffi::c_int, );
/// Draw vertex array elements
pub fn rlDrawVertexArrayElements(offset: ffi::c_int, count: ffi::c_int, buffer: *const ffi::c_void, );
/// Draw vertex array (currently active vao) with instancing
pub fn rlDrawVertexArrayInstanced(offset: ffi::c_int, count: ffi::c_int, instances: ffi::c_int, );
/// Draw vertex array elements with instancing
pub fn rlDrawVertexArrayElementsInstanced(offset: ffi::c_int, count: ffi::c_int, buffer: *const ffi::c_void, instances: ffi::c_int, );
/// Load texture data
pub fn rlLoadTexture(data: *const ffi::c_void, width: ffi::c_int, height: ffi::c_int, format: ffi::c_int, mipmapCount: ffi::c_int, ) -> ffi::c_uint;
/// Load depth texture/renderbuffer (to be attached to fbo)
pub fn rlLoadTextureDepth(width: ffi::c_int, height: ffi::c_int, useRenderBuffer: bool, ) -> ffi::c_uint;
/// Load texture cubemap data
pub fn rlLoadTextureCubemap(data: *const ffi::c_void, size: ffi::c_int, format: ffi::c_int, ) -> ffi::c_uint;
/// Update texture with new data on GPU
pub fn rlUpdateTexture(id: ffi::c_uint, offsetX: ffi::c_int, offsetY: ffi::c_int, width: ffi::c_int, height: ffi::c_int, format: ffi::c_int, data: *const ffi::c_void, );
/// Get OpenGL internal formats
pub fn rlGetGlTextureFormats(format: ffi::c_int, glInternalFormat: *mut ffi::c_uint, glFormat: *mut ffi::c_uint, glType: *mut ffi::c_uint, );
/// Get name string for pixel format
pub fn rlGetPixelFormatName(format: ffi::c_uint, ) -> *const ffi::c_char;
/// Unload texture from GPU memory
pub fn rlUnloadTexture(id: ffi::c_uint, );
/// Generate mipmap data for selected texture
pub fn rlGenTextureMipmaps(id: ffi::c_uint, width: ffi::c_int, height: ffi::c_int, format: ffi::c_int, mipmaps: *mut ffi::c_int, );
/// Read texture pixel data
pub fn rlReadTexturePixels(id: ffi::c_uint, width: ffi::c_int, height: ffi::c_int, format: ffi::c_int, ) -> *mut ffi::c_void;
/// Read screen pixel data (color buffer)
pub fn rlReadScreenPixels(width: ffi::c_int, height: ffi::c_int, ) -> *mut ffi::c_uchar;
/// Load an empty framebuffer
pub fn rlLoadFramebuffer() -> ffi::c_uint;
/// Attach texture/renderbuffer to a framebuffer
pub fn rlFramebufferAttach(fboId: ffi::c_uint, texId: ffi::c_uint, attachType: ffi::c_int, texType: ffi::c_int, mipLevel: ffi::c_int, );
/// Verify framebuffer is complete
pub fn rlFramebufferComplete(id: ffi::c_uint, ) -> bool;
/// Delete framebuffer from GPU
pub fn rlUnloadFramebuffer(id: ffi::c_uint, );
/// Load shader from code strings
pub fn rlLoadShaderCode(vsCode: *const ffi::c_char, fsCode: *const ffi::c_char, ) -> ffi::c_uint;
/// Compile custom shader and return shader id (type: RL_VERTEX_SHADER, RL_FRAGMENT_SHADER, RL_COMPUTE_SHADER)
pub fn rlCompileShader(shaderCode: *const ffi::c_char, r#type: ffi::c_int, ) -> ffi::c_uint;
/// Load custom shader program
pub fn rlLoadShaderProgram(vShaderId: ffi::c_uint, fShaderId: ffi::c_uint, ) -> ffi::c_uint;
/// Unload shader program
pub fn rlUnloadShaderProgram(id: ffi::c_uint, );
/// Get shader location uniform
pub fn rlGetLocationUniform(shaderId: ffi::c_uint, uniformName: *const ffi::c_char, ) -> ffi::c_int;
/// Get shader location attribute
pub fn rlGetLocationAttrib(shaderId: ffi::c_uint, attribName: *const ffi::c_char, ) -> ffi::c_int;
/// Set shader value uniform
pub fn rlSetUniform(locIndex: ffi::c_int, value: *const ffi::c_void, uniformType: ffi::c_int, count: ffi::c_int, );
/// Set shader value matrix
pub fn rlSetUniformMatrix(locIndex: ffi::c_int, mat: Matrix, );
/// Set shader value sampler
pub fn rlSetUniformSampler(locIndex: ffi::c_int, textureId: ffi::c_uint, );
/// Set shader currently active (id and locations)
pub fn rlSetShader(id: ffi::c_uint, locs: *mut ffi::c_int, );
/// Load compute shader program
pub fn rlLoadComputeShaderProgram(shaderId: ffi::c_uint, ) -> ffi::c_uint;
/// Dispatch compute shader (equivalent to *draw* for graphics pipeline)
pub fn rlComputeShaderDispatch(groupX: ffi::c_uint, groupY: ffi::c_uint, groupZ: ffi::c_uint, );
/// Load shader storage buffer object (SSBO)
pub fn rlLoadShaderBuffer(size: ffi::c_uint, data: *const ffi::c_void, usageHint: ffi::c_int, ) -> ffi::c_uint;
/// Unload shader storage buffer object (SSBO)
pub fn rlUnloadShaderBuffer(ssboId: ffi::c_uint, );
/// Update SSBO buffer data
pub fn rlUpdateShaderBuffer(id: ffi::c_uint, data: *const ffi::c_void, dataSize: ffi::c_uint, offset: ffi::c_uint, );
/// Bind SSBO buffer
pub fn rlBindShaderBuffer(id: ffi::c_uint, index: ffi::c_uint, );
/// Read SSBO buffer data (GPU-\>CPU)
pub fn rlReadShaderBuffer(id: ffi::c_uint, dest: *mut ffi::c_void, count: ffi::c_uint, offset: ffi::c_uint, );
/// Copy SSBO data between buffers
pub fn rlCopyShaderBuffer(destId: ffi::c_uint, srcId: ffi::c_uint, destOffset: ffi::c_uint, srcOffset: ffi::c_uint, count: ffi::c_uint, );
/// Get SSBO buffer size
pub fn rlGetShaderBufferSize(id: ffi::c_uint, ) -> ffi::c_uint;
/// Bind image texture
pub fn rlBindImageTexture(id: ffi::c_uint, index: ffi::c_uint, format: ffi::c_int, readonly: bool, );
/// Get internal modelview matrix
pub fn rlGetMatrixModelview() -> Matrix;
/// Get internal projection matrix
pub fn rlGetMatrixProjection() -> Matrix;
/// Get internal accumulated transform matrix
pub fn rlGetMatrixTransform() -> Matrix;
/// Get internal projection matrix for stereo render (selected eye)
pub fn rlGetMatrixProjectionStereo(eye: ffi::c_int, ) -> Matrix;
/// Get internal view offset matrix for stereo render (selected eye)
pub fn rlGetMatrixViewOffsetStereo(eye: ffi::c_int, ) -> Matrix;
/// Set a custom projection matrix (replaces internal projection matrix)
pub fn rlSetMatrixProjection(proj: Matrix, );
/// Set a custom modelview matrix (replaces internal modelview matrix)
pub fn rlSetMatrixModelview(view: Matrix, );
/// Set eyes projection matrices for stereo rendering
pub fn rlSetMatrixProjectionStereo(right: Matrix, left: Matrix, );
/// Set eyes view offsets matrices for stereo rendering
pub fn rlSetMatrixViewOffsetStereo(right: Matrix, left: Matrix, );
/// Load and draw a cube
pub fn rlLoadDrawCube();
/// Load and draw a quad
pub fn rlLoadDrawQuad();
}
/// OpenGL version
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum rlGlVersion {
    /// OpenGL 1.1
    RlOpengl11 = 1,
    /// OpenGL 2.1 (GLSL 120)
    RlOpengl21 = 2,
    /// OpenGL 3.3 (GLSL 330)
    RlOpengl33 = 3,
    /// OpenGL 4.3 (using GLSL 330)
    RlOpengl43 = 4,
    /// OpenGL ES 2.0 (GLSL 100)
    RlOpenglEs20 = 5,
    /// OpenGL ES 3.0 (GLSL 300 es)
    RlOpenglEs30 = 6,
}
impl TryFrom<i32> for rlGlVersion {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            1 => Ok(rlGlVersion::RlOpengl11),
            2 => Ok(rlGlVersion::RlOpengl21),
            3 => Ok(rlGlVersion::RlOpengl33),
            4 => Ok(rlGlVersion::RlOpengl43),
            5 => Ok(rlGlVersion::RlOpenglEs20),
            6 => Ok(rlGlVersion::RlOpenglEs30),
            _ => Err(())
        }
    }
}
/// Trace log level
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum rlTraceLogLevel {
    /// Display all logs
    All = 0,
    /// Trace logging, intended for internal use only
    Trace = 1,
    /// Debug logging, used for internal debugging, it should be disabled on release builds
    Debug = 2,
    /// Info logging, used for program execution info
    Info = 3,
    /// Warning logging, used on recoverable failures
    Warning = 4,
    /// Error logging, used on unrecoverable failures
    Error = 5,
    /// Fatal logging, used to abort program: exit(EXIT_FAILURE)
    Fatal = 6,
    /// Disable logging
    None = 7,
}
impl TryFrom<i32> for rlTraceLogLevel {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(rlTraceLogLevel::All),
            1 => Ok(rlTraceLogLevel::Trace),
            2 => Ok(rlTraceLogLevel::Debug),
            3 => Ok(rlTraceLogLevel::Info),
            4 => Ok(rlTraceLogLevel::Warning),
            5 => Ok(rlTraceLogLevel::Error),
            6 => Ok(rlTraceLogLevel::Fatal),
            7 => Ok(rlTraceLogLevel::None),
            _ => Err(())
        }
    }
}
/// Texture pixel formats
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum rlPixelFormat {
    /// 8 bit per pixel (no alpha)
    UncompressedGrayscale = 1,
    /// 8*2 bpp (2 channels)
    UncompressedGrayAlpha = 2,
    /// 16 bpp
    UncompressedR5G6B5 = 3,
    /// 24 bpp
    UncompressedR8G8B8 = 4,
    /// 16 bpp (1 bit alpha)
    UncompressedR5G5B5A1 = 5,
    /// 16 bpp (4 bit alpha)
    UncompressedR4G4B4A4 = 6,
    /// 32 bpp
    UncompressedR8G8B8A8 = 7,
    /// 32 bpp (1 channel - float)
    UncompressedR32 = 8,
    /// 32*3 bpp (3 channels - float)
    UncompressedR32G32B32 = 9,
    /// 32*4 bpp (4 channels - float)
    UncompressedR32G32B32A32 = 10,
    /// 16 bpp (1 channel - half float)
    UncompressedR16 = 11,
    /// 16*3 bpp (3 channels - half float)
    UncompressedR16G16B16 = 12,
    /// 16*4 bpp (4 channels - half float)
    UncompressedR16G16B16A16 = 13,
    /// 4 bpp (no alpha)
    CompressedDxt1Rgb = 14,
    /// 4 bpp (1 bit alpha)
    CompressedDxt1Rgba = 15,
    /// 8 bpp
    CompressedDxt3Rgba = 16,
    /// 8 bpp
    CompressedDxt5Rgba = 17,
    /// 4 bpp
    CompressedEtc1Rgb = 18,
    /// 4 bpp
    CompressedEtc2Rgb = 19,
    /// 8 bpp
    CompressedEtc2EacRgba = 20,
    /// 4 bpp
    CompressedPvrtRgb = 21,
    /// 4 bpp
    CompressedPvrtRgba = 22,
    /// 8 bpp
    CompressedAstc4X4Rgba = 23,
    /// 2 bpp
    CompressedAstc8X8Rgba = 24,
}
impl TryFrom<i32> for rlPixelFormat {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            1 => Ok(rlPixelFormat::UncompressedGrayscale),
            2 => Ok(rlPixelFormat::UncompressedGrayAlpha),
            3 => Ok(rlPixelFormat::UncompressedR5G6B5),
            4 => Ok(rlPixelFormat::UncompressedR8G8B8),
            5 => Ok(rlPixelFormat::UncompressedR5G5B5A1),
            6 => Ok(rlPixelFormat::UncompressedR4G4B4A4),
            7 => Ok(rlPixelFormat::UncompressedR8G8B8A8),
            8 => Ok(rlPixelFormat::UncompressedR32),
            9 => Ok(rlPixelFormat::UncompressedR32G32B32),
            10 => Ok(rlPixelFormat::UncompressedR32G32B32A32),
            11 => Ok(rlPixelFormat::UncompressedR16),
            12 => Ok(rlPixelFormat::UncompressedR16G16B16),
            13 => Ok(rlPixelFormat::UncompressedR16G16B16A16),
            14 => Ok(rlPixelFormat::CompressedDxt1Rgb),
            15 => Ok(rlPixelFormat::CompressedDxt1Rgba),
            16 => Ok(rlPixelFormat::CompressedDxt3Rgba),
            17 => Ok(rlPixelFormat::CompressedDxt5Rgba),
            18 => Ok(rlPixelFormat::CompressedEtc1Rgb),
            19 => Ok(rlPixelFormat::CompressedEtc2Rgb),
            20 => Ok(rlPixelFormat::CompressedEtc2EacRgba),
            21 => Ok(rlPixelFormat::CompressedPvrtRgb),
            22 => Ok(rlPixelFormat::CompressedPvrtRgba),
            23 => Ok(rlPixelFormat::CompressedAstc4X4Rgba),
            24 => Ok(rlPixelFormat::CompressedAstc8X8Rgba),
            _ => Err(())
        }
    }
}
/// Texture parameters: filter mode
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum rlTextureFilter {
    /// No filter, just pixel approximation
    Point = 0,
    /// Linear filtering
    Bilinear = 1,
    /// Trilinear filtering (linear with mipmaps)
    Trilinear = 2,
    /// Anisotropic filtering 4x
    Anisotropic4X = 3,
    /// Anisotropic filtering 8x
    Anisotropic8X = 4,
    /// Anisotropic filtering 16x
    Anisotropic16X = 5,
}
impl TryFrom<i32> for rlTextureFilter {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(rlTextureFilter::Point),
            1 => Ok(rlTextureFilter::Bilinear),
            2 => Ok(rlTextureFilter::Trilinear),
            3 => Ok(rlTextureFilter::Anisotropic4X),
            4 => Ok(rlTextureFilter::Anisotropic8X),
            5 => Ok(rlTextureFilter::Anisotropic16X),
            _ => Err(())
        }
    }
}
/// Color blending modes (pre-defined)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum rlBlendMode {
    /// Blend textures considering alpha (default)
    Alpha = 0,
    /// Blend textures adding colors
    Additive = 1,
    /// Blend textures multiplying colors
    Multiplied = 2,
    /// Blend textures adding colors (alternative)
    AddColors = 3,
    /// Blend textures subtracting colors (alternative)
    SubtractColors = 4,
    /// Blend premultiplied textures considering alpha
    AlphaPremultiply = 5,
    /// Blend textures using custom src/dst factors (use rlSetBlendFactors())
    Custom = 6,
    /// Blend textures using custom src/dst factors (use rlSetBlendFactorsSeparate())
    CustomSeparate = 7,
}
impl TryFrom<i32> for rlBlendMode {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(rlBlendMode::Alpha),
            1 => Ok(rlBlendMode::Additive),
            2 => Ok(rlBlendMode::Multiplied),
            3 => Ok(rlBlendMode::AddColors),
            4 => Ok(rlBlendMode::SubtractColors),
            5 => Ok(rlBlendMode::AlphaPremultiply),
            6 => Ok(rlBlendMode::Custom),
            7 => Ok(rlBlendMode::CustomSeparate),
            _ => Err(())
        }
    }
}
/// Shader location point type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum rlShaderLocationIndex {
    /// Shader location: vertex attribute: position
    VertexPosition = 0,
    /// Shader location: vertex attribute: texcoord01
    VertexTexcoord01 = 1,
    /// Shader location: vertex attribute: texcoord02
    VertexTexcoord02 = 2,
    /// Shader location: vertex attribute: normal
    VertexNormal = 3,
    /// Shader location: vertex attribute: tangent
    VertexTangent = 4,
    /// Shader location: vertex attribute: color
    VertexColor = 5,
    /// Shader location: matrix uniform: model-view-projection
    MatrixMvp = 6,
    /// Shader location: matrix uniform: view (camera transform)
    MatrixView = 7,
    /// Shader location: matrix uniform: projection
    MatrixProjection = 8,
    /// Shader location: matrix uniform: model (transform)
    MatrixModel = 9,
    /// Shader location: matrix uniform: normal
    MatrixNormal = 10,
    /// Shader location: vector uniform: view
    VectorView = 11,
    /// Shader location: vector uniform: diffuse color
    ColorDiffuse = 12,
    /// Shader location: vector uniform: specular color
    ColorSpecular = 13,
    /// Shader location: vector uniform: ambient color
    ColorAmbient = 14,
    /// Shader location: sampler2d texture: albedo (same as: RL_SHADER_LOC_MAP_DIFFUSE)
    MapAlbedo = 15,
    /// Shader location: sampler2d texture: metalness (same as: RL_SHADER_LOC_MAP_SPECULAR)
    MapMetalness = 16,
    /// Shader location: sampler2d texture: normal
    MapNormal = 17,
    /// Shader location: sampler2d texture: roughness
    MapRoughness = 18,
    /// Shader location: sampler2d texture: occlusion
    MapOcclusion = 19,
    /// Shader location: sampler2d texture: emission
    MapEmission = 20,
    /// Shader location: sampler2d texture: height
    MapHeight = 21,
    /// Shader location: samplerCube texture: cubemap
    MapCubemap = 22,
    /// Shader location: samplerCube texture: irradiance
    MapIrradiance = 23,
    /// Shader location: samplerCube texture: prefilter
    MapPrefilter = 24,
    /// Shader location: sampler2d texture: brdf
    MapBrdf = 25,
}
impl TryFrom<i32> for rlShaderLocationIndex {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(rlShaderLocationIndex::VertexPosition),
            1 => Ok(rlShaderLocationIndex::VertexTexcoord01),
            2 => Ok(rlShaderLocationIndex::VertexTexcoord02),
            3 => Ok(rlShaderLocationIndex::VertexNormal),
            4 => Ok(rlShaderLocationIndex::VertexTangent),
            5 => Ok(rlShaderLocationIndex::VertexColor),
            6 => Ok(rlShaderLocationIndex::MatrixMvp),
            7 => Ok(rlShaderLocationIndex::MatrixView),
            8 => Ok(rlShaderLocationIndex::MatrixProjection),
            9 => Ok(rlShaderLocationIndex::MatrixModel),
            10 => Ok(rlShaderLocationIndex::MatrixNormal),
            11 => Ok(rlShaderLocationIndex::VectorView),
            12 => Ok(rlShaderLocationIndex::ColorDiffuse),
            13 => Ok(rlShaderLocationIndex::ColorSpecular),
            14 => Ok(rlShaderLocationIndex::ColorAmbient),
            15 => Ok(rlShaderLocationIndex::MapAlbedo),
            16 => Ok(rlShaderLocationIndex::MapMetalness),
            17 => Ok(rlShaderLocationIndex::MapNormal),
            18 => Ok(rlShaderLocationIndex::MapRoughness),
            19 => Ok(rlShaderLocationIndex::MapOcclusion),
            20 => Ok(rlShaderLocationIndex::MapEmission),
            21 => Ok(rlShaderLocationIndex::MapHeight),
            22 => Ok(rlShaderLocationIndex::MapCubemap),
            23 => Ok(rlShaderLocationIndex::MapIrradiance),
            24 => Ok(rlShaderLocationIndex::MapPrefilter),
            25 => Ok(rlShaderLocationIndex::MapBrdf),
            _ => Err(())
        }
    }
}
/// Shader uniform data type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum rlShaderUniformDataType {
    /// Shader uniform type: float
    Float = 0,
    /// Shader uniform type: vec2 (2 float)
    Vec2 = 1,
    /// Shader uniform type: vec3 (3 float)
    Vec3 = 2,
    /// Shader uniform type: vec4 (4 float)
    Vec4 = 3,
    /// Shader uniform type: int
    Int = 4,
    /// Shader uniform type: ivec2 (2 int)
    Ivec2 = 5,
    /// Shader uniform type: ivec3 (3 int)
    Ivec3 = 6,
    /// Shader uniform type: ivec4 (4 int)
    Ivec4 = 7,
    /// Shader uniform type: sampler2d
    Sampler2D = 8,
}
impl TryFrom<i32> for rlShaderUniformDataType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(rlShaderUniformDataType::Float),
            1 => Ok(rlShaderUniformDataType::Vec2),
            2 => Ok(rlShaderUniformDataType::Vec3),
            3 => Ok(rlShaderUniformDataType::Vec4),
            4 => Ok(rlShaderUniformDataType::Int),
            5 => Ok(rlShaderUniformDataType::Ivec2),
            6 => Ok(rlShaderUniformDataType::Ivec3),
            7 => Ok(rlShaderUniformDataType::Ivec4),
            8 => Ok(rlShaderUniformDataType::Sampler2D),
            _ => Err(())
        }
    }
}
/// Shader attribute data types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum rlShaderAttributeDataType {
    /// Shader attribute type: float
    Float = 0,
    /// Shader attribute type: vec2 (2 float)
    Vec2 = 1,
    /// Shader attribute type: vec3 (3 float)
    Vec3 = 2,
    /// Shader attribute type: vec4 (4 float)
    Vec4 = 3,
}
impl TryFrom<i32> for rlShaderAttributeDataType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(rlShaderAttributeDataType::Float),
            1 => Ok(rlShaderAttributeDataType::Vec2),
            2 => Ok(rlShaderAttributeDataType::Vec3),
            3 => Ok(rlShaderAttributeDataType::Vec4),
            _ => Err(())
        }
    }
}
/// Framebuffer attachment type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum rlFramebufferAttachType {
    /// Framebuffer attachment type: color 0
    ColorChannel0 = 0,
    /// Framebuffer attachment type: color 1
    ColorChannel1 = 1,
    /// Framebuffer attachment type: color 2
    ColorChannel2 = 2,
    /// Framebuffer attachment type: color 3
    ColorChannel3 = 3,
    /// Framebuffer attachment type: color 4
    ColorChannel4 = 4,
    /// Framebuffer attachment type: color 5
    ColorChannel5 = 5,
    /// Framebuffer attachment type: color 6
    ColorChannel6 = 6,
    /// Framebuffer attachment type: color 7
    ColorChannel7 = 7,
    /// Framebuffer attachment type: depth
    Depth = 100,
    /// Framebuffer attachment type: stencil
    Stencil = 200,
}
impl TryFrom<i32> for rlFramebufferAttachType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(rlFramebufferAttachType::ColorChannel0),
            1 => Ok(rlFramebufferAttachType::ColorChannel1),
            2 => Ok(rlFramebufferAttachType::ColorChannel2),
            3 => Ok(rlFramebufferAttachType::ColorChannel3),
            4 => Ok(rlFramebufferAttachType::ColorChannel4),
            5 => Ok(rlFramebufferAttachType::ColorChannel5),
            6 => Ok(rlFramebufferAttachType::ColorChannel6),
            7 => Ok(rlFramebufferAttachType::ColorChannel7),
            100 => Ok(rlFramebufferAttachType::Depth),
            200 => Ok(rlFramebufferAttachType::Stencil),
            _ => Err(())
        }
    }
}
/// Framebuffer texture attachment type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum rlFramebufferAttachTextureType {
    /// Framebuffer texture attachment type: cubemap, +X side
    CubemapPositiveX = 0,
    /// Framebuffer texture attachment type: cubemap, -X side
    CubemapNegativeX = 1,
    /// Framebuffer texture attachment type: cubemap, +Y side
    CubemapPositiveY = 2,
    /// Framebuffer texture attachment type: cubemap, -Y side
    CubemapNegativeY = 3,
    /// Framebuffer texture attachment type: cubemap, +Z side
    CubemapPositiveZ = 4,
    /// Framebuffer texture attachment type: cubemap, -Z side
    CubemapNegativeZ = 5,
    /// Framebuffer texture attachment type: texture2d
    Texture2D = 100,
    /// Framebuffer texture attachment type: renderbuffer
    Renderbuffer = 200,
}
impl TryFrom<i32> for rlFramebufferAttachTextureType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(rlFramebufferAttachTextureType::CubemapPositiveX),
            1 => Ok(rlFramebufferAttachTextureType::CubemapNegativeX),
            2 => Ok(rlFramebufferAttachTextureType::CubemapPositiveY),
            3 => Ok(rlFramebufferAttachTextureType::CubemapNegativeY),
            4 => Ok(rlFramebufferAttachTextureType::CubemapPositiveZ),
            5 => Ok(rlFramebufferAttachTextureType::CubemapNegativeZ),
            100 => Ok(rlFramebufferAttachTextureType::Texture2D),
            200 => Ok(rlFramebufferAttachTextureType::Renderbuffer),
            _ => Err(())
        }
    }
}
/// Face culling mode
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum rlCullMode {
    Front = 0,
    Back = 1,
}
impl TryFrom<i32> for rlCullMode {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(rlCullMode::Front),
            1 => Ok(rlCullMode::Back),
            _ => Err(())
        }
    }
}
use crate::prelude::Matrix;
