#![allow(non_snake_case, non_camel_case_types, unused)]
use std::ffi;

pub const RAYLIB_VERSION_MAJOR: i32 = 5;
pub const RAYLIB_VERSION_MINOR: i32 = 1;
pub const RAYLIB_VERSION_PATCH: i32 = 0;
pub const RAYLIB_VERSION: &str = "5.1-dev";
/// Vector2, 2 components
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector2 {
    /// Vector x component
    pub x: ffi::c_float,
    /// Vector y component
    pub y: ffi::c_float,
}
/// Vector3, 3 components
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector3 {
    /// Vector x component
    pub x: ffi::c_float,
    /// Vector y component
    pub y: ffi::c_float,
    /// Vector z component
    pub z: ffi::c_float,
}
/// Vector4, 4 components
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector4 {
    /// Vector x component
    pub x: ffi::c_float,
    /// Vector y component
    pub y: ffi::c_float,
    /// Vector z component
    pub z: ffi::c_float,
    /// Vector w component
    pub w: ffi::c_float,
}
/// Matrix, 4x4 components, column major, OpenGL style, right-handed
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    /// Matrix first row (4 components)
    pub m0: ffi::c_float,
    /// Matrix first row (4 components)
    pub m4: ffi::c_float,
    /// Matrix first row (4 components)
    pub m8: ffi::c_float,
    /// Matrix first row (4 components)
    pub m12: ffi::c_float,
    /// Matrix second row (4 components)
    pub m1: ffi::c_float,
    /// Matrix second row (4 components)
    pub m5: ffi::c_float,
    /// Matrix second row (4 components)
    pub m9: ffi::c_float,
    /// Matrix second row (4 components)
    pub m13: ffi::c_float,
    /// Matrix third row (4 components)
    pub m2: ffi::c_float,
    /// Matrix third row (4 components)
    pub m6: ffi::c_float,
    /// Matrix third row (4 components)
    pub m10: ffi::c_float,
    /// Matrix third row (4 components)
    pub m14: ffi::c_float,
    /// Matrix fourth row (4 components)
    pub m3: ffi::c_float,
    /// Matrix fourth row (4 components)
    pub m7: ffi::c_float,
    /// Matrix fourth row (4 components)
    pub m11: ffi::c_float,
    /// Matrix fourth row (4 components)
    pub m15: ffi::c_float,
}
/// Color, 4 components, R8G8B8A8 (32bit)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Color {
    /// Color red value
    pub r: ffi::c_uchar,
    /// Color green value
    pub g: ffi::c_uchar,
    /// Color blue value
    pub b: ffi::c_uchar,
    /// Color alpha value
    pub a: ffi::c_uchar,
}
/// Rectangle, 4 components
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    /// Rectangle top-left corner position x
    pub x: ffi::c_float,
    /// Rectangle top-left corner position y
    pub y: ffi::c_float,
    /// Rectangle width
    pub width: ffi::c_float,
    /// Rectangle height
    pub height: ffi::c_float,
}
/// Image, pixel data stored in CPU memory (RAM)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Image {
    /// Image raw data
    pub data: *mut ffi::c_void,
    /// Image base width
    pub width: ffi::c_int,
    /// Image base height
    pub height: ffi::c_int,
    /// Mipmap levels, 1 by default
    pub mipmaps: ffi::c_int,
    /// Data format (PixelFormat type)
    pub format: ffi::c_int,
}
/// Texture, tex data stored in GPU memory (VRAM)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Texture {
    /// OpenGL texture id
    pub id: ffi::c_uint,
    /// Texture base width
    pub width: ffi::c_int,
    /// Texture base height
    pub height: ffi::c_int,
    /// Mipmap levels, 1 by default
    pub mipmaps: ffi::c_int,
    /// Data format (PixelFormat type)
    pub format: ffi::c_int,
}
/// RenderTexture, fbo for texture rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderTexture {
    /// OpenGL framebuffer object id
    pub id: ffi::c_uint,
    /// Color buffer attachment texture
    pub texture: Texture,
    /// Depth buffer attachment texture
    pub depth: Texture,
}
/// NPatchInfo, n-patch layout info
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NPatchInfo {
    /// Texture source rectangle
    pub source: Rectangle,
    /// Left border offset
    pub left: ffi::c_int,
    /// Top border offset
    pub top: ffi::c_int,
    /// Right border offset
    pub right: ffi::c_int,
    /// Bottom border offset
    pub bottom: ffi::c_int,
    /// Layout of the n-patch: 3x3, 1x3 or 3x1
    pub layout: ffi::c_int,
}
/// GlyphInfo, font characters glyphs info
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlyphInfo {
    /// Character value (Unicode)
    pub value: ffi::c_int,
    /// Character offset X when drawing
    pub offsetX: ffi::c_int,
    /// Character offset Y when drawing
    pub offsetY: ffi::c_int,
    /// Character advance position X
    pub advanceX: ffi::c_int,
    /// Character image data
    pub image: Image,
}
/// Font, font texture and GlyphInfo array data
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Font {
    /// Base size (default chars height)
    pub baseSize: ffi::c_int,
    /// Number of glyph characters
    pub glyphCount: ffi::c_int,
    /// Padding around the glyph characters
    pub glyphPadding: ffi::c_int,
    /// Texture atlas containing the glyphs
    pub texture: Texture2D,
    /// Rectangles in texture for the glyphs
    pub recs: *mut Rectangle,
    /// Glyphs info data
    pub glyphs: *mut GlyphInfo,
}
/// Camera, defines position/orientation in 3d space
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera3D {
    /// Camera position
    pub position: Vector3,
    /// Camera target it looks-at
    pub target: Vector3,
    /// Camera up vector (rotation over its axis)
    pub up: Vector3,
    /// Camera field-of-view aperture in Y (degrees) in perspective, used as near plane width in orthographic
    pub fovy: ffi::c_float,
    /// Camera projection: CAMERA_PERSPECTIVE or CAMERA_ORTHOGRAPHIC
    pub projection: ffi::c_int,
}
/// Camera2D, defines position/orientation in 2d space
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera2D {
    /// Camera offset (displacement from target)
    pub offset: Vector2,
    /// Camera target (rotation and zoom origin)
    pub target: Vector2,
    /// Camera rotation in degrees
    pub rotation: ffi::c_float,
    /// Camera zoom (scaling), should be 1.0f by default
    pub zoom: ffi::c_float,
}
/// Mesh, vertex data and vao/vbo
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mesh {
    /// Number of vertices stored in arrays
    pub vertexCount: ffi::c_int,
    /// Number of triangles stored (indexed or not)
    pub triangleCount: ffi::c_int,
    /// Vertex position (XYZ - 3 components per vertex) (shader-location = 0)
    pub vertices: *mut ffi::c_float,
    /// Vertex texture coordinates (UV - 2 components per vertex) (shader-location = 1)
    pub texcoords: *mut ffi::c_float,
    /// Vertex texture second coordinates (UV - 2 components per vertex) (shader-location = 5)
    pub texcoords2: *mut ffi::c_float,
    /// Vertex normals (XYZ - 3 components per vertex) (shader-location = 2)
    pub normals: *mut ffi::c_float,
    /// Vertex tangents (XYZW - 4 components per vertex) (shader-location = 4)
    pub tangents: *mut ffi::c_float,
    /// Vertex colors (RGBA - 4 components per vertex) (shader-location = 3)
    pub colors: *mut ffi::c_uchar,
    /// Vertex indices (in case vertex data comes indexed)
    pub indices: *mut ffi::c_ushort,
    /// Animated vertex positions (after bones transformations)
    pub animVertices: *mut ffi::c_float,
    /// Animated normals (after bones transformations)
    pub animNormals: *mut ffi::c_float,
    /// Vertex bone ids, max 255 bone ids, up to 4 bones influence by vertex (skinning)
    pub boneIds: *mut ffi::c_uchar,
    /// Vertex bone weight, up to 4 bones influence by vertex (skinning)
    pub boneWeights: *mut ffi::c_float,
    /// OpenGL Vertex Array Object id
    pub vaoId: ffi::c_uint,
    /// OpenGL Vertex Buffer Objects id (default vertex data)
    pub vboId: *mut ffi::c_uint,
}
/// Shader
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shader {
    /// Shader program id
    pub id: ffi::c_uint,
    /// Shader locations array (RL_MAX_SHADER_LOCATIONS)
    pub locs: *mut ffi::c_int,
}
/// MaterialMap
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialMap {
    /// Material map texture
    pub texture: Texture2D,
    /// Material map color
    pub color: Color,
    /// Material map value
    pub value: ffi::c_float,
}
/// Material, includes shader and maps
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    /// Material shader
    pub shader: Shader,
    /// Material maps array (MAX_MATERIAL_MAPS)
    pub maps: *mut MaterialMap,
    /// Material generic parameters (if required)
    pub params: [ffi::c_float; 4],
}
/// Transform, vertex transformation data
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    /// Translation
    pub translation: Vector3,
    /// Rotation
    pub rotation: Quaternion,
    /// Scale
    pub scale: Vector3,
}
/// Bone, skeletal animation bone
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoneInfo {
    /// Bone name
    pub name: [ffi::c_char; 32],
    /// Bone parent
    pub parent: ffi::c_int,
}
/// Model, meshes, materials and animation data
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Model {
    /// Local transform matrix
    pub transform: Matrix,
    /// Number of meshes
    pub meshCount: ffi::c_int,
    /// Number of materials
    pub materialCount: ffi::c_int,
    /// Meshes array
    pub meshes: *mut Mesh,
    /// Materials array
    pub materials: *mut Material,
    /// Mesh material number
    pub meshMaterial: *mut ffi::c_int,
    /// Number of bones
    pub boneCount: ffi::c_int,
    /// Bones information (skeleton)
    pub bones: *mut BoneInfo,
    /// Bones base transformation (pose)
    pub bindPose: *mut Transform,
}
/// ModelAnimation
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModelAnimation {
    /// Number of bones
    pub boneCount: ffi::c_int,
    /// Number of animation frames
    pub frameCount: ffi::c_int,
    /// Bones information (skeleton)
    pub bones: *mut BoneInfo,
    /// Poses array by frame
    pub framePoses: *mut *mut Transform,
    /// Animation name
    pub name: [ffi::c_char; 32],
}
/// Ray, ray for raycasting
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    /// Ray position (origin)
    pub position: Vector3,
    /// Ray direction
    pub direction: Vector3,
}
/// RayCollision, ray hit information
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayCollision {
    /// Did the ray hit something?
    pub hit: bool,
    /// Distance to the nearest hit
    pub distance: ffi::c_float,
    /// Point of the nearest hit
    pub point: Vector3,
    /// Surface normal of hit
    pub normal: Vector3,
}
/// BoundingBox
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    /// Minimum vertex box-corner
    pub min: Vector3,
    /// Maximum vertex box-corner
    pub max: Vector3,
}
/// Wave, audio wave data
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wave {
    /// Total number of frames (considering channels)
    pub frameCount: ffi::c_uint,
    /// Frequency (samples per second)
    pub sampleRate: ffi::c_uint,
    /// Bit depth (bits per sample): 8, 16, 32 (24 not supported)
    pub sampleSize: ffi::c_uint,
    /// Number of channels (1-mono, 2-stereo, ...)
    pub channels: ffi::c_uint,
    /// Buffer data pointer
    pub data: *mut ffi::c_void,
}
type rAudioBuffer = ffi::c_void;
type rAudioProcessor = ffi::c_void;

/// AudioStream, custom audio stream
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioStream {
    /// Pointer to internal data used by the audio system
    pub buffer: *mut rAudioBuffer,
    /// Pointer to internal data processor, useful for audio effects
    pub processor: *mut rAudioProcessor,
    /// Frequency (samples per second)
    pub sampleRate: ffi::c_uint,
    /// Bit depth (bits per sample): 8, 16, 32 (24 not supported)
    pub sampleSize: ffi::c_uint,
    /// Number of channels (1-mono, 2-stereo, ...)
    pub channels: ffi::c_uint,
}
/// Sound
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sound {
    /// Audio stream
    pub stream: AudioStream,
    /// Total number of frames (considering channels)
    pub frameCount: ffi::c_uint,
}
/// Music, audio stream, anything longer than ~10 seconds should be streamed
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Music {
    /// Audio stream
    pub stream: AudioStream,
    /// Total number of frames (considering channels)
    pub frameCount: ffi::c_uint,
    /// Music looping enable
    pub looping: bool,
    /// Type of music context (audio filetype)
    pub ctxType: ffi::c_int,
    /// Audio context data, depends on type
    pub ctxData: *mut ffi::c_void,
}
/// VrDeviceInfo, Head-Mounted-Display device parameters
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrDeviceInfo {
    /// Horizontal resolution in pixels
    pub hResolution: ffi::c_int,
    /// Vertical resolution in pixels
    pub vResolution: ffi::c_int,
    /// Horizontal size in meters
    pub hScreenSize: ffi::c_float,
    /// Vertical size in meters
    pub vScreenSize: ffi::c_float,
    /// Distance between eye and display in meters
    pub eyeToScreenDistance: ffi::c_float,
    /// Lens separation distance in meters
    pub lensSeparationDistance: ffi::c_float,
    /// IPD (distance between pupils) in meters
    pub interpupillaryDistance: ffi::c_float,
    /// Lens distortion constant parameters
    pub lensDistortionValues: [ffi::c_float; 4],
    /// Chromatic aberration correction parameters
    pub chromaAbCorrection: [ffi::c_float; 4],
}
/// VrStereoConfig, VR stereo rendering configuration for simulator
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrStereoConfig {
    /// VR projection matrices (per eye)
    pub projection: [Matrix; 2],
    /// VR view offset matrices (per eye)
    pub viewOffset: [Matrix; 2],
    /// VR left lens center
    pub leftLensCenter: [ffi::c_float; 2],
    /// VR right lens center
    pub rightLensCenter: [ffi::c_float; 2],
    /// VR left screen center
    pub leftScreenCenter: [ffi::c_float; 2],
    /// VR right screen center
    pub rightScreenCenter: [ffi::c_float; 2],
    /// VR distortion scale
    pub scale: [ffi::c_float; 2],
    /// VR distortion scale in
    pub scaleIn: [ffi::c_float; 2],
}
/// File path list
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FilePathList {
    /// Filepaths max entries
    pub capacity: ffi::c_uint,
    /// Filepaths entries count
    pub count: ffi::c_uint,
    /// Filepaths entries
    pub paths: *mut *mut ffi::c_char,
}
/// Automation event
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutomationEvent {
    /// Event frame
    pub frame: ffi::c_uint,
    /// Event type (AutomationEventType)
    pub r#type: ffi::c_uint,
    /// Event parameters (if required)
    pub params: [ffi::c_int; 4],
}
/// Automation event list
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutomationEventList {
    /// Events max entries (MAX_AUTOMATION_EVENTS)
    pub capacity: ffi::c_uint,
    /// Events entries count
    pub count: ffi::c_uint,
    /// Events entries
    pub events: *mut AutomationEvent,
}

/// Quaternion, 4 float components
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    /// Imaginary `i` part of the quaternion
    pub x: f32,
    /// Imaginary `j` part of the quaternion
    pub y: f32,
    /// Imaginary `k` part of the quaternion
    pub z: f32,
    /// Real part of the quaternion
    pub w: f32
}
/// Texture2D, same as Texture
pub type Texture2D = Texture;
/// TextureCubemap, same as Texture
pub type TextureCubemap = Texture;
/// RenderTexture2D, same as RenderTexture
pub type RenderTexture2D = RenderTexture;
/// Camera type fallback, defaults to Camera3D
pub type Camera = Camera3D;

/// Logging: Redirect trace log messages
pub type TraceLogCallback = extern fn(ffi::c_int, *const ffi::c_char, va_list::VaList, );
/// FileIO: Load binary data
pub type LoadFileDataCallback = extern fn(*const ffi::c_char, *mut ffi::c_int, ) -> *mut ffi::c_uchar;
/// FileIO: Save binary data
pub type SaveFileDataCallback = extern fn(*const ffi::c_char, *mut ffi::c_void, ffi::c_int, ) -> bool;
/// FileIO: Load text data
pub type LoadFileTextCallback = extern fn(*const ffi::c_char, ) -> *mut ffi::c_char;
/// FileIO: Save text data
pub type SaveFileTextCallback = extern fn(*const ffi::c_char, *mut ffi::c_char, ) -> bool;
pub type AudioCallback = extern fn(*mut ffi::c_void, ffi::c_uint, );

#[link(name = "raylib", kind = "static")]
extern "C" {
/// Initialize window and OpenGL context
pub fn InitWindow(width: ffi::c_int, height: ffi::c_int, title: *const ffi::c_char, );
/// Close window and unload OpenGL context
pub fn CloseWindow();
/// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
pub fn WindowShouldClose() -> bool;
/// Check if window has been initialized successfully
pub fn IsWindowReady() -> bool;
/// Check if window is currently fullscreen
pub fn IsWindowFullscreen() -> bool;
/// Check if window is currently hidden (only PLATFORM_DESKTOP)
pub fn IsWindowHidden() -> bool;
/// Check if window is currently minimized (only PLATFORM_DESKTOP)
pub fn IsWindowMinimized() -> bool;
/// Check if window is currently maximized (only PLATFORM_DESKTOP)
pub fn IsWindowMaximized() -> bool;
/// Check if window is currently focused (only PLATFORM_DESKTOP)
pub fn IsWindowFocused() -> bool;
/// Check if window has been resized last frame
pub fn IsWindowResized() -> bool;
/// Check if one specific window flag is enabled
pub fn IsWindowState(flag: ffi::c_uint, ) -> bool;
/// Set window configuration state using flags (only PLATFORM_DESKTOP)
pub fn SetWindowState(flags: ffi::c_uint, );
/// Clear window configuration state flags
pub fn ClearWindowState(flags: ffi::c_uint, );
/// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP)
pub fn ToggleFullscreen();
/// Toggle window state: borderless windowed (only PLATFORM_DESKTOP)
pub fn ToggleBorderlessWindowed();
/// Set window state: maximized, if resizable (only PLATFORM_DESKTOP)
pub fn MaximizeWindow();
/// Set window state: minimized, if resizable (only PLATFORM_DESKTOP)
pub fn MinimizeWindow();
/// Set window state: not minimized/maximized (only PLATFORM_DESKTOP)
pub fn RestoreWindow();
/// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
pub fn SetWindowIcon(image: Image, );
/// Set icon for window (multiple images, RGBA 32bit, only PLATFORM_DESKTOP)
pub fn SetWindowIcons(images: *mut Image, count: ffi::c_int, );
/// Set title for window (only PLATFORM_DESKTOP and PLATFORM_WEB)
pub fn SetWindowTitle(title: *const ffi::c_char, );
/// Set window position on screen (only PLATFORM_DESKTOP)
pub fn SetWindowPosition(x: ffi::c_int, y: ffi::c_int, );
/// Set monitor for the current window
pub fn SetWindowMonitor(monitor: ffi::c_int, );
/// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
pub fn SetWindowMinSize(width: ffi::c_int, height: ffi::c_int, );
/// Set window maximum dimensions (for FLAG_WINDOW_RESIZABLE)
pub fn SetWindowMaxSize(width: ffi::c_int, height: ffi::c_int, );
/// Set window dimensions
pub fn SetWindowSize(width: ffi::c_int, height: ffi::c_int, );
/// Set window opacity \[0.0f..1.0f\] (only PLATFORM_DESKTOP)
pub fn SetWindowOpacity(opacity: ffi::c_float, );
/// Set window focused (only PLATFORM_DESKTOP)
pub fn SetWindowFocused();
/// Get native window handle
pub fn GetWindowHandle() -> *mut ffi::c_void;
/// Get current screen width
pub fn GetScreenWidth() -> ffi::c_int;
/// Get current screen height
pub fn GetScreenHeight() -> ffi::c_int;
/// Get current render width (it considers HiDPI)
pub fn GetRenderWidth() -> ffi::c_int;
/// Get current render height (it considers HiDPI)
pub fn GetRenderHeight() -> ffi::c_int;
/// Get number of connected monitors
pub fn GetMonitorCount() -> ffi::c_int;
/// Get current connected monitor
pub fn GetCurrentMonitor() -> ffi::c_int;
/// Get specified monitor position
pub fn GetMonitorPosition(monitor: ffi::c_int, ) -> Vector2;
/// Get specified monitor width (current video mode used by monitor)
pub fn GetMonitorWidth(monitor: ffi::c_int, ) -> ffi::c_int;
/// Get specified monitor height (current video mode used by monitor)
pub fn GetMonitorHeight(monitor: ffi::c_int, ) -> ffi::c_int;
/// Get specified monitor physical width in millimetres
pub fn GetMonitorPhysicalWidth(monitor: ffi::c_int, ) -> ffi::c_int;
/// Get specified monitor physical height in millimetres
pub fn GetMonitorPhysicalHeight(monitor: ffi::c_int, ) -> ffi::c_int;
/// Get specified monitor refresh rate
pub fn GetMonitorRefreshRate(monitor: ffi::c_int, ) -> ffi::c_int;
/// Get window position XY on monitor
pub fn GetWindowPosition() -> Vector2;
/// Get window scale DPI factor
pub fn GetWindowScaleDPI() -> Vector2;
/// Get the human-readable, UTF-8 encoded name of the specified monitor
pub fn GetMonitorName(monitor: ffi::c_int, ) -> *const ffi::c_char;
/// Set clipboard text content
pub fn SetClipboardText(text: *const ffi::c_char, );
/// Get clipboard text content
pub fn GetClipboardText() -> *const ffi::c_char;
/// Enable waiting for events on EndDrawing(), no automatic event polling
pub fn EnableEventWaiting();
/// Disable waiting for events on EndDrawing(), automatic events polling
pub fn DisableEventWaiting();
/// Shows cursor
pub fn ShowCursor();
/// Hides cursor
pub fn HideCursor();
/// Check if cursor is not visible
pub fn IsCursorHidden() -> bool;
/// Enables cursor (unlock cursor)
pub fn EnableCursor();
/// Disables cursor (lock cursor)
pub fn DisableCursor();
/// Check if cursor is on the screen
pub fn IsCursorOnScreen() -> bool;
/// Set background color (framebuffer clear color)
pub fn ClearBackground(color: Color, );
/// Setup canvas (framebuffer) to start drawing
pub fn BeginDrawing();
/// End canvas drawing and swap buffers (double buffering)
pub fn EndDrawing();
/// Begin 2D mode with custom camera (2D)
pub fn BeginMode2D(camera: Camera2D, );
/// Ends 2D mode with custom camera
pub fn EndMode2D();
/// Begin 3D mode with custom camera (3D)
pub fn BeginMode3D(camera: Camera3D, );
/// Ends 3D mode and returns to default 2D orthographic mode
pub fn EndMode3D();
/// Begin drawing to render texture
pub fn BeginTextureMode(target: RenderTexture2D, );
/// Ends drawing to render texture
pub fn EndTextureMode();
/// Begin custom shader drawing
pub fn BeginShaderMode(shader: Shader, );
/// End custom shader drawing (use default shader)
pub fn EndShaderMode();
/// Begin blending mode (alpha, additive, multiplied, subtract, custom)
pub fn BeginBlendMode(mode: ffi::c_int, );
/// End blending mode (reset to default: alpha blending)
pub fn EndBlendMode();
/// Begin scissor mode (define screen area for following drawing)
pub fn BeginScissorMode(x: ffi::c_int, y: ffi::c_int, width: ffi::c_int, height: ffi::c_int, );
/// End scissor mode
pub fn EndScissorMode();
/// Begin stereo rendering (requires VR simulator)
pub fn BeginVrStereoMode(config: VrStereoConfig, );
/// End stereo rendering (requires VR simulator)
pub fn EndVrStereoMode();
/// Load VR stereo config for VR simulator device parameters
pub fn LoadVrStereoConfig(device: VrDeviceInfo, ) -> VrStereoConfig;
/// Unload VR stereo config
pub fn UnloadVrStereoConfig(config: VrStereoConfig, );
/// Load shader from files and bind default locations
pub fn LoadShader(vsFileName: *const ffi::c_char, fsFileName: *const ffi::c_char, ) -> Shader;
/// Load shader from code strings and bind default locations
pub fn LoadShaderFromMemory(vsCode: *const ffi::c_char, fsCode: *const ffi::c_char, ) -> Shader;
/// Check if a shader is ready
pub fn IsShaderReady(shader: Shader, ) -> bool;
/// Get shader uniform location
pub fn GetShaderLocation(shader: Shader, uniformName: *const ffi::c_char, ) -> ffi::c_int;
/// Get shader attribute location
pub fn GetShaderLocationAttrib(shader: Shader, attribName: *const ffi::c_char, ) -> ffi::c_int;
/// Set shader uniform value
pub fn SetShaderValue(shader: Shader, locIndex: ffi::c_int, value: *const ffi::c_void, uniformType: ffi::c_int, );
/// Set shader uniform value vector
pub fn SetShaderValueV(shader: Shader, locIndex: ffi::c_int, value: *const ffi::c_void, uniformType: ffi::c_int, count: ffi::c_int, );
/// Set shader uniform value (matrix 4x4)
pub fn SetShaderValueMatrix(shader: Shader, locIndex: ffi::c_int, mat: Matrix, );
/// Set shader uniform value for texture (sampler2d)
pub fn SetShaderValueTexture(shader: Shader, locIndex: ffi::c_int, texture: Texture2D, );
/// Unload shader from GPU memory (VRAM)
pub fn UnloadShader(shader: Shader, );
/// Get a ray trace from screen position (i.e mouse)
pub fn GetScreenToWorldRay(position: Vector2, camera: Camera, ) -> Ray;
/// Get a ray trace from screen position (i.e mouse) in a viewport
pub fn GetScreenToWorldRayEx(position: Vector2, camera: Camera, width: ffi::c_int, height: ffi::c_int, ) -> Ray;
/// Get the screen space position for a 3d world space position
pub fn GetWorldToScreen(position: Vector3, camera: Camera, ) -> Vector2;
/// Get size position for a 3d world space position
pub fn GetWorldToScreenEx(position: Vector3, camera: Camera, width: ffi::c_int, height: ffi::c_int, ) -> Vector2;
/// Get the screen space position for a 2d camera world space position
pub fn GetWorldToScreen2D(position: Vector2, camera: Camera2D, ) -> Vector2;
/// Get the world space position for a 2d camera screen space position
pub fn GetScreenToWorld2D(position: Vector2, camera: Camera2D, ) -> Vector2;
/// Get camera transform matrix (view matrix)
pub fn GetCameraMatrix(camera: Camera, ) -> Matrix;
/// Get camera 2d transform matrix
pub fn GetCameraMatrix2D(camera: Camera2D, ) -> Matrix;
/// Set target FPS (maximum)
pub fn SetTargetFPS(fps: ffi::c_int, );
/// Get time in seconds for last frame drawn (delta time)
pub fn GetFrameTime() -> ffi::c_float;
/// Get elapsed time in seconds since InitWindow()
pub fn GetTime() -> ffi::c_double;
/// Get current FPS
pub fn GetFPS() -> ffi::c_int;
/// Swap back buffer with front buffer (screen drawing)
pub fn SwapScreenBuffer();
/// Register all input events
pub fn PollInputEvents();
/// Wait for some time (halt program execution)
pub fn WaitTime(seconds: ffi::c_double, );
/// Set the seed for the random number generator
pub fn SetRandomSeed(seed: ffi::c_uint, );
/// Get a random value between min and max (both included)
pub fn GetRandomValue(min: ffi::c_int, max: ffi::c_int, ) -> ffi::c_int;
/// Load random values sequence, no values repeated
pub fn LoadRandomSequence(count: ffi::c_uint, min: ffi::c_int, max: ffi::c_int, ) -> *mut ffi::c_int;
/// Unload random values sequence
pub fn UnloadRandomSequence(sequence: *mut ffi::c_int, );
/// Takes a screenshot of current screen (filename extension defines format)
pub fn TakeScreenshot(fileName: *const ffi::c_char, );
/// Setup init configuration flags (view FLAGS)
pub fn SetConfigFlags(flags: ffi::c_uint, );
/// Open URL with default system browser (if available)
pub fn OpenURL(url: *const ffi::c_char, );
/// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...)
pub fn TraceLog(logLevel: ffi::c_int, text: *const ffi::c_char, args: ..., );
/// Set the current threshold (minimum) log level
pub fn SetTraceLogLevel(logLevel: ffi::c_int, );
/// Internal memory allocator
pub fn MemAlloc(size: ffi::c_uint, ) -> *mut ffi::c_void;
/// Internal memory reallocator
pub fn MemRealloc(ptr: *mut ffi::c_void, size: ffi::c_uint, ) -> *mut ffi::c_void;
/// Internal memory free
pub fn MemFree(ptr: *mut ffi::c_void, );
/// Set custom trace log
pub fn SetTraceLogCallback(callback: TraceLogCallback, );
/// Set custom file binary data loader
pub fn SetLoadFileDataCallback(callback: LoadFileDataCallback, );
/// Set custom file binary data saver
pub fn SetSaveFileDataCallback(callback: SaveFileDataCallback, );
/// Set custom file text data loader
pub fn SetLoadFileTextCallback(callback: LoadFileTextCallback, );
/// Set custom file text data saver
pub fn SetSaveFileTextCallback(callback: SaveFileTextCallback, );
/// Load file data as byte array (read)
pub fn LoadFileData(fileName: *const ffi::c_char, dataSize: *mut ffi::c_int, ) -> *mut ffi::c_uchar;
/// Unload file data allocated by LoadFileData()
pub fn UnloadFileData(data: *mut ffi::c_uchar, );
/// Save data to file from byte array (write), returns true on success
pub fn SaveFileData(fileName: *const ffi::c_char, data: *mut ffi::c_void, dataSize: ffi::c_int, ) -> bool;
/// Export data to code (.h), returns true on success
pub fn ExportDataAsCode(data: *const ffi::c_uchar, dataSize: ffi::c_int, fileName: *const ffi::c_char, ) -> bool;
/// Load text data from file (read), returns a '\0' terminated string
pub fn LoadFileText(fileName: *const ffi::c_char, ) -> *mut ffi::c_char;
/// Unload file text data allocated by LoadFileText()
pub fn UnloadFileText(text: *mut ffi::c_char, );
/// Save text data to file (write), string must be '\0' terminated, returns true on success
pub fn SaveFileText(fileName: *const ffi::c_char, text: *mut ffi::c_char, ) -> bool;
/// Check if file exists
pub fn FileExists(fileName: *const ffi::c_char, ) -> bool;
/// Check if a directory path exists
pub fn DirectoryExists(dirPath: *const ffi::c_char, ) -> bool;
/// Check file extension (including point: .png, .wav)
pub fn IsFileExtension(fileName: *const ffi::c_char, ext: *const ffi::c_char, ) -> bool;
/// Get file length in bytes (NOTE: GetFileSize() conflicts with windows.h)
pub fn GetFileLength(fileName: *const ffi::c_char, ) -> ffi::c_int;
/// Get pointer to extension for a filename string (includes dot: '.png')
pub fn GetFileExtension(fileName: *const ffi::c_char, ) -> *const ffi::c_char;
/// Get pointer to filename for a path string
pub fn GetFileName(filePath: *const ffi::c_char, ) -> *const ffi::c_char;
/// Get filename string without extension (uses static string)
pub fn GetFileNameWithoutExt(filePath: *const ffi::c_char, ) -> *const ffi::c_char;
/// Get full path for a given fileName with path (uses static string)
pub fn GetDirectoryPath(filePath: *const ffi::c_char, ) -> *const ffi::c_char;
/// Get previous directory path for a given path (uses static string)
pub fn GetPrevDirectoryPath(dirPath: *const ffi::c_char, ) -> *const ffi::c_char;
/// Get current working directory (uses static string)
pub fn GetWorkingDirectory() -> *const ffi::c_char;
/// Get the directory of the running application (uses static string)
pub fn GetApplicationDirectory() -> *const ffi::c_char;
/// Change working directory, return true on success
pub fn ChangeDirectory(dir: *const ffi::c_char, ) -> bool;
/// Check if a given path is a file or a directory
pub fn IsPathFile(path: *const ffi::c_char, ) -> bool;
/// Check if fileName is valid for the platform/OS
pub fn IsFileNameValid(fileName: *const ffi::c_char, ) -> bool;
/// Load directory filepaths
pub fn LoadDirectoryFiles(dirPath: *const ffi::c_char, ) -> FilePathList;
/// Load directory filepaths with extension filtering and recursive directory scan
pub fn LoadDirectoryFilesEx(basePath: *const ffi::c_char, filter: *const ffi::c_char, scanSubdirs: bool, ) -> FilePathList;
/// Unload filepaths
pub fn UnloadDirectoryFiles(files: FilePathList, );
/// Check if a file has been dropped into window
pub fn IsFileDropped() -> bool;
/// Load dropped filepaths
pub fn LoadDroppedFiles() -> FilePathList;
/// Unload dropped filepaths
pub fn UnloadDroppedFiles(files: FilePathList, );
/// Get file modification time (last write time)
pub fn GetFileModTime(fileName: *const ffi::c_char, ) -> ffi::c_long;
/// Compress data (DEFLATE algorithm), memory must be MemFree()
pub fn CompressData(data: *const ffi::c_uchar, dataSize: ffi::c_int, compDataSize: *mut ffi::c_int, ) -> *mut ffi::c_uchar;
/// Decompress data (DEFLATE algorithm), memory must be MemFree()
pub fn DecompressData(compData: *const ffi::c_uchar, compDataSize: ffi::c_int, dataSize: *mut ffi::c_int, ) -> *mut ffi::c_uchar;
/// Encode data to Base64 string, memory must be MemFree()
pub fn EncodeDataBase64(data: *const ffi::c_uchar, dataSize: ffi::c_int, outputSize: *mut ffi::c_int, ) -> *mut ffi::c_char;
/// Decode Base64 string data, memory must be MemFree()
pub fn DecodeDataBase64(data: *const ffi::c_uchar, outputSize: *mut ffi::c_int, ) -> *mut ffi::c_uchar;
/// Load automation events list from file, NULL for empty list, capacity = MAX_AUTOMATION_EVENTS
pub fn LoadAutomationEventList(fileName: *const ffi::c_char, ) -> AutomationEventList;
/// Unload automation events list from file
pub fn UnloadAutomationEventList(list: AutomationEventList, );
/// Export automation events list as text file
pub fn ExportAutomationEventList(list: AutomationEventList, fileName: *const ffi::c_char, ) -> bool;
/// Set automation event list to record to
pub fn SetAutomationEventList(list: *mut AutomationEventList, );
/// Set automation event internal base frame to start recording
pub fn SetAutomationEventBaseFrame(frame: ffi::c_int, );
/// Start recording automation events (AutomationEventList must be set)
pub fn StartAutomationEventRecording();
/// Stop recording automation events
pub fn StopAutomationEventRecording();
/// Play a recorded automation event
pub fn PlayAutomationEvent(event: AutomationEvent, );
/// Check if a key has been pressed once
pub fn IsKeyPressed(key: ffi::c_int, ) -> bool;
/// Check if a key has been pressed again (Only PLATFORM_DESKTOP)
pub fn IsKeyPressedRepeat(key: ffi::c_int, ) -> bool;
/// Check if a key is being pressed
pub fn IsKeyDown(key: ffi::c_int, ) -> bool;
/// Check if a key has been released once
pub fn IsKeyReleased(key: ffi::c_int, ) -> bool;
/// Check if a key is NOT being pressed
pub fn IsKeyUp(key: ffi::c_int, ) -> bool;
/// Get key pressed (keycode), call it multiple times for keys queued, returns 0 when the queue is empty
pub fn GetKeyPressed() -> ffi::c_int;
/// Get char pressed (unicode), call it multiple times for chars queued, returns 0 when the queue is empty
pub fn GetCharPressed() -> ffi::c_int;
/// Set a custom key to exit program (default is ESC)
pub fn SetExitKey(key: ffi::c_int, );
/// Check if a gamepad is available
pub fn IsGamepadAvailable(gamepad: ffi::c_int, ) -> bool;
/// Get gamepad internal name id
pub fn GetGamepadName(gamepad: ffi::c_int, ) -> *const ffi::c_char;
/// Check if a gamepad button has been pressed once
pub fn IsGamepadButtonPressed(gamepad: ffi::c_int, button: ffi::c_int, ) -> bool;
/// Check if a gamepad button is being pressed
pub fn IsGamepadButtonDown(gamepad: ffi::c_int, button: ffi::c_int, ) -> bool;
/// Check if a gamepad button has been released once
pub fn IsGamepadButtonReleased(gamepad: ffi::c_int, button: ffi::c_int, ) -> bool;
/// Check if a gamepad button is NOT being pressed
pub fn IsGamepadButtonUp(gamepad: ffi::c_int, button: ffi::c_int, ) -> bool;
/// Get the last gamepad button pressed
pub fn GetGamepadButtonPressed() -> ffi::c_int;
/// Get gamepad axis count for a gamepad
pub fn GetGamepadAxisCount(gamepad: ffi::c_int, ) -> ffi::c_int;
/// Get axis movement value for a gamepad axis
pub fn GetGamepadAxisMovement(gamepad: ffi::c_int, axis: ffi::c_int, ) -> ffi::c_float;
/// Set internal gamepad mappings (SDL_GameControllerDB)
pub fn SetGamepadMappings(mappings: *const ffi::c_char, ) -> ffi::c_int;
/// Set gamepad vibration for both motors
pub fn SetGamepadVibration(gamepad: ffi::c_int, leftMotor: ffi::c_float, rightMotor: ffi::c_float, );
/// Check if a mouse button has been pressed once
pub fn IsMouseButtonPressed(button: ffi::c_int, ) -> bool;
/// Check if a mouse button is being pressed
pub fn IsMouseButtonDown(button: ffi::c_int, ) -> bool;
/// Check if a mouse button has been released once
pub fn IsMouseButtonReleased(button: ffi::c_int, ) -> bool;
/// Check if a mouse button is NOT being pressed
pub fn IsMouseButtonUp(button: ffi::c_int, ) -> bool;
/// Get mouse position X
pub fn GetMouseX() -> ffi::c_int;
/// Get mouse position Y
pub fn GetMouseY() -> ffi::c_int;
/// Get mouse position XY
pub fn GetMousePosition() -> Vector2;
/// Get mouse delta between frames
pub fn GetMouseDelta() -> Vector2;
/// Set mouse position XY
pub fn SetMousePosition(x: ffi::c_int, y: ffi::c_int, );
/// Set mouse offset
pub fn SetMouseOffset(offsetX: ffi::c_int, offsetY: ffi::c_int, );
/// Set mouse scaling
pub fn SetMouseScale(scaleX: ffi::c_float, scaleY: ffi::c_float, );
/// Get mouse wheel movement for X or Y, whichever is larger
pub fn GetMouseWheelMove() -> ffi::c_float;
/// Get mouse wheel movement for both X and Y
pub fn GetMouseWheelMoveV() -> Vector2;
/// Set mouse cursor
pub fn SetMouseCursor(cursor: ffi::c_int, );
/// Get touch position X for touch point 0 (relative to screen size)
pub fn GetTouchX() -> ffi::c_int;
/// Get touch position Y for touch point 0 (relative to screen size)
pub fn GetTouchY() -> ffi::c_int;
/// Get touch position XY for a touch point index (relative to screen size)
pub fn GetTouchPosition(index: ffi::c_int, ) -> Vector2;
/// Get touch point identifier for given index
pub fn GetTouchPointId(index: ffi::c_int, ) -> ffi::c_int;
/// Get number of touch points
pub fn GetTouchPointCount() -> ffi::c_int;
/// Enable a set of gestures using flags
pub fn SetGesturesEnabled(flags: ffi::c_uint, );
/// Check if a gesture have been detected
pub fn IsGestureDetected(gesture: ffi::c_uint, ) -> bool;
/// Get latest detected gesture
pub fn GetGestureDetected() -> ffi::c_int;
/// Get gesture hold time in milliseconds
pub fn GetGestureHoldDuration() -> ffi::c_float;
/// Get gesture drag vector
pub fn GetGestureDragVector() -> Vector2;
/// Get gesture drag angle
pub fn GetGestureDragAngle() -> ffi::c_float;
/// Get gesture pinch delta
pub fn GetGesturePinchVector() -> Vector2;
/// Get gesture pinch angle
pub fn GetGesturePinchAngle() -> ffi::c_float;
/// Update camera position for selected mode
pub fn UpdateCamera(camera: *mut Camera, mode: ffi::c_int, );
/// Update camera movement/rotation
pub fn UpdateCameraPro(camera: *mut Camera, movement: Vector3, rotation: Vector3, zoom: ffi::c_float, );
/// Set texture and rectangle to be used on shapes drawing
pub fn SetShapesTexture(texture: Texture2D, source: Rectangle, );
/// Get texture that is used for shapes drawing
pub fn GetShapesTexture() -> Texture2D;
/// Get texture source rectangle that is used for shapes drawing
pub fn GetShapesTextureRectangle() -> Rectangle;
/// Draw a pixel
pub fn DrawPixel(posX: ffi::c_int, posY: ffi::c_int, color: Color, );
/// Draw a pixel (Vector version)
pub fn DrawPixelV(position: Vector2, color: Color, );
/// Draw a line
pub fn DrawLine(startPosX: ffi::c_int, startPosY: ffi::c_int, endPosX: ffi::c_int, endPosY: ffi::c_int, color: Color, );
/// Draw a line (using gl lines)
pub fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color, );
/// Draw a line (using triangles/quads)
pub fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: ffi::c_float, color: Color, );
/// Draw lines sequence (using gl lines)
pub fn DrawLineStrip(points: *mut Vector2, pointCount: ffi::c_int, color: Color, );
/// Draw line segment cubic-bezier in-out interpolation
pub fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: ffi::c_float, color: Color, );
/// Draw a color-filled circle
pub fn DrawCircle(centerX: ffi::c_int, centerY: ffi::c_int, radius: ffi::c_float, color: Color, );
/// Draw a piece of a circle
pub fn DrawCircleSector(center: Vector2, radius: ffi::c_float, startAngle: ffi::c_float, endAngle: ffi::c_float, segments: ffi::c_int, color: Color, );
/// Draw circle sector outline
pub fn DrawCircleSectorLines(center: Vector2, radius: ffi::c_float, startAngle: ffi::c_float, endAngle: ffi::c_float, segments: ffi::c_int, color: Color, );
/// Draw a gradient-filled circle
pub fn DrawCircleGradient(centerX: ffi::c_int, centerY: ffi::c_int, radius: ffi::c_float, color1: Color, color2: Color, );
/// Draw a color-filled circle (Vector version)
pub fn DrawCircleV(center: Vector2, radius: ffi::c_float, color: Color, );
/// Draw circle outline
pub fn DrawCircleLines(centerX: ffi::c_int, centerY: ffi::c_int, radius: ffi::c_float, color: Color, );
/// Draw circle outline (Vector version)
pub fn DrawCircleLinesV(center: Vector2, radius: ffi::c_float, color: Color, );
/// Draw ellipse
pub fn DrawEllipse(centerX: ffi::c_int, centerY: ffi::c_int, radiusH: ffi::c_float, radiusV: ffi::c_float, color: Color, );
/// Draw ellipse outline
pub fn DrawEllipseLines(centerX: ffi::c_int, centerY: ffi::c_int, radiusH: ffi::c_float, radiusV: ffi::c_float, color: Color, );
/// Draw ring
pub fn DrawRing(center: Vector2, innerRadius: ffi::c_float, outerRadius: ffi::c_float, startAngle: ffi::c_float, endAngle: ffi::c_float, segments: ffi::c_int, color: Color, );
/// Draw ring outline
pub fn DrawRingLines(center: Vector2, innerRadius: ffi::c_float, outerRadius: ffi::c_float, startAngle: ffi::c_float, endAngle: ffi::c_float, segments: ffi::c_int, color: Color, );
/// Draw a color-filled rectangle
pub fn DrawRectangle(posX: ffi::c_int, posY: ffi::c_int, width: ffi::c_int, height: ffi::c_int, color: Color, );
/// Draw a color-filled rectangle (Vector version)
pub fn DrawRectangleV(position: Vector2, size: Vector2, color: Color, );
/// Draw a color-filled rectangle
pub fn DrawRectangleRec(rec: Rectangle, color: Color, );
/// Draw a color-filled rectangle with pro parameters
pub fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: ffi::c_float, color: Color, );
/// Draw a vertical-gradient-filled rectangle
pub fn DrawRectangleGradientV(posX: ffi::c_int, posY: ffi::c_int, width: ffi::c_int, height: ffi::c_int, color1: Color, color2: Color, );
/// Draw a horizontal-gradient-filled rectangle
pub fn DrawRectangleGradientH(posX: ffi::c_int, posY: ffi::c_int, width: ffi::c_int, height: ffi::c_int, color1: Color, color2: Color, );
/// Draw a gradient-filled rectangle with custom vertex colors
pub fn DrawRectangleGradientEx(rec: Rectangle, col1: Color, col2: Color, col3: Color, col4: Color, );
/// Draw rectangle outline
pub fn DrawRectangleLines(posX: ffi::c_int, posY: ffi::c_int, width: ffi::c_int, height: ffi::c_int, color: Color, );
/// Draw rectangle outline with extended parameters
pub fn DrawRectangleLinesEx(rec: Rectangle, lineThick: ffi::c_float, color: Color, );
/// Draw rectangle with rounded edges
pub fn DrawRectangleRounded(rec: Rectangle, roundness: ffi::c_float, segments: ffi::c_int, color: Color, );
/// Draw rectangle lines with rounded edges
pub fn DrawRectangleRoundedLines(rec: Rectangle, roundness: ffi::c_float, segments: ffi::c_int, color: Color, );
/// Draw rectangle with rounded edges outline
pub fn DrawRectangleRoundedLinesEx(rec: Rectangle, roundness: ffi::c_float, segments: ffi::c_int, lineThick: ffi::c_float, color: Color, );
/// Draw a color-filled triangle (vertex in counter-clockwise order!)
pub fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color, );
/// Draw triangle outline (vertex in counter-clockwise order!)
pub fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color, );
/// Draw a triangle fan defined by points (first vertex is the center)
pub fn DrawTriangleFan(points: *mut Vector2, pointCount: ffi::c_int, color: Color, );
/// Draw a triangle strip defined by points
pub fn DrawTriangleStrip(points: *mut Vector2, pointCount: ffi::c_int, color: Color, );
/// Draw a regular polygon (Vector version)
pub fn DrawPoly(center: Vector2, sides: ffi::c_int, radius: ffi::c_float, rotation: ffi::c_float, color: Color, );
/// Draw a polygon outline of n sides
pub fn DrawPolyLines(center: Vector2, sides: ffi::c_int, radius: ffi::c_float, rotation: ffi::c_float, color: Color, );
/// Draw a polygon outline of n sides with extended parameters
pub fn DrawPolyLinesEx(center: Vector2, sides: ffi::c_int, radius: ffi::c_float, rotation: ffi::c_float, lineThick: ffi::c_float, color: Color, );
/// Draw spline: Linear, minimum 2 points
pub fn DrawSplineLinear(points: *mut Vector2, pointCount: ffi::c_int, thick: ffi::c_float, color: Color, );
/// Draw spline: B-Spline, minimum 4 points
pub fn DrawSplineBasis(points: *mut Vector2, pointCount: ffi::c_int, thick: ffi::c_float, color: Color, );
/// Draw spline: Catmull-Rom, minimum 4 points
pub fn DrawSplineCatmullRom(points: *mut Vector2, pointCount: ffi::c_int, thick: ffi::c_float, color: Color, );
/// Draw spline: Quadratic Bezier, minimum 3 points (1 control point): \[p1, c2, p3, c4...\]
pub fn DrawSplineBezierQuadratic(points: *mut Vector2, pointCount: ffi::c_int, thick: ffi::c_float, color: Color, );
/// Draw spline: Cubic Bezier, minimum 4 points (2 control points): \[p1, c2, c3, p4, c5, c6...\]
pub fn DrawSplineBezierCubic(points: *mut Vector2, pointCount: ffi::c_int, thick: ffi::c_float, color: Color, );
/// Draw spline segment: Linear, 2 points
pub fn DrawSplineSegmentLinear(p1: Vector2, p2: Vector2, thick: ffi::c_float, color: Color, );
/// Draw spline segment: B-Spline, 4 points
pub fn DrawSplineSegmentBasis(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thick: ffi::c_float, color: Color, );
/// Draw spline segment: Catmull-Rom, 4 points
pub fn DrawSplineSegmentCatmullRom(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thick: ffi::c_float, color: Color, );
/// Draw spline segment: Quadratic Bezier, 2 points, 1 control point
pub fn DrawSplineSegmentBezierQuadratic(p1: Vector2, c2: Vector2, p3: Vector2, thick: ffi::c_float, color: Color, );
/// Draw spline segment: Cubic Bezier, 2 points, 2 control points
pub fn DrawSplineSegmentBezierCubic(p1: Vector2, c2: Vector2, c3: Vector2, p4: Vector2, thick: ffi::c_float, color: Color, );
/// Get (evaluate) spline point: Linear
pub fn GetSplinePointLinear(startPos: Vector2, endPos: Vector2, t: ffi::c_float, ) -> Vector2;
/// Get (evaluate) spline point: B-Spline
pub fn GetSplinePointBasis(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: ffi::c_float, ) -> Vector2;
/// Get (evaluate) spline point: Catmull-Rom
pub fn GetSplinePointCatmullRom(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: ffi::c_float, ) -> Vector2;
/// Get (evaluate) spline point: Quadratic Bezier
pub fn GetSplinePointBezierQuad(p1: Vector2, c2: Vector2, p3: Vector2, t: ffi::c_float, ) -> Vector2;
/// Get (evaluate) spline point: Cubic Bezier
pub fn GetSplinePointBezierCubic(p1: Vector2, c2: Vector2, c3: Vector2, p4: Vector2, t: ffi::c_float, ) -> Vector2;
/// Check collision between two rectangles
pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle, ) -> bool;
/// Check collision between two circles
pub fn CheckCollisionCircles(center1: Vector2, radius1: ffi::c_float, center2: Vector2, radius2: ffi::c_float, ) -> bool;
/// Check collision between circle and rectangle
pub fn CheckCollisionCircleRec(center: Vector2, radius: ffi::c_float, rec: Rectangle, ) -> bool;
/// Check if point is inside rectangle
pub fn CheckCollisionPointRec(point: Vector2, rec: Rectangle, ) -> bool;
/// Check if point is inside circle
pub fn CheckCollisionPointCircle(point: Vector2, center: Vector2, radius: ffi::c_float, ) -> bool;
/// Check if point is inside a triangle
pub fn CheckCollisionPointTriangle(point: Vector2, p1: Vector2, p2: Vector2, p3: Vector2, ) -> bool;
/// Check if point is within a polygon described by array of vertices
pub fn CheckCollisionPointPoly(point: Vector2, points: *mut Vector2, pointCount: ffi::c_int, ) -> bool;
/// Check the collision between two lines defined by two points each, returns collision point by reference
pub fn CheckCollisionLines(startPos1: Vector2, endPos1: Vector2, startPos2: Vector2, endPos2: Vector2, collisionPoint: *mut Vector2, ) -> bool;
/// Check if point belongs to line created between two points \[p1\] and \[p2\] with defined margin in pixels \[threshold\]
pub fn CheckCollisionPointLine(point: Vector2, p1: Vector2, p2: Vector2, threshold: ffi::c_int, ) -> bool;
/// Check if circle collides with a line created betweeen two points \[p1\] and \[p2\]
pub fn CheckCollisionCircleLine(center: Vector2, radius: ffi::c_float, p1: Vector2, p2: Vector2, ) -> bool;
/// Get collision rectangle for two rectangles collision
pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle, ) -> Rectangle;
/// Load image from file into CPU memory (RAM)
pub fn LoadImage(fileName: *const ffi::c_char, ) -> Image;
/// Load image from RAW file data
pub fn LoadImageRaw(fileName: *const ffi::c_char, width: ffi::c_int, height: ffi::c_int, format: ffi::c_int, headerSize: ffi::c_int, ) -> Image;
/// Load image from SVG file data or string with specified size
pub fn LoadImageSvg(fileNameOrString: *const ffi::c_char, width: ffi::c_int, height: ffi::c_int, ) -> Image;
/// Load image sequence from file (frames appended to image.data)
pub fn LoadImageAnim(fileName: *const ffi::c_char, frames: *mut ffi::c_int, ) -> Image;
/// Load image sequence from memory buffer
pub fn LoadImageAnimFromMemory(fileType: *const ffi::c_char, fileData: *const ffi::c_uchar, dataSize: ffi::c_int, frames: *mut ffi::c_int, ) -> Image;
/// Load image from memory buffer, fileType refers to extension: i.e. '.png'
pub fn LoadImageFromMemory(fileType: *const ffi::c_char, fileData: *const ffi::c_uchar, dataSize: ffi::c_int, ) -> Image;
/// Load image from GPU texture data
pub fn LoadImageFromTexture(texture: Texture2D, ) -> Image;
/// Load image from screen buffer and (screenshot)
pub fn LoadImageFromScreen() -> Image;
/// Check if an image is ready
pub fn IsImageReady(image: Image, ) -> bool;
/// Unload image from CPU memory (RAM)
pub fn UnloadImage(image: Image, );
/// Export image data to file, returns true on success
pub fn ExportImage(image: Image, fileName: *const ffi::c_char, ) -> bool;
/// Export image to memory buffer
pub fn ExportImageToMemory(image: Image, fileType: *const ffi::c_char, fileSize: *mut ffi::c_int, ) -> *mut ffi::c_uchar;
/// Export image as code file defining an array of bytes, returns true on success
pub fn ExportImageAsCode(image: Image, fileName: *const ffi::c_char, ) -> bool;
/// Generate image: plain color
pub fn GenImageColor(width: ffi::c_int, height: ffi::c_int, color: Color, ) -> Image;
/// Generate image: linear gradient, direction in degrees \[0..360\], 0=Vertical gradient
pub fn GenImageGradientLinear(width: ffi::c_int, height: ffi::c_int, direction: ffi::c_int, start: Color, end: Color, ) -> Image;
/// Generate image: radial gradient
pub fn GenImageGradientRadial(width: ffi::c_int, height: ffi::c_int, density: ffi::c_float, inner: Color, outer: Color, ) -> Image;
/// Generate image: square gradient
pub fn GenImageGradientSquare(width: ffi::c_int, height: ffi::c_int, density: ffi::c_float, inner: Color, outer: Color, ) -> Image;
/// Generate image: checked
pub fn GenImageChecked(width: ffi::c_int, height: ffi::c_int, checksX: ffi::c_int, checksY: ffi::c_int, col1: Color, col2: Color, ) -> Image;
/// Generate image: white noise
pub fn GenImageWhiteNoise(width: ffi::c_int, height: ffi::c_int, factor: ffi::c_float, ) -> Image;
/// Generate image: perlin noise
pub fn GenImagePerlinNoise(width: ffi::c_int, height: ffi::c_int, offsetX: ffi::c_int, offsetY: ffi::c_int, scale: ffi::c_float, ) -> Image;
/// Generate image: cellular algorithm, bigger tileSize means bigger cells
pub fn GenImageCellular(width: ffi::c_int, height: ffi::c_int, tileSize: ffi::c_int, ) -> Image;
/// Generate image: grayscale image from text data
pub fn GenImageText(width: ffi::c_int, height: ffi::c_int, text: *const ffi::c_char, ) -> Image;
/// Create an image duplicate (useful for transformations)
pub fn ImageCopy(image: Image, ) -> Image;
/// Create an image from another image piece
pub fn ImageFromImage(image: Image, rec: Rectangle, ) -> Image;
/// Create an image from text (default font)
pub fn ImageText(text: *const ffi::c_char, fontSize: ffi::c_int, color: Color, ) -> Image;
/// Create an image from text (custom sprite font)
pub fn ImageTextEx(font: Font, text: *const ffi::c_char, fontSize: ffi::c_float, spacing: ffi::c_float, tint: Color, ) -> Image;
/// Convert image data to desired format
pub fn ImageFormat(image: *mut Image, newFormat: ffi::c_int, );
/// Convert image to POT (power-of-two)
pub fn ImageToPOT(image: *mut Image, fill: Color, );
/// Crop an image to a defined rectangle
pub fn ImageCrop(image: *mut Image, crop: Rectangle, );
/// Crop image depending on alpha value
pub fn ImageAlphaCrop(image: *mut Image, threshold: ffi::c_float, );
/// Clear alpha channel to desired color
pub fn ImageAlphaClear(image: *mut Image, color: Color, threshold: ffi::c_float, );
/// Apply alpha mask to image
pub fn ImageAlphaMask(image: *mut Image, alphaMask: Image, );
/// Premultiply alpha channel
pub fn ImageAlphaPremultiply(image: *mut Image, );
/// Apply Gaussian blur using a box blur approximation
pub fn ImageBlurGaussian(image: *mut Image, blurSize: ffi::c_int, );
/// Apply Custom Square image convolution kernel
pub fn ImageKernelConvolution(image: *mut Image, kernel: *mut ffi::c_float, kernelSize: ffi::c_int, );
/// Resize image (Bicubic scaling algorithm)
pub fn ImageResize(image: *mut Image, newWidth: ffi::c_int, newHeight: ffi::c_int, );
/// Resize image (Nearest-Neighbor scaling algorithm)
pub fn ImageResizeNN(image: *mut Image, newWidth: ffi::c_int, newHeight: ffi::c_int, );
/// Resize canvas and fill with color
pub fn ImageResizeCanvas(image: *mut Image, newWidth: ffi::c_int, newHeight: ffi::c_int, offsetX: ffi::c_int, offsetY: ffi::c_int, fill: Color, );
/// Compute all mipmap levels for a provided image
pub fn ImageMipmaps(image: *mut Image, );
/// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
pub fn ImageDither(image: *mut Image, rBpp: ffi::c_int, gBpp: ffi::c_int, bBpp: ffi::c_int, aBpp: ffi::c_int, );
/// Flip image vertically
pub fn ImageFlipVertical(image: *mut Image, );
/// Flip image horizontally
pub fn ImageFlipHorizontal(image: *mut Image, );
/// Rotate image by input angle in degrees (-359 to 359)
pub fn ImageRotate(image: *mut Image, degrees: ffi::c_int, );
/// Rotate image clockwise 90deg
pub fn ImageRotateCW(image: *mut Image, );
/// Rotate image counter-clockwise 90deg
pub fn ImageRotateCCW(image: *mut Image, );
/// Modify image color: tint
pub fn ImageColorTint(image: *mut Image, color: Color, );
/// Modify image color: invert
pub fn ImageColorInvert(image: *mut Image, );
/// Modify image color: grayscale
pub fn ImageColorGrayscale(image: *mut Image, );
/// Modify image color: contrast (-100 to 100)
pub fn ImageColorContrast(image: *mut Image, contrast: ffi::c_float, );
/// Modify image color: brightness (-255 to 255)
pub fn ImageColorBrightness(image: *mut Image, brightness: ffi::c_int, );
/// Modify image color: replace color
pub fn ImageColorReplace(image: *mut Image, color: Color, replace: Color, );
/// Load color data from image as a Color array (RGBA - 32bit)
pub fn LoadImageColors(image: Image, ) -> *mut Color;
/// Load colors palette from image as a Color array (RGBA - 32bit)
pub fn LoadImagePalette(image: Image, maxPaletteSize: ffi::c_int, colorCount: *mut ffi::c_int, ) -> *mut Color;
/// Unload color data loaded with LoadImageColors()
pub fn UnloadImageColors(colors: *mut Color, );
/// Unload colors palette loaded with LoadImagePalette()
pub fn UnloadImagePalette(colors: *mut Color, );
/// Get image alpha border rectangle
pub fn GetImageAlphaBorder(image: Image, threshold: ffi::c_float, ) -> Rectangle;
/// Get image pixel color at (x, y) position
pub fn GetImageColor(image: Image, x: ffi::c_int, y: ffi::c_int, ) -> Color;
/// Clear image background with given color
pub fn ImageClearBackground(dst: *mut Image, color: Color, );
/// Draw pixel within an image
pub fn ImageDrawPixel(dst: *mut Image, posX: ffi::c_int, posY: ffi::c_int, color: Color, );
/// Draw pixel within an image (Vector version)
pub fn ImageDrawPixelV(dst: *mut Image, position: Vector2, color: Color, );
/// Draw line within an image
pub fn ImageDrawLine(dst: *mut Image, startPosX: ffi::c_int, startPosY: ffi::c_int, endPosX: ffi::c_int, endPosY: ffi::c_int, color: Color, );
/// Draw line within an image (Vector version)
pub fn ImageDrawLineV(dst: *mut Image, start: Vector2, end: Vector2, color: Color, );
/// Draw a filled circle within an image
pub fn ImageDrawCircle(dst: *mut Image, centerX: ffi::c_int, centerY: ffi::c_int, radius: ffi::c_int, color: Color, );
/// Draw a filled circle within an image (Vector version)
pub fn ImageDrawCircleV(dst: *mut Image, center: Vector2, radius: ffi::c_int, color: Color, );
/// Draw circle outline within an image
pub fn ImageDrawCircleLines(dst: *mut Image, centerX: ffi::c_int, centerY: ffi::c_int, radius: ffi::c_int, color: Color, );
/// Draw circle outline within an image (Vector version)
pub fn ImageDrawCircleLinesV(dst: *mut Image, center: Vector2, radius: ffi::c_int, color: Color, );
/// Draw rectangle within an image
pub fn ImageDrawRectangle(dst: *mut Image, posX: ffi::c_int, posY: ffi::c_int, width: ffi::c_int, height: ffi::c_int, color: Color, );
/// Draw rectangle within an image (Vector version)
pub fn ImageDrawRectangleV(dst: *mut Image, position: Vector2, size: Vector2, color: Color, );
/// Draw rectangle within an image
pub fn ImageDrawRectangleRec(dst: *mut Image, rec: Rectangle, color: Color, );
/// Draw rectangle lines within an image
pub fn ImageDrawRectangleLines(dst: *mut Image, rec: Rectangle, thick: ffi::c_int, color: Color, );
/// Draw a source image within a destination image (tint applied to source)
pub fn ImageDraw(dst: *mut Image, src: Image, srcRec: Rectangle, dstRec: Rectangle, tint: Color, );
/// Draw text (using default font) within an image (destination)
pub fn ImageDrawText(dst: *mut Image, text: *const ffi::c_char, posX: ffi::c_int, posY: ffi::c_int, fontSize: ffi::c_int, color: Color, );
/// Draw text (custom sprite font) within an image (destination)
pub fn ImageDrawTextEx(dst: *mut Image, font: Font, text: *const ffi::c_char, position: Vector2, fontSize: ffi::c_float, spacing: ffi::c_float, tint: Color, );
/// Load texture from file into GPU memory (VRAM)
pub fn LoadTexture(fileName: *const ffi::c_char, ) -> Texture2D;
/// Load texture from image data
pub fn LoadTextureFromImage(image: Image, ) -> Texture2D;
/// Load cubemap from image, multiple image cubemap layouts supported
pub fn LoadTextureCubemap(image: Image, layout: ffi::c_int, ) -> TextureCubemap;
/// Load texture for rendering (framebuffer)
pub fn LoadRenderTexture(width: ffi::c_int, height: ffi::c_int, ) -> RenderTexture2D;
/// Check if a texture is ready
pub fn IsTextureReady(texture: Texture2D, ) -> bool;
/// Unload texture from GPU memory (VRAM)
pub fn UnloadTexture(texture: Texture2D, );
/// Check if a render texture is ready
pub fn IsRenderTextureReady(target: RenderTexture2D, ) -> bool;
/// Unload render texture from GPU memory (VRAM)
pub fn UnloadRenderTexture(target: RenderTexture2D, );
/// Update GPU texture with new data
pub fn UpdateTexture(texture: Texture2D, pixels: *const ffi::c_void, );
/// Update GPU texture rectangle with new data
pub fn UpdateTextureRec(texture: Texture2D, rec: Rectangle, pixels: *const ffi::c_void, );
/// Generate GPU mipmaps for a texture
pub fn GenTextureMipmaps(texture: *mut Texture2D, );
/// Set texture scaling filter mode
pub fn SetTextureFilter(texture: Texture2D, filter: ffi::c_int, );
/// Set texture wrapping mode
pub fn SetTextureWrap(texture: Texture2D, wrap: ffi::c_int, );
/// Draw a Texture2D
pub fn DrawTexture(texture: Texture2D, posX: ffi::c_int, posY: ffi::c_int, tint: Color, );
/// Draw a Texture2D with position defined as Vector2
pub fn DrawTextureV(texture: Texture2D, position: Vector2, tint: Color, );
/// Draw a Texture2D with extended parameters
pub fn DrawTextureEx(texture: Texture2D, position: Vector2, rotation: ffi::c_float, scale: ffi::c_float, tint: Color, );
/// Draw a part of a texture defined by a rectangle
pub fn DrawTextureRec(texture: Texture2D, source: Rectangle, position: Vector2, tint: Color, );
/// Draw a part of a texture defined by a rectangle with 'pro' parameters
pub fn DrawTexturePro(texture: Texture2D, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: ffi::c_float, tint: Color, );
/// Draws a texture (or part of it) that stretches or shrinks nicely
pub fn DrawTextureNPatch(texture: Texture2D, nPatchInfo: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: ffi::c_float, tint: Color, );
/// Check if two colors are equal
pub fn ColorIsEqual(col1: Color, col2: Color, ) -> bool;
/// Get color with alpha applied, alpha goes from 0.0f to 1.0f
pub fn Fade(color: Color, alpha: ffi::c_float, ) -> Color;
/// Get hexadecimal value for a Color (0xRRGGBBAA)
pub fn ColorToInt(color: Color, ) -> ffi::c_int;
/// Get Color normalized as float \[0..1\]
pub fn ColorNormalize(color: Color, ) -> Vector4;
/// Get Color from normalized values \[0..1\]
pub fn ColorFromNormalized(normalized: Vector4, ) -> Color;
/// Get HSV values for a Color, hue \[0..360\], saturation/value \[0..1\]
pub fn ColorToHSV(color: Color, ) -> Vector3;
/// Get a Color from HSV values, hue \[0..360\], saturation/value \[0..1\]
pub fn ColorFromHSV(hue: ffi::c_float, saturation: ffi::c_float, value: ffi::c_float, ) -> Color;
/// Get color multiplied with another color
pub fn ColorTint(color: Color, tint: Color, ) -> Color;
/// Get color with brightness correction, brightness factor goes from -1.0f to 1.0f
pub fn ColorBrightness(color: Color, factor: ffi::c_float, ) -> Color;
/// Get color with contrast correction, contrast values between -1.0f and 1.0f
pub fn ColorContrast(color: Color, contrast: ffi::c_float, ) -> Color;
/// Get color with alpha applied, alpha goes from 0.0f to 1.0f
pub fn ColorAlpha(color: Color, alpha: ffi::c_float, ) -> Color;
/// Get src alpha-blended into dst color with tint
pub fn ColorAlphaBlend(dst: Color, src: Color, tint: Color, ) -> Color;
/// Get Color structure from hexadecimal value
pub fn GetColor(hexValue: ffi::c_uint, ) -> Color;
/// Get Color from a source pixel pointer of certain format
pub fn GetPixelColor(srcPtr: *mut ffi::c_void, format: ffi::c_int, ) -> Color;
/// Set color formatted into destination pixel pointer
pub fn SetPixelColor(dstPtr: *mut ffi::c_void, color: Color, format: ffi::c_int, );
/// Get pixel data size in bytes for certain format
pub fn GetPixelDataSize(width: ffi::c_int, height: ffi::c_int, format: ffi::c_int, ) -> ffi::c_int;
/// Get the default Font
pub fn GetFontDefault() -> Font;
/// Load font from file into GPU memory (VRAM)
pub fn LoadFont(fileName: *const ffi::c_char, ) -> Font;
/// Load font from file with extended parameters, use NULL for codepoints and 0 for codepointCount to load the default character setFont
pub fn LoadFontEx(fileName: *const ffi::c_char, fontSize: ffi::c_int, codepoints: *mut ffi::c_int, codepointCount: ffi::c_int, ) -> Font;
/// Load font from Image (XNA style)
pub fn LoadFontFromImage(image: Image, key: Color, firstChar: ffi::c_int, ) -> Font;
/// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
pub fn LoadFontFromMemory(fileType: *const ffi::c_char, fileData: *const ffi::c_uchar, dataSize: ffi::c_int, fontSize: ffi::c_int, codepoints: *mut ffi::c_int, codepointCount: ffi::c_int, ) -> Font;
/// Check if a font is ready
pub fn IsFontReady(font: Font, ) -> bool;
/// Load font data for further use
pub fn LoadFontData(fileData: *const ffi::c_uchar, dataSize: ffi::c_int, fontSize: ffi::c_int, codepoints: *mut ffi::c_int, codepointCount: ffi::c_int, r#type: ffi::c_int, ) -> *mut GlyphInfo;
/// Generate image font atlas using chars info
pub fn GenImageFontAtlas(glyphs: *const GlyphInfo, glyphRecs: *mut *mut Rectangle, glyphCount: ffi::c_int, fontSize: ffi::c_int, padding: ffi::c_int, packMethod: ffi::c_int, ) -> Image;
/// Unload font chars info data (RAM)
pub fn UnloadFontData(glyphs: *mut GlyphInfo, glyphCount: ffi::c_int, );
/// Unload font from GPU memory (VRAM)
pub fn UnloadFont(font: Font, );
/// Export font as code file, returns true on success
pub fn ExportFontAsCode(font: Font, fileName: *const ffi::c_char, ) -> bool;
/// Draw current FPS
pub fn DrawFPS(posX: ffi::c_int, posY: ffi::c_int, );
/// Draw text (using default font)
pub fn DrawText(text: *const ffi::c_char, posX: ffi::c_int, posY: ffi::c_int, fontSize: ffi::c_int, color: Color, );
/// Draw text using font and additional parameters
pub fn DrawTextEx(font: Font, text: *const ffi::c_char, position: Vector2, fontSize: ffi::c_float, spacing: ffi::c_float, tint: Color, );
/// Draw text using Font and pro parameters (rotation)
pub fn DrawTextPro(font: Font, text: *const ffi::c_char, position: Vector2, origin: Vector2, rotation: ffi::c_float, fontSize: ffi::c_float, spacing: ffi::c_float, tint: Color, );
/// Draw one character (codepoint)
pub fn DrawTextCodepoint(font: Font, codepoint: ffi::c_int, position: Vector2, fontSize: ffi::c_float, tint: Color, );
/// Draw multiple character (codepoint)
pub fn DrawTextCodepoints(font: Font, codepoints: *const ffi::c_int, codepointCount: ffi::c_int, position: Vector2, fontSize: ffi::c_float, spacing: ffi::c_float, tint: Color, );
/// Set vertical line spacing when drawing with line-breaks
pub fn SetTextLineSpacing(spacing: ffi::c_int, );
/// Measure string width for default font
pub fn MeasureText(text: *const ffi::c_char, fontSize: ffi::c_int, ) -> ffi::c_int;
/// Measure string size for Font
pub fn MeasureTextEx(font: Font, text: *const ffi::c_char, fontSize: ffi::c_float, spacing: ffi::c_float, ) -> Vector2;
/// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
pub fn GetGlyphIndex(font: Font, codepoint: ffi::c_int, ) -> ffi::c_int;
/// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
pub fn GetGlyphInfo(font: Font, codepoint: ffi::c_int, ) -> GlyphInfo;
/// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
pub fn GetGlyphAtlasRec(font: Font, codepoint: ffi::c_int, ) -> Rectangle;
/// Load UTF-8 text encoded from codepoints array
pub fn LoadUTF8(codepoints: *const ffi::c_int, length: ffi::c_int, ) -> *mut ffi::c_char;
/// Unload UTF-8 text encoded from codepoints array
pub fn UnloadUTF8(text: *mut ffi::c_char, );
/// Load all codepoints from a UTF-8 text string, codepoints count returned by parameter
pub fn LoadCodepoints(text: *const ffi::c_char, count: *mut ffi::c_int, ) -> *mut ffi::c_int;
/// Unload codepoints data from memory
pub fn UnloadCodepoints(codepoints: *mut ffi::c_int, );
/// Get total number of codepoints in a UTF-8 encoded string
pub fn GetCodepointCount(text: *const ffi::c_char, ) -> ffi::c_int;
/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
pub fn GetCodepoint(text: *const ffi::c_char, codepointSize: *mut ffi::c_int, ) -> ffi::c_int;
/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
pub fn GetCodepointNext(text: *const ffi::c_char, codepointSize: *mut ffi::c_int, ) -> ffi::c_int;
/// Get previous codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
pub fn GetCodepointPrevious(text: *const ffi::c_char, codepointSize: *mut ffi::c_int, ) -> ffi::c_int;
/// Encode one codepoint into UTF-8 byte array (array length returned as parameter)
pub fn CodepointToUTF8(codepoint: ffi::c_int, utf8Size: *mut ffi::c_int, ) -> *const ffi::c_char;
/// Copy one string to another, returns bytes copied
pub fn TextCopy(dst: *mut ffi::c_char, src: *const ffi::c_char, ) -> ffi::c_int;
/// Check if two text string are equal
pub fn TextIsEqual(text1: *const ffi::c_char, text2: *const ffi::c_char, ) -> bool;
/// Get text length, checks for '\0' ending
pub fn TextLength(text: *const ffi::c_char, ) -> ffi::c_uint;
/// Text formatting with variables (sprintf() style)
pub fn TextFormat(text: *const ffi::c_char, args: ..., ) -> *const ffi::c_char;
/// Get a piece of a text string
pub fn TextSubtext(text: *const ffi::c_char, position: ffi::c_int, length: ffi::c_int, ) -> *const ffi::c_char;
/// Replace text string (WARNING: memory must be freed!)
pub fn TextReplace(text: *const ffi::c_char, replace: *const ffi::c_char, by: *const ffi::c_char, ) -> *mut ffi::c_char;
/// Insert text in a position (WARNING: memory must be freed!)
pub fn TextInsert(text: *const ffi::c_char, insert: *const ffi::c_char, position: ffi::c_int, ) -> *mut ffi::c_char;
/// Join text strings with delimiter
pub fn TextJoin(textList: *const *mut ffi::c_char, count: ffi::c_int, delimiter: *const ffi::c_char, ) -> *const ffi::c_char;
/// Split text into multiple strings
pub fn TextSplit(text: *const ffi::c_char, delimiter: ffi::c_char, count: *mut ffi::c_int, ) -> *const *mut ffi::c_char;
/// Append text at specific position and move cursor!
pub fn TextAppend(text: *mut ffi::c_char, append: *const ffi::c_char, position: *mut ffi::c_int, );
/// Find first text occurrence within a string
pub fn TextFindIndex(text: *const ffi::c_char, find: *const ffi::c_char, ) -> ffi::c_int;
/// Get upper case version of provided string
pub fn TextToUpper(text: *const ffi::c_char, ) -> *const ffi::c_char;
/// Get lower case version of provided string
pub fn TextToLower(text: *const ffi::c_char, ) -> *const ffi::c_char;
/// Get Pascal case notation version of provided string
pub fn TextToPascal(text: *const ffi::c_char, ) -> *const ffi::c_char;
/// Get Snake case notation version of provided string
pub fn TextToSnake(text: *const ffi::c_char, ) -> *const ffi::c_char;
/// Get Camel case notation version of provided string
pub fn TextToCamel(text: *const ffi::c_char, ) -> *const ffi::c_char;
/// Get integer value from text (negative values not supported)
pub fn TextToInteger(text: *const ffi::c_char, ) -> ffi::c_int;
/// Get float value from text (negative values not supported)
pub fn TextToFloat(text: *const ffi::c_char, ) -> ffi::c_float;
/// Draw a line in 3D world space
pub fn DrawLine3D(startPos: Vector3, endPos: Vector3, color: Color, );
/// Draw a point in 3D space, actually a small line
pub fn DrawPoint3D(position: Vector3, color: Color, );
/// Draw a circle in 3D world space
pub fn DrawCircle3D(center: Vector3, radius: ffi::c_float, rotationAxis: Vector3, rotationAngle: ffi::c_float, color: Color, );
/// Draw a color-filled triangle (vertex in counter-clockwise order!)
pub fn DrawTriangle3D(v1: Vector3, v2: Vector3, v3: Vector3, color: Color, );
/// Draw a triangle strip defined by points
pub fn DrawTriangleStrip3D(points: *mut Vector3, pointCount: ffi::c_int, color: Color, );
/// Draw cube
pub fn DrawCube(position: Vector3, width: ffi::c_float, height: ffi::c_float, length: ffi::c_float, color: Color, );
/// Draw cube (Vector version)
pub fn DrawCubeV(position: Vector3, size: Vector3, color: Color, );
/// Draw cube wires
pub fn DrawCubeWires(position: Vector3, width: ffi::c_float, height: ffi::c_float, length: ffi::c_float, color: Color, );
/// Draw cube wires (Vector version)
pub fn DrawCubeWiresV(position: Vector3, size: Vector3, color: Color, );
/// Draw sphere
pub fn DrawSphere(centerPos: Vector3, radius: ffi::c_float, color: Color, );
/// Draw sphere with extended parameters
pub fn DrawSphereEx(centerPos: Vector3, radius: ffi::c_float, rings: ffi::c_int, slices: ffi::c_int, color: Color, );
/// Draw sphere wires
pub fn DrawSphereWires(centerPos: Vector3, radius: ffi::c_float, rings: ffi::c_int, slices: ffi::c_int, color: Color, );
/// Draw a cylinder/cone
pub fn DrawCylinder(position: Vector3, radiusTop: ffi::c_float, radiusBottom: ffi::c_float, height: ffi::c_float, slices: ffi::c_int, color: Color, );
/// Draw a cylinder with base at startPos and top at endPos
pub fn DrawCylinderEx(startPos: Vector3, endPos: Vector3, startRadius: ffi::c_float, endRadius: ffi::c_float, sides: ffi::c_int, color: Color, );
/// Draw a cylinder/cone wires
pub fn DrawCylinderWires(position: Vector3, radiusTop: ffi::c_float, radiusBottom: ffi::c_float, height: ffi::c_float, slices: ffi::c_int, color: Color, );
/// Draw a cylinder wires with base at startPos and top at endPos
pub fn DrawCylinderWiresEx(startPos: Vector3, endPos: Vector3, startRadius: ffi::c_float, endRadius: ffi::c_float, sides: ffi::c_int, color: Color, );
/// Draw a capsule with the center of its sphere caps at startPos and endPos
pub fn DrawCapsule(startPos: Vector3, endPos: Vector3, radius: ffi::c_float, slices: ffi::c_int, rings: ffi::c_int, color: Color, );
/// Draw capsule wireframe with the center of its sphere caps at startPos and endPos
pub fn DrawCapsuleWires(startPos: Vector3, endPos: Vector3, radius: ffi::c_float, slices: ffi::c_int, rings: ffi::c_int, color: Color, );
/// Draw a plane XZ
pub fn DrawPlane(centerPos: Vector3, size: Vector2, color: Color, );
/// Draw a ray line
pub fn DrawRay(ray: Ray, color: Color, );
/// Draw a grid (centered at (0, 0, 0))
pub fn DrawGrid(slices: ffi::c_int, spacing: ffi::c_float, );
/// Load model from files (meshes and materials)
pub fn LoadModel(fileName: *const ffi::c_char, ) -> Model;
/// Load model from generated mesh (default material)
pub fn LoadModelFromMesh(mesh: Mesh, ) -> Model;
/// Check if a model is ready
pub fn IsModelReady(model: Model, ) -> bool;
/// Unload model (including meshes) from memory (RAM and/or VRAM)
pub fn UnloadModel(model: Model, );
/// Compute model bounding box limits (considers all meshes)
pub fn GetModelBoundingBox(model: Model, ) -> BoundingBox;
/// Draw a model (with texture if set)
pub fn DrawModel(model: Model, position: Vector3, scale: ffi::c_float, tint: Color, );
/// Draw a model with extended parameters
pub fn DrawModelEx(model: Model, position: Vector3, rotationAxis: Vector3, rotationAngle: ffi::c_float, scale: Vector3, tint: Color, );
/// Draw a model wires (with texture if set)
pub fn DrawModelWires(model: Model, position: Vector3, scale: ffi::c_float, tint: Color, );
/// Draw a model wires (with texture if set) with extended parameters
pub fn DrawModelWiresEx(model: Model, position: Vector3, rotationAxis: Vector3, rotationAngle: ffi::c_float, scale: Vector3, tint: Color, );
/// Draw bounding box (wires)
pub fn DrawBoundingBox(r#box: BoundingBox, color: Color, );
/// Draw a billboard texture
pub fn DrawBillboard(camera: Camera, texture: Texture2D, position: Vector3, size: ffi::c_float, tint: Color, );
/// Draw a billboard texture defined by source
pub fn DrawBillboardRec(camera: Camera, texture: Texture2D, source: Rectangle, position: Vector3, size: Vector2, tint: Color, );
/// Draw a billboard texture defined by source and rotation
pub fn DrawBillboardPro(camera: Camera, texture: Texture2D, source: Rectangle, position: Vector3, up: Vector3, size: Vector2, origin: Vector2, rotation: ffi::c_float, tint: Color, );
/// Upload mesh vertex data in GPU and provide VAO/VBO ids
pub fn UploadMesh(mesh: *mut Mesh, dynamic: bool, );
/// Update mesh vertex data in GPU for a specific buffer index
pub fn UpdateMeshBuffer(mesh: Mesh, index: ffi::c_int, data: *const ffi::c_void, dataSize: ffi::c_int, offset: ffi::c_int, );
/// Unload mesh data from CPU and GPU
pub fn UnloadMesh(mesh: Mesh, );
/// Draw a 3d mesh with material and transform
pub fn DrawMesh(mesh: Mesh, material: Material, transform: Matrix, );
/// Draw multiple mesh instances with material and different transforms
pub fn DrawMeshInstanced(mesh: Mesh, material: Material, transforms: *const Matrix, instances: ffi::c_int, );
/// Compute mesh bounding box limits
pub fn GetMeshBoundingBox(mesh: Mesh, ) -> BoundingBox;
/// Compute mesh tangents
pub fn GenMeshTangents(mesh: *mut Mesh, );
/// Export mesh data to file, returns true on success
pub fn ExportMesh(mesh: Mesh, fileName: *const ffi::c_char, ) -> bool;
/// Export mesh as code file (.h) defining multiple arrays of vertex attributes
pub fn ExportMeshAsCode(mesh: Mesh, fileName: *const ffi::c_char, ) -> bool;
/// Generate polygonal mesh
pub fn GenMeshPoly(sides: ffi::c_int, radius: ffi::c_float, ) -> Mesh;
/// Generate plane mesh (with subdivisions)
pub fn GenMeshPlane(width: ffi::c_float, length: ffi::c_float, resX: ffi::c_int, resZ: ffi::c_int, ) -> Mesh;
/// Generate cuboid mesh
pub fn GenMeshCube(width: ffi::c_float, height: ffi::c_float, length: ffi::c_float, ) -> Mesh;
/// Generate sphere mesh (standard sphere)
pub fn GenMeshSphere(radius: ffi::c_float, rings: ffi::c_int, slices: ffi::c_int, ) -> Mesh;
/// Generate half-sphere mesh (no bottom cap)
pub fn GenMeshHemiSphere(radius: ffi::c_float, rings: ffi::c_int, slices: ffi::c_int, ) -> Mesh;
/// Generate cylinder mesh
pub fn GenMeshCylinder(radius: ffi::c_float, height: ffi::c_float, slices: ffi::c_int, ) -> Mesh;
/// Generate cone/pyramid mesh
pub fn GenMeshCone(radius: ffi::c_float, height: ffi::c_float, slices: ffi::c_int, ) -> Mesh;
/// Generate torus mesh
pub fn GenMeshTorus(radius: ffi::c_float, size: ffi::c_float, radSeg: ffi::c_int, sides: ffi::c_int, ) -> Mesh;
/// Generate trefoil knot mesh
pub fn GenMeshKnot(radius: ffi::c_float, size: ffi::c_float, radSeg: ffi::c_int, sides: ffi::c_int, ) -> Mesh;
/// Generate heightmap mesh from image data
pub fn GenMeshHeightmap(heightmap: Image, size: Vector3, ) -> Mesh;
/// Generate cubes-based map mesh from image data
pub fn GenMeshCubicmap(cubicmap: Image, cubeSize: Vector3, ) -> Mesh;
/// Load materials from model file
pub fn LoadMaterials(fileName: *const ffi::c_char, materialCount: *mut ffi::c_int, ) -> *mut Material;
/// Load default material (Supports: DIFFUSE, SPECULAR, NORMAL maps)
pub fn LoadMaterialDefault() -> Material;
/// Check if a material is ready
pub fn IsMaterialReady(material: Material, ) -> bool;
/// Unload material from GPU memory (VRAM)
pub fn UnloadMaterial(material: Material, );
/// Set texture for a material map type (MATERIAL_MAP_DIFFUSE, MATERIAL_MAP_SPECULAR...)
pub fn SetMaterialTexture(material: *mut Material, mapType: ffi::c_int, texture: Texture2D, );
/// Set material for a mesh
pub fn SetModelMeshMaterial(model: *mut Model, meshId: ffi::c_int, materialId: ffi::c_int, );
/// Load model animations from file
pub fn LoadModelAnimations(fileName: *const ffi::c_char, animCount: *mut ffi::c_int, ) -> *mut ModelAnimation;
/// Update model animation pose
pub fn UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: ffi::c_int, );
/// Unload animation data
pub fn UnloadModelAnimation(anim: ModelAnimation, );
/// Unload animation array data
pub fn UnloadModelAnimations(animations: *mut ModelAnimation, animCount: ffi::c_int, );
/// Check model animation skeleton match
pub fn IsModelAnimationValid(model: Model, anim: ModelAnimation, ) -> bool;
/// Check collision between two spheres
pub fn CheckCollisionSpheres(center1: Vector3, radius1: ffi::c_float, center2: Vector3, radius2: ffi::c_float, ) -> bool;
/// Check collision between two bounding boxes
pub fn CheckCollisionBoxes(box1: BoundingBox, box2: BoundingBox, ) -> bool;
/// Check collision between box and sphere
pub fn CheckCollisionBoxSphere(r#box: BoundingBox, center: Vector3, radius: ffi::c_float, ) -> bool;
/// Get collision info between ray and sphere
pub fn GetRayCollisionSphere(ray: Ray, center: Vector3, radius: ffi::c_float, ) -> RayCollision;
/// Get collision info between ray and box
pub fn GetRayCollisionBox(ray: Ray, r#box: BoundingBox, ) -> RayCollision;
/// Get collision info between ray and mesh
pub fn GetRayCollisionMesh(ray: Ray, mesh: Mesh, transform: Matrix, ) -> RayCollision;
/// Get collision info between ray and triangle
pub fn GetRayCollisionTriangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3, ) -> RayCollision;
/// Get collision info between ray and quad
pub fn GetRayCollisionQuad(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3, p4: Vector3, ) -> RayCollision;
/// Initialize audio device and context
pub fn InitAudioDevice();
/// Close the audio device and context
pub fn CloseAudioDevice();
/// Check if audio device has been initialized successfully
pub fn IsAudioDeviceReady() -> bool;
/// Set master volume (listener)
pub fn SetMasterVolume(volume: ffi::c_float, );
/// Get master volume (listener)
pub fn GetMasterVolume() -> ffi::c_float;
/// Load wave data from file
pub fn LoadWave(fileName: *const ffi::c_char, ) -> Wave;
/// Load wave from memory buffer, fileType refers to extension: i.e. '.wav'
pub fn LoadWaveFromMemory(fileType: *const ffi::c_char, fileData: *const ffi::c_uchar, dataSize: ffi::c_int, ) -> Wave;
/// Checks if wave data is ready
pub fn IsWaveReady(wave: Wave, ) -> bool;
/// Load sound from file
pub fn LoadSound(fileName: *const ffi::c_char, ) -> Sound;
/// Load sound from wave data
pub fn LoadSoundFromWave(wave: Wave, ) -> Sound;
/// Create a new sound that shares the same sample data as the source sound, does not own the sound data
pub fn LoadSoundAlias(source: Sound, ) -> Sound;
/// Checks if a sound is ready
pub fn IsSoundReady(sound: Sound, ) -> bool;
/// Update sound buffer with new data
pub fn UpdateSound(sound: Sound, data: *const ffi::c_void, sampleCount: ffi::c_int, );
/// Unload wave data
pub fn UnloadWave(wave: Wave, );
/// Unload sound
pub fn UnloadSound(sound: Sound, );
/// Unload a sound alias (does not deallocate sample data)
pub fn UnloadSoundAlias(alias: Sound, );
/// Export wave data to file, returns true on success
pub fn ExportWave(wave: Wave, fileName: *const ffi::c_char, ) -> bool;
/// Export wave sample data to code (.h), returns true on success
pub fn ExportWaveAsCode(wave: Wave, fileName: *const ffi::c_char, ) -> bool;
/// Play a sound
pub fn PlaySound(sound: Sound, );
/// Stop playing a sound
pub fn StopSound(sound: Sound, );
/// Pause a sound
pub fn PauseSound(sound: Sound, );
/// Resume a paused sound
pub fn ResumeSound(sound: Sound, );
/// Check if a sound is currently playing
pub fn IsSoundPlaying(sound: Sound, ) -> bool;
/// Set volume for a sound (1.0 is max level)
pub fn SetSoundVolume(sound: Sound, volume: ffi::c_float, );
/// Set pitch for a sound (1.0 is base level)
pub fn SetSoundPitch(sound: Sound, pitch: ffi::c_float, );
/// Set pan for a sound (0.5 is center)
pub fn SetSoundPan(sound: Sound, pan: ffi::c_float, );
/// Copy a wave to a new wave
pub fn WaveCopy(wave: Wave, ) -> Wave;
/// Crop a wave to defined frames range
pub fn WaveCrop(wave: *mut Wave, initFrame: ffi::c_int, finalFrame: ffi::c_int, );
/// Convert wave data to desired format
pub fn WaveFormat(wave: *mut Wave, sampleRate: ffi::c_int, sampleSize: ffi::c_int, channels: ffi::c_int, );
/// Load samples data from wave as a 32bit float data array
pub fn LoadWaveSamples(wave: Wave, ) -> *mut ffi::c_float;
/// Unload samples data loaded with LoadWaveSamples()
pub fn UnloadWaveSamples(samples: *mut ffi::c_float, );
/// Load music stream from file
pub fn LoadMusicStream(fileName: *const ffi::c_char, ) -> Music;
/// Load music stream from data
pub fn LoadMusicStreamFromMemory(fileType: *const ffi::c_char, data: *const ffi::c_uchar, dataSize: ffi::c_int, ) -> Music;
/// Checks if a music stream is ready
pub fn IsMusicReady(music: Music, ) -> bool;
/// Unload music stream
pub fn UnloadMusicStream(music: Music, );
/// Start music playing
pub fn PlayMusicStream(music: Music, );
/// Check if music is playing
pub fn IsMusicStreamPlaying(music: Music, ) -> bool;
/// Updates buffers for music streaming
pub fn UpdateMusicStream(music: Music, );
/// Stop music playing
pub fn StopMusicStream(music: Music, );
/// Pause music playing
pub fn PauseMusicStream(music: Music, );
/// Resume playing paused music
pub fn ResumeMusicStream(music: Music, );
/// Seek music to a position (in seconds)
pub fn SeekMusicStream(music: Music, position: ffi::c_float, );
/// Set volume for music (1.0 is max level)
pub fn SetMusicVolume(music: Music, volume: ffi::c_float, );
/// Set pitch for a music (1.0 is base level)
pub fn SetMusicPitch(music: Music, pitch: ffi::c_float, );
/// Set pan for a music (0.5 is center)
pub fn SetMusicPan(music: Music, pan: ffi::c_float, );
/// Get music time length (in seconds)
pub fn GetMusicTimeLength(music: Music, ) -> ffi::c_float;
/// Get current music time played (in seconds)
pub fn GetMusicTimePlayed(music: Music, ) -> ffi::c_float;
/// Load audio stream (to stream raw audio pcm data)
pub fn LoadAudioStream(sampleRate: ffi::c_uint, sampleSize: ffi::c_uint, channels: ffi::c_uint, ) -> AudioStream;
/// Checks if an audio stream is ready
pub fn IsAudioStreamReady(stream: AudioStream, ) -> bool;
/// Unload audio stream and free memory
pub fn UnloadAudioStream(stream: AudioStream, );
/// Update audio stream buffers with data
pub fn UpdateAudioStream(stream: AudioStream, data: *const ffi::c_void, frameCount: ffi::c_int, );
/// Check if any audio stream buffers requires refill
pub fn IsAudioStreamProcessed(stream: AudioStream, ) -> bool;
/// Play audio stream
pub fn PlayAudioStream(stream: AudioStream, );
/// Pause audio stream
pub fn PauseAudioStream(stream: AudioStream, );
/// Resume audio stream
pub fn ResumeAudioStream(stream: AudioStream, );
/// Check if audio stream is playing
pub fn IsAudioStreamPlaying(stream: AudioStream, ) -> bool;
/// Stop audio stream
pub fn StopAudioStream(stream: AudioStream, );
/// Set volume for audio stream (1.0 is max level)
pub fn SetAudioStreamVolume(stream: AudioStream, volume: ffi::c_float, );
/// Set pitch for audio stream (1.0 is base level)
pub fn SetAudioStreamPitch(stream: AudioStream, pitch: ffi::c_float, );
/// Set pan for audio stream (0.5 is centered)
pub fn SetAudioStreamPan(stream: AudioStream, pan: ffi::c_float, );
/// Default size for new audio streams
pub fn SetAudioStreamBufferSizeDefault(size: ffi::c_int, );
/// Audio thread callback to request new data
pub fn SetAudioStreamCallback(stream: AudioStream, callback: AudioCallback, );
/// Attach audio stream processor to stream, receives the samples as 'float'
pub fn AttachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback, );
/// Detach audio stream processor from stream
pub fn DetachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback, );
/// Attach audio stream processor to the entire audio pipeline, receives the samples as 'float'
pub fn AttachAudioMixedProcessor(processor: AudioCallback, );
/// Detach audio stream processor from the entire audio pipeline
pub fn DetachAudioMixedProcessor(processor: AudioCallback, );
}
/// System/Window config flags
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum ConfigFlags {
    /// Set to try enabling V-Sync on GPU
    VsyncHint = 64,
    /// Set to run program in fullscreen
    FullscreenMode = 2,
    /// Set to allow resizable window
    WindowResizable = 4,
    /// Set to disable window decoration (frame and buttons)
    WindowUndecorated = 8,
    /// Set to hide window
    WindowHidden = 128,
    /// Set to minimize window (iconify)
    WindowMinimized = 512,
    /// Set to maximize window (expanded to monitor)
    WindowMaximized = 1024,
    /// Set to window non focused
    WindowUnfocused = 2048,
    /// Set to window always on top
    WindowTopmost = 4096,
    /// Set to allow windows running while minimized
    WindowAlwaysRun = 256,
    /// Set to allow transparent framebuffer
    WindowTransparent = 16,
    /// Set to support HighDPI
    WindowHighdpi = 8192,
    /// Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
    WindowMousePassthrough = 16384,
    /// Set to run program in borderless windowed mode
    BorderlessWindowedMode = 32768,
    /// Set to try enabling MSAA 4X
    Msaa4XHint = 32,
    /// Set to try enabling interlaced video format (for V3D)
    InterlacedHint = 65536,
}
impl TryFrom<i32> for ConfigFlags {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            64 => Ok(ConfigFlags::VsyncHint),
            2 => Ok(ConfigFlags::FullscreenMode),
            4 => Ok(ConfigFlags::WindowResizable),
            8 => Ok(ConfigFlags::WindowUndecorated),
            128 => Ok(ConfigFlags::WindowHidden),
            512 => Ok(ConfigFlags::WindowMinimized),
            1024 => Ok(ConfigFlags::WindowMaximized),
            2048 => Ok(ConfigFlags::WindowUnfocused),
            4096 => Ok(ConfigFlags::WindowTopmost),
            256 => Ok(ConfigFlags::WindowAlwaysRun),
            16 => Ok(ConfigFlags::WindowTransparent),
            8192 => Ok(ConfigFlags::WindowHighdpi),
            16384 => Ok(ConfigFlags::WindowMousePassthrough),
            32768 => Ok(ConfigFlags::BorderlessWindowedMode),
            32 => Ok(ConfigFlags::Msaa4XHint),
            65536 => Ok(ConfigFlags::InterlacedHint),
            _ => Err(())
        }
    }
}
/// Trace log level
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum TraceLogLevel {
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
impl TryFrom<i32> for TraceLogLevel {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(TraceLogLevel::All),
            1 => Ok(TraceLogLevel::Trace),
            2 => Ok(TraceLogLevel::Debug),
            3 => Ok(TraceLogLevel::Info),
            4 => Ok(TraceLogLevel::Warning),
            5 => Ok(TraceLogLevel::Error),
            6 => Ok(TraceLogLevel::Fatal),
            7 => Ok(TraceLogLevel::None),
            _ => Err(())
        }
    }
}
/// Keyboard keys (US keyboard layout)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Key {
    /// Key: NULL, used for no key pressed
    Null = 0,
    /// Key: '
    Apostrophe = 39,
    /// Key: ,
    Comma = 44,
    /// Key: -
    Minus = 45,
    /// Key: .
    Period = 46,
    /// Key: /
    Slash = 47,
    /// Key: 0
    Zero = 48,
    /// Key: 1
    One = 49,
    /// Key: 2
    Two = 50,
    /// Key: 3
    Three = 51,
    /// Key: 4
    Four = 52,
    /// Key: 5
    Five = 53,
    /// Key: 6
    Six = 54,
    /// Key: 7
    Seven = 55,
    /// Key: 8
    Eight = 56,
    /// Key: 9
    Nine = 57,
    /// Key: ;
    Semicolon = 59,
    /// Key: =
    Equal = 61,
    /// Key: A | a
    A = 65,
    /// Key: B | b
    B = 66,
    /// Key: C | c
    C = 67,
    /// Key: D | d
    D = 68,
    /// Key: E | e
    E = 69,
    /// Key: F | f
    F = 70,
    /// Key: G | g
    G = 71,
    /// Key: H | h
    H = 72,
    /// Key: I | i
    I = 73,
    /// Key: J | j
    J = 74,
    /// Key: K | k
    K = 75,
    /// Key: L | l
    L = 76,
    /// Key: M | m
    M = 77,
    /// Key: N | n
    N = 78,
    /// Key: O | o
    O = 79,
    /// Key: P | p
    P = 80,
    /// Key: Q | q
    Q = 81,
    /// Key: R | r
    R = 82,
    /// Key: S | s
    S = 83,
    /// Key: T | t
    T = 84,
    /// Key: U | u
    U = 85,
    /// Key: V | v
    V = 86,
    /// Key: W | w
    W = 87,
    /// Key: X | x
    X = 88,
    /// Key: Y | y
    Y = 89,
    /// Key: Z | z
    Z = 90,
    /// Key: \[
    LeftBracket = 91,
    /// Key: '\'
    Backslash = 92,
    /// Key: \]
    RightBracket = 93,
    /// Key: `
    Grave = 96,
    /// Key: Space
    Space = 32,
    /// Key: Esc
    Escape = 256,
    /// Key: Enter
    Enter = 257,
    /// Key: Tab
    Tab = 258,
    /// Key: Backspace
    Backspace = 259,
    /// Key: Ins
    Insert = 260,
    /// Key: Del
    Delete = 261,
    /// Key: Cursor right
    Right = 262,
    /// Key: Cursor left
    Left = 263,
    /// Key: Cursor down
    Down = 264,
    /// Key: Cursor up
    Up = 265,
    /// Key: Page up
    PageUp = 266,
    /// Key: Page down
    PageDown = 267,
    /// Key: Home
    Home = 268,
    /// Key: End
    End = 269,
    /// Key: Caps lock
    CapsLock = 280,
    /// Key: Scroll down
    ScrollLock = 281,
    /// Key: Num lock
    NumLock = 282,
    /// Key: Print screen
    PrintScreen = 283,
    /// Key: Pause
    Pause = 284,
    /// Key: F1
    F1 = 290,
    /// Key: F2
    F2 = 291,
    /// Key: F3
    F3 = 292,
    /// Key: F4
    F4 = 293,
    /// Key: F5
    F5 = 294,
    /// Key: F6
    F6 = 295,
    /// Key: F7
    F7 = 296,
    /// Key: F8
    F8 = 297,
    /// Key: F9
    F9 = 298,
    /// Key: F10
    F10 = 299,
    /// Key: F11
    F11 = 300,
    /// Key: F12
    F12 = 301,
    /// Key: Shift left
    LeftShift = 340,
    /// Key: Control left
    LeftControl = 341,
    /// Key: Alt left
    LeftAlt = 342,
    /// Key: Super left
    LeftSuper = 343,
    /// Key: Shift right
    RightShift = 344,
    /// Key: Control right
    RightControl = 345,
    /// Key: Alt right
    RightAlt = 346,
    /// Key: Super right
    RightSuper = 347,
    /// Key: KB menu
    KbMenu = 348,
    /// Key: Keypad 0
    Kp0 = 320,
    /// Key: Keypad 1
    Kp1 = 321,
    /// Key: Keypad 2
    Kp2 = 322,
    /// Key: Keypad 3
    Kp3 = 323,
    /// Key: Keypad 4
    Kp4 = 324,
    /// Key: Keypad 5
    Kp5 = 325,
    /// Key: Keypad 6
    Kp6 = 326,
    /// Key: Keypad 7
    Kp7 = 327,
    /// Key: Keypad 8
    Kp8 = 328,
    /// Key: Keypad 9
    Kp9 = 329,
    /// Key: Keypad .
    KpDecimal = 330,
    /// Key: Keypad /
    KpDivide = 331,
    /// Key: Keypad *
    KpMultiply = 332,
    /// Key: Keypad -
    KpSubtract = 333,
    /// Key: Keypad +
    KpAdd = 334,
    /// Key: Keypad Enter
    KpEnter = 335,
    /// Key: Keypad =
    KpEqual = 336,
    /// Key: Android back button
    Back = 4,
    /// Key: Android menu button
    Menu = 5,
    /// Key: Android volume up button
    VolumeUp = 24,
    /// Key: Android volume down button
    VolumeDown = 25,
}
impl TryFrom<i32> for Key {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(Key::Null),
            39 => Ok(Key::Apostrophe),
            44 => Ok(Key::Comma),
            45 => Ok(Key::Minus),
            46 => Ok(Key::Period),
            47 => Ok(Key::Slash),
            48 => Ok(Key::Zero),
            49 => Ok(Key::One),
            50 => Ok(Key::Two),
            51 => Ok(Key::Three),
            52 => Ok(Key::Four),
            53 => Ok(Key::Five),
            54 => Ok(Key::Six),
            55 => Ok(Key::Seven),
            56 => Ok(Key::Eight),
            57 => Ok(Key::Nine),
            59 => Ok(Key::Semicolon),
            61 => Ok(Key::Equal),
            65 => Ok(Key::A),
            66 => Ok(Key::B),
            67 => Ok(Key::C),
            68 => Ok(Key::D),
            69 => Ok(Key::E),
            70 => Ok(Key::F),
            71 => Ok(Key::G),
            72 => Ok(Key::H),
            73 => Ok(Key::I),
            74 => Ok(Key::J),
            75 => Ok(Key::K),
            76 => Ok(Key::L),
            77 => Ok(Key::M),
            78 => Ok(Key::N),
            79 => Ok(Key::O),
            80 => Ok(Key::P),
            81 => Ok(Key::Q),
            82 => Ok(Key::R),
            83 => Ok(Key::S),
            84 => Ok(Key::T),
            85 => Ok(Key::U),
            86 => Ok(Key::V),
            87 => Ok(Key::W),
            88 => Ok(Key::X),
            89 => Ok(Key::Y),
            90 => Ok(Key::Z),
            91 => Ok(Key::LeftBracket),
            92 => Ok(Key::Backslash),
            93 => Ok(Key::RightBracket),
            96 => Ok(Key::Grave),
            32 => Ok(Key::Space),
            256 => Ok(Key::Escape),
            257 => Ok(Key::Enter),
            258 => Ok(Key::Tab),
            259 => Ok(Key::Backspace),
            260 => Ok(Key::Insert),
            261 => Ok(Key::Delete),
            262 => Ok(Key::Right),
            263 => Ok(Key::Left),
            264 => Ok(Key::Down),
            265 => Ok(Key::Up),
            266 => Ok(Key::PageUp),
            267 => Ok(Key::PageDown),
            268 => Ok(Key::Home),
            269 => Ok(Key::End),
            280 => Ok(Key::CapsLock),
            281 => Ok(Key::ScrollLock),
            282 => Ok(Key::NumLock),
            283 => Ok(Key::PrintScreen),
            284 => Ok(Key::Pause),
            290 => Ok(Key::F1),
            291 => Ok(Key::F2),
            292 => Ok(Key::F3),
            293 => Ok(Key::F4),
            294 => Ok(Key::F5),
            295 => Ok(Key::F6),
            296 => Ok(Key::F7),
            297 => Ok(Key::F8),
            298 => Ok(Key::F9),
            299 => Ok(Key::F10),
            300 => Ok(Key::F11),
            301 => Ok(Key::F12),
            340 => Ok(Key::LeftShift),
            341 => Ok(Key::LeftControl),
            342 => Ok(Key::LeftAlt),
            343 => Ok(Key::LeftSuper),
            344 => Ok(Key::RightShift),
            345 => Ok(Key::RightControl),
            346 => Ok(Key::RightAlt),
            347 => Ok(Key::RightSuper),
            348 => Ok(Key::KbMenu),
            320 => Ok(Key::Kp0),
            321 => Ok(Key::Kp1),
            322 => Ok(Key::Kp2),
            323 => Ok(Key::Kp3),
            324 => Ok(Key::Kp4),
            325 => Ok(Key::Kp5),
            326 => Ok(Key::Kp6),
            327 => Ok(Key::Kp7),
            328 => Ok(Key::Kp8),
            329 => Ok(Key::Kp9),
            330 => Ok(Key::KpDecimal),
            331 => Ok(Key::KpDivide),
            332 => Ok(Key::KpMultiply),
            333 => Ok(Key::KpSubtract),
            334 => Ok(Key::KpAdd),
            335 => Ok(Key::KpEnter),
            336 => Ok(Key::KpEqual),
            4 => Ok(Key::Back),
            5 => Ok(Key::Menu),
            24 => Ok(Key::VolumeUp),
            25 => Ok(Key::VolumeDown),
            _ => Err(())
        }
    }
}
/// Mouse buttons
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum MouseButton {
    /// Mouse button left
    Left = 0,
    /// Mouse button right
    Right = 1,
    /// Mouse button middle (pressed wheel)
    Middle = 2,
    /// Mouse button side (advanced mouse device)
    Side = 3,
    /// Mouse button extra (advanced mouse device)
    Extra = 4,
    /// Mouse button forward (advanced mouse device)
    Forward = 5,
    /// Mouse button back (advanced mouse device)
    Back = 6,
}
impl TryFrom<i32> for MouseButton {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(MouseButton::Left),
            1 => Ok(MouseButton::Right),
            2 => Ok(MouseButton::Middle),
            3 => Ok(MouseButton::Side),
            4 => Ok(MouseButton::Extra),
            5 => Ok(MouseButton::Forward),
            6 => Ok(MouseButton::Back),
            _ => Err(())
        }
    }
}
/// Mouse cursor
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum MouseCursor {
    /// Default pointer shape
    Default = 0,
    /// Arrow shape
    Arrow = 1,
    /// Text writing cursor shape
    Ibeam = 2,
    /// Cross shape
    Crosshair = 3,
    /// Pointing hand cursor
    PointingHand = 4,
    /// Horizontal resize/move arrow shape
    ResizeEw = 5,
    /// Vertical resize/move arrow shape
    ResizeNs = 6,
    /// Top-left to bottom-right diagonal resize/move arrow shape
    ResizeNwse = 7,
    /// The top-right to bottom-left diagonal resize/move arrow shape
    ResizeNesw = 8,
    /// The omnidirectional resize/move cursor shape
    ResizeAll = 9,
    /// The operation-not-allowed shape
    NotAllowed = 10,
}
impl TryFrom<i32> for MouseCursor {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(MouseCursor::Default),
            1 => Ok(MouseCursor::Arrow),
            2 => Ok(MouseCursor::Ibeam),
            3 => Ok(MouseCursor::Crosshair),
            4 => Ok(MouseCursor::PointingHand),
            5 => Ok(MouseCursor::ResizeEw),
            6 => Ok(MouseCursor::ResizeNs),
            7 => Ok(MouseCursor::ResizeNwse),
            8 => Ok(MouseCursor::ResizeNesw),
            9 => Ok(MouseCursor::ResizeAll),
            10 => Ok(MouseCursor::NotAllowed),
            _ => Err(())
        }
    }
}
/// Gamepad buttons
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum GamepadButton {
    /// Unknown button, just for error checking
    Unknown = 0,
    /// Gamepad left DPAD up button
    LeftFaceUp = 1,
    /// Gamepad left DPAD right button
    LeftFaceRight = 2,
    /// Gamepad left DPAD down button
    LeftFaceDown = 3,
    /// Gamepad left DPAD left button
    LeftFaceLeft = 4,
    /// Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
    RightFaceUp = 5,
    /// Gamepad right button right (i.e. PS3: Circle, Xbox: B)
    RightFaceRight = 6,
    /// Gamepad right button down (i.e. PS3: Cross, Xbox: A)
    RightFaceDown = 7,
    /// Gamepad right button left (i.e. PS3: Square, Xbox: X)
    RightFaceLeft = 8,
    /// Gamepad top/back trigger left (first), it could be a trailing button
    LeftTrigger1 = 9,
    /// Gamepad top/back trigger left (second), it could be a trailing button
    LeftTrigger2 = 10,
    /// Gamepad top/back trigger right (first), it could be a trailing button
    RightTrigger1 = 11,
    /// Gamepad top/back trigger right (second), it could be a trailing button
    RightTrigger2 = 12,
    /// Gamepad center buttons, left one (i.e. PS3: Select)
    MiddleLeft = 13,
    /// Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
    Middle = 14,
    /// Gamepad center buttons, right one (i.e. PS3: Start)
    MiddleRight = 15,
    /// Gamepad joystick pressed button left
    LeftThumb = 16,
    /// Gamepad joystick pressed button right
    RightThumb = 17,
}
impl TryFrom<i32> for GamepadButton {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(GamepadButton::Unknown),
            1 => Ok(GamepadButton::LeftFaceUp),
            2 => Ok(GamepadButton::LeftFaceRight),
            3 => Ok(GamepadButton::LeftFaceDown),
            4 => Ok(GamepadButton::LeftFaceLeft),
            5 => Ok(GamepadButton::RightFaceUp),
            6 => Ok(GamepadButton::RightFaceRight),
            7 => Ok(GamepadButton::RightFaceDown),
            8 => Ok(GamepadButton::RightFaceLeft),
            9 => Ok(GamepadButton::LeftTrigger1),
            10 => Ok(GamepadButton::LeftTrigger2),
            11 => Ok(GamepadButton::RightTrigger1),
            12 => Ok(GamepadButton::RightTrigger2),
            13 => Ok(GamepadButton::MiddleLeft),
            14 => Ok(GamepadButton::Middle),
            15 => Ok(GamepadButton::MiddleRight),
            16 => Ok(GamepadButton::LeftThumb),
            17 => Ok(GamepadButton::RightThumb),
            _ => Err(())
        }
    }
}
/// Gamepad axis
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum GamepadAxis {
    /// Gamepad left stick X axis
    LeftX = 0,
    /// Gamepad left stick Y axis
    LeftY = 1,
    /// Gamepad right stick X axis
    RightX = 2,
    /// Gamepad right stick Y axis
    RightY = 3,
    /// Gamepad back trigger left, pressure level: \[1..-1\]
    LeftTrigger = 4,
    /// Gamepad back trigger right, pressure level: \[1..-1\]
    RightTrigger = 5,
}
impl TryFrom<i32> for GamepadAxis {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(GamepadAxis::LeftX),
            1 => Ok(GamepadAxis::LeftY),
            2 => Ok(GamepadAxis::RightX),
            3 => Ok(GamepadAxis::RightY),
            4 => Ok(GamepadAxis::LeftTrigger),
            5 => Ok(GamepadAxis::RightTrigger),
            _ => Err(())
        }
    }
}
/// Material map index
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum MaterialMapIndex {
    /// Albedo material (same as: MATERIAL_MAP_DIFFUSE)
    Albedo = 0,
    /// Metalness material (same as: MATERIAL_MAP_SPECULAR)
    Metalness = 1,
    /// Normal material
    Normal = 2,
    /// Roughness material
    Roughness = 3,
    /// Ambient occlusion material
    Occlusion = 4,
    /// Emission material
    Emission = 5,
    /// Heightmap material
    Height = 6,
    /// Cubemap material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
    Cubemap = 7,
    /// Irradiance material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
    Irradiance = 8,
    /// Prefilter material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
    Prefilter = 9,
    /// Brdf material
    Brdf = 10,
}
impl TryFrom<i32> for MaterialMapIndex {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(MaterialMapIndex::Albedo),
            1 => Ok(MaterialMapIndex::Metalness),
            2 => Ok(MaterialMapIndex::Normal),
            3 => Ok(MaterialMapIndex::Roughness),
            4 => Ok(MaterialMapIndex::Occlusion),
            5 => Ok(MaterialMapIndex::Emission),
            6 => Ok(MaterialMapIndex::Height),
            7 => Ok(MaterialMapIndex::Cubemap),
            8 => Ok(MaterialMapIndex::Irradiance),
            9 => Ok(MaterialMapIndex::Prefilter),
            10 => Ok(MaterialMapIndex::Brdf),
            _ => Err(())
        }
    }
}
/// Shader location index
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum ShaderLocationIndex {
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
    /// Shader location: sampler2d texture: albedo (same as: SHADER_LOC_MAP_DIFFUSE)
    MapAlbedo = 15,
    /// Shader location: sampler2d texture: metalness (same as: SHADER_LOC_MAP_SPECULAR)
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
impl TryFrom<i32> for ShaderLocationIndex {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(ShaderLocationIndex::VertexPosition),
            1 => Ok(ShaderLocationIndex::VertexTexcoord01),
            2 => Ok(ShaderLocationIndex::VertexTexcoord02),
            3 => Ok(ShaderLocationIndex::VertexNormal),
            4 => Ok(ShaderLocationIndex::VertexTangent),
            5 => Ok(ShaderLocationIndex::VertexColor),
            6 => Ok(ShaderLocationIndex::MatrixMvp),
            7 => Ok(ShaderLocationIndex::MatrixView),
            8 => Ok(ShaderLocationIndex::MatrixProjection),
            9 => Ok(ShaderLocationIndex::MatrixModel),
            10 => Ok(ShaderLocationIndex::MatrixNormal),
            11 => Ok(ShaderLocationIndex::VectorView),
            12 => Ok(ShaderLocationIndex::ColorDiffuse),
            13 => Ok(ShaderLocationIndex::ColorSpecular),
            14 => Ok(ShaderLocationIndex::ColorAmbient),
            15 => Ok(ShaderLocationIndex::MapAlbedo),
            16 => Ok(ShaderLocationIndex::MapMetalness),
            17 => Ok(ShaderLocationIndex::MapNormal),
            18 => Ok(ShaderLocationIndex::MapRoughness),
            19 => Ok(ShaderLocationIndex::MapOcclusion),
            20 => Ok(ShaderLocationIndex::MapEmission),
            21 => Ok(ShaderLocationIndex::MapHeight),
            22 => Ok(ShaderLocationIndex::MapCubemap),
            23 => Ok(ShaderLocationIndex::MapIrradiance),
            24 => Ok(ShaderLocationIndex::MapPrefilter),
            25 => Ok(ShaderLocationIndex::MapBrdf),
            _ => Err(())
        }
    }
}
/// Shader uniform data type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum ShaderUniformDataType {
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
impl TryFrom<i32> for ShaderUniformDataType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(ShaderUniformDataType::Float),
            1 => Ok(ShaderUniformDataType::Vec2),
            2 => Ok(ShaderUniformDataType::Vec3),
            3 => Ok(ShaderUniformDataType::Vec4),
            4 => Ok(ShaderUniformDataType::Int),
            5 => Ok(ShaderUniformDataType::Ivec2),
            6 => Ok(ShaderUniformDataType::Ivec3),
            7 => Ok(ShaderUniformDataType::Ivec4),
            8 => Ok(ShaderUniformDataType::Sampler2D),
            _ => Err(())
        }
    }
}
/// Shader attribute data types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum ShaderAttributeDataType {
    /// Shader attribute type: float
    Float = 0,
    /// Shader attribute type: vec2 (2 float)
    Vec2 = 1,
    /// Shader attribute type: vec3 (3 float)
    Vec3 = 2,
    /// Shader attribute type: vec4 (4 float)
    Vec4 = 3,
}
impl TryFrom<i32> for ShaderAttributeDataType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(ShaderAttributeDataType::Float),
            1 => Ok(ShaderAttributeDataType::Vec2),
            2 => Ok(ShaderAttributeDataType::Vec3),
            3 => Ok(ShaderAttributeDataType::Vec4),
            _ => Err(())
        }
    }
}
/// Pixel formats
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum PixelFormat {
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
impl TryFrom<i32> for PixelFormat {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            1 => Ok(PixelFormat::UncompressedGrayscale),
            2 => Ok(PixelFormat::UncompressedGrayAlpha),
            3 => Ok(PixelFormat::UncompressedR5G6B5),
            4 => Ok(PixelFormat::UncompressedR8G8B8),
            5 => Ok(PixelFormat::UncompressedR5G5B5A1),
            6 => Ok(PixelFormat::UncompressedR4G4B4A4),
            7 => Ok(PixelFormat::UncompressedR8G8B8A8),
            8 => Ok(PixelFormat::UncompressedR32),
            9 => Ok(PixelFormat::UncompressedR32G32B32),
            10 => Ok(PixelFormat::UncompressedR32G32B32A32),
            11 => Ok(PixelFormat::UncompressedR16),
            12 => Ok(PixelFormat::UncompressedR16G16B16),
            13 => Ok(PixelFormat::UncompressedR16G16B16A16),
            14 => Ok(PixelFormat::CompressedDxt1Rgb),
            15 => Ok(PixelFormat::CompressedDxt1Rgba),
            16 => Ok(PixelFormat::CompressedDxt3Rgba),
            17 => Ok(PixelFormat::CompressedDxt5Rgba),
            18 => Ok(PixelFormat::CompressedEtc1Rgb),
            19 => Ok(PixelFormat::CompressedEtc2Rgb),
            20 => Ok(PixelFormat::CompressedEtc2EacRgba),
            21 => Ok(PixelFormat::CompressedPvrtRgb),
            22 => Ok(PixelFormat::CompressedPvrtRgba),
            23 => Ok(PixelFormat::CompressedAstc4X4Rgba),
            24 => Ok(PixelFormat::CompressedAstc8X8Rgba),
            _ => Err(())
        }
    }
}
/// Texture parameters: filter mode
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum TextureFilter {
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
impl TryFrom<i32> for TextureFilter {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(TextureFilter::Point),
            1 => Ok(TextureFilter::Bilinear),
            2 => Ok(TextureFilter::Trilinear),
            3 => Ok(TextureFilter::Anisotropic4X),
            4 => Ok(TextureFilter::Anisotropic8X),
            5 => Ok(TextureFilter::Anisotropic16X),
            _ => Err(())
        }
    }
}
/// Texture parameters: wrap mode
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum TextureWrap {
    /// Repeats texture in tiled mode
    Repeat = 0,
    /// Clamps texture to edge pixel in tiled mode
    Clamp = 1,
    /// Mirrors and repeats the texture in tiled mode
    MirrorRepeat = 2,
    /// Mirrors and clamps to border the texture in tiled mode
    MirrorClamp = 3,
}
impl TryFrom<i32> for TextureWrap {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(TextureWrap::Repeat),
            1 => Ok(TextureWrap::Clamp),
            2 => Ok(TextureWrap::MirrorRepeat),
            3 => Ok(TextureWrap::MirrorClamp),
            _ => Err(())
        }
    }
}
/// Cubemap layouts
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum CubemapLayout {
    /// Automatically detect layout type
    AutoDetect = 0,
    /// Layout is defined by a vertical line with faces
    LineVertical = 1,
    /// Layout is defined by a horizontal line with faces
    LineHorizontal = 2,
    /// Layout is defined by a 3x4 cross with cubemap faces
    CrossThreeByFour = 3,
    /// Layout is defined by a 4x3 cross with cubemap faces
    CrossFourByThree = 4,
    /// Layout is defined by a panorama image (equirrectangular map)
    Panorama = 5,
}
impl TryFrom<i32> for CubemapLayout {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(CubemapLayout::AutoDetect),
            1 => Ok(CubemapLayout::LineVertical),
            2 => Ok(CubemapLayout::LineHorizontal),
            3 => Ok(CubemapLayout::CrossThreeByFour),
            4 => Ok(CubemapLayout::CrossFourByThree),
            5 => Ok(CubemapLayout::Panorama),
            _ => Err(())
        }
    }
}
/// Font type, defines generation method
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum FontType {
    /// Default font generation, anti-aliased
    Default = 0,
    /// Bitmap font generation, no anti-aliasing
    Bitmap = 1,
    /// SDF font generation, requires external shader
    Sdf = 2,
}
impl TryFrom<i32> for FontType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(FontType::Default),
            1 => Ok(FontType::Bitmap),
            2 => Ok(FontType::Sdf),
            _ => Err(())
        }
    }
}
/// Color blending modes (pre-defined)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum BlendMode {
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
    /// Blend textures using custom rgb/alpha separate src/dst factors (use rlSetBlendFactorsSeparate())
    CustomSeparate = 7,
}
impl TryFrom<i32> for BlendMode {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(BlendMode::Alpha),
            1 => Ok(BlendMode::Additive),
            2 => Ok(BlendMode::Multiplied),
            3 => Ok(BlendMode::AddColors),
            4 => Ok(BlendMode::SubtractColors),
            5 => Ok(BlendMode::AlphaPremultiply),
            6 => Ok(BlendMode::Custom),
            7 => Ok(BlendMode::CustomSeparate),
            _ => Err(())
        }
    }
}
/// Gesture
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Gesture {
    /// No gesture
    None = 0,
    /// Tap gesture
    Tap = 1,
    /// Double tap gesture
    Doubletap = 2,
    /// Hold gesture
    Hold = 4,
    /// Drag gesture
    Drag = 8,
    /// Swipe right gesture
    SwipeRight = 16,
    /// Swipe left gesture
    SwipeLeft = 32,
    /// Swipe up gesture
    SwipeUp = 64,
    /// Swipe down gesture
    SwipeDown = 128,
    /// Pinch in gesture
    PinchIn = 256,
    /// Pinch out gesture
    PinchOut = 512,
}
impl TryFrom<i32> for Gesture {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(Gesture::None),
            1 => Ok(Gesture::Tap),
            2 => Ok(Gesture::Doubletap),
            4 => Ok(Gesture::Hold),
            8 => Ok(Gesture::Drag),
            16 => Ok(Gesture::SwipeRight),
            32 => Ok(Gesture::SwipeLeft),
            64 => Ok(Gesture::SwipeUp),
            128 => Ok(Gesture::SwipeDown),
            256 => Ok(Gesture::PinchIn),
            512 => Ok(Gesture::PinchOut),
            _ => Err(())
        }
    }
}
/// Camera system modes
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum CameraMode {
    /// Camera custom, controlled by user (UpdateCamera() does nothing)
    Custom = 0,
    /// Camera free mode
    Free = 1,
    /// Camera orbital, around target, zoom supported
    Orbital = 2,
    /// Camera first person
    FirstPerson = 3,
    /// Camera third person
    ThirdPerson = 4,
}
impl TryFrom<i32> for CameraMode {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(CameraMode::Custom),
            1 => Ok(CameraMode::Free),
            2 => Ok(CameraMode::Orbital),
            3 => Ok(CameraMode::FirstPerson),
            4 => Ok(CameraMode::ThirdPerson),
            _ => Err(())
        }
    }
}
/// Camera projection
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum CameraProjection {
    /// Perspective projection
    Perspective = 0,
    /// Orthographic projection
    Orthographic = 1,
}
impl TryFrom<i32> for CameraProjection {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(CameraProjection::Perspective),
            1 => Ok(CameraProjection::Orthographic),
            _ => Err(())
        }
    }
}
/// N-patch layout
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum NPatchLayout {
    /// Npatch layout: 3x3 tiles
    NinePatch = 0,
    /// Npatch layout: 1x3 tiles
    ThreePatchVertical = 1,
    /// Npatch layout: 3x1 tiles
    ThreePatchHorizontal = 2,
}
impl TryFrom<i32> for NPatchLayout {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match value {
            0 => Ok(NPatchLayout::NinePatch),
            1 => Ok(NPatchLayout::ThreePatchVertical),
            2 => Ok(NPatchLayout::ThreePatchHorizontal),
            _ => Err(())
        }
    }
}
