use super::{DrawHandle3D, material::Material};

use crate::ffi;
use crate::prelude::{Vector2, Vector3, Vector4, Color, Image, Matrix, BoundingBox};

/// A raylib mesh.
/// 
/// Unlike a [`ffi::Mesh`], a [`Mesh`] is garanteed to have been uploaded to the GPU and to be drawable.
/// Note that to draw a mesh, you need to create a [`DrawHandle3D`].
/// 
/// To procedurally create a mesh, use the [`MeshBuilder`] struct (see the `mesh_generation` example).
#[derive(Debug)]
#[repr(transparent)]
pub struct Mesh(ffi::Mesh);

/// # Mesh generation functions
impl Mesh {
    /// Generates a polygonal mesh.
    /// 
    /// # Panics
    /// Panics if `sides` is less than `3`.
    #[inline]
    pub fn gen_poly(sides: u32, radius: f32) -> Mesh {
        assert!(sides >= 3);
        Mesh(unsafe { ffi::GenMeshPoly(sides as i32, radius) })
    }
    /// Generates a plane mesh.
    #[inline]
    pub fn gen_plane(w: f32, length: f32, res_x: u32, res_z: u32) -> Mesh {
        Mesh(unsafe { ffi::GenMeshPlane(w, length, res_x as i32, res_z as i32) })
    }
    /// Generates a cuboid mesh.
    #[inline]
    pub fn gen_cube(w: f32, h: f32, l: f32) -> Mesh {
        Mesh(unsafe { ffi::GenMeshCube(w, h, l) })
    }
    /// Generates a sphere mesh (standard UV sphere).
    #[inline]
    pub fn gen_sphere(radius: f32, rings: u32, slices: u32) -> Mesh {
        Mesh(unsafe { ffi::GenMeshSphere(radius, rings as i32, slices as i32) })
    }
    /// Generates a half-shere mesh (no bottom cap).
    #[inline]
    pub fn gen_hemisphere(radius: f32, rings: u32, slices: u32) -> Mesh {
        Mesh(unsafe { ffi::GenMeshHemiSphere(radius, rings as i32, slices as i32) })
    }
    /// Generates a cylinder mesh.
    #[inline]
    pub fn gen_cylinder(radius: f32, height: f32, slices: u32) -> Mesh {
        Mesh(unsafe { ffi::GenMeshCylinder(radius, height, slices as i32) })
    }
    /// Generates a cone/pyramid mesh.
    #[inline]
    pub fn gen_cone(radius: f32, height: f32, slices: u32) -> Mesh {
        Mesh(unsafe { ffi::GenMeshCone(radius, height, slices as i32) })
    }
    /// Generates a torus mesh.
    #[inline]
    pub fn gen_torus(radius: f32, size: f32, rad_seg: u32, sides: u32) -> Mesh {
        Mesh(unsafe { ffi::GenMeshTorus(radius, size, rad_seg as i32, sides as i32) })
    }
    /// Generates a trefoil knot mesh.
    #[inline]
    pub fn gen_knot(radius: f32, size: f32, rad_seg: u32, sides: u32) -> Mesh {
        Mesh(unsafe { ffi::GenMeshKnot(radius, size, rad_seg as i32, sides as i32) })
    }
    /// Generates a heightmap mesh from image data.
    #[inline]
    pub fn gen_heightmap(heightmap: &Image, size: Vector3) -> Mesh {
        Mesh(unsafe { ffi::GenMeshHeightmap(heightmap.get_ffi_image(), size) })
    }
    /// Generates a cubes-based map mesh from image data.
    #[inline]
    pub fn gen_cubicmap(cubicmap: &Image, size: Vector3) -> Mesh {
        Mesh(unsafe { ffi::GenMeshCubicmap(cubicmap.get_ffi_image(), size) })
    }
}

impl Mesh {
    pub fn get_bounding_box(&self) -> BoundingBox {
        unsafe { ffi::GetMeshBoundingBox(self.0) }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe { ffi::UnloadMesh(self.0) }
    }
}

pub struct MeshBuilder(ffi::Mesh);

fn copy_slice_to_raylib_memory<T: Copy>(slice: &[T]) -> *mut T {
    let mem = unsafe { ffi::MemAlloc((slice.len() * std::mem::size_of::<T>()) as u32) };
    let mem = mem as *mut T;
    let mem = unsafe { std::slice::from_raw_parts_mut(mem, slice.len()) };
    mem.copy_from_slice(slice);
    mem.as_mut_ptr()
}

impl MeshBuilder {
    /// Creates a new mesh builder.
    /// ```
    /// # use raylib::prelude::*;
    /// # let rl = &mut init_window(800, 800, "Test", 60);
    /// let vertices = &[vec3(0.0, 0.0, 0.0), vec3(0.5, 1.0, 0.0), vec3(1.0, 0.0, 0.0)];
    /// let texcoords = &[vec2(0.0, 0.0), vec2(0.5, 1.0), vec2(1.0, 0.0)];
    /// let normals = &[vec3(-1.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(1.0, 0.0, 0.0)];
    /// 
    /// let triangle = MeshBuilder::new(vertices, texcoords)
    ///     .normals(normals)
    ///     .build();
    /// ```
    /// 
    /// # Panics
    /// Panics if `vertices` and `texcoords` have different length, or if their length is bigger than `i32::MAX`.
    pub fn new(vertices: &[Vector3], texcoords: &[Vector2]) -> Self {
        assert_eq!(vertices.len(), texcoords.len());
        assert!(vertices.len() < i32::MAX as usize);

        Self(ffi::Mesh {
            vertexCount: vertices.len() as i32,
            triangleCount: vertices.len() as i32 / 3,
            vertices: copy_slice_to_raylib_memory(vertices) as *mut f32,   // SAFETY: Vector3 contains 3 floats and is repr(C)
            texcoords: copy_slice_to_raylib_memory(texcoords) as *mut f32, // SAFETY: Vector2 contains 2 floats and is repr(C)
            texcoords2: std::ptr::null_mut(),
            normals: std::ptr::null_mut(),
            tangents: std::ptr::null_mut(),
            colors: std::ptr::null_mut(),
            indices: std::ptr::null_mut(),
            animVertices: std::ptr::null_mut(),
            animNormals: std::ptr::null_mut(),
            boneIds: std::ptr::null_mut(),
            boneWeights: std::ptr::null_mut(),
            vaoId: 0,
            vboId: std::ptr::null_mut(),
        })
    }

    /// Sets the second uv buffer.
    /// 
    /// # Panics
    /// Panics if the given slice doesn't have the same length as the original vertex buffer.
    pub fn texcoords2(mut self, texcoords2: &[Vector2]) -> Self {
        assert_eq!(self.0.vertexCount as usize, texcoords2.len());

        self.0.normals = copy_slice_to_raylib_memory(texcoords2) as *mut f32;
        self
    }

    /// Sets the normal buffer.
    /// 
    /// # Panics
    /// Panics if the given slice doesn't have the same length as the original vertex buffer.
    pub fn normals(mut self, normals: &[Vector3]) -> Self {
        assert_eq!(self.0.vertexCount as usize, normals.len());

        self.0.normals = copy_slice_to_raylib_memory(normals) as *mut f32;
        self
    }

    /// Set the tangents buffer.
    /// 
    /// # Panics
    /// Panics if the given slice doesn't have the same length as the original vertex buffer.
    pub fn tangents(mut self, tangents: &[Vector4]) -> Self {
        assert_eq!(self.0.vertexCount as usize, tangents.len());

        self.0.tangents = copy_slice_to_raylib_memory(tangents) as *mut f32;
        self
    }

    /// Set the colors buffer.
    /// 
    /// # Panics
    /// Panics if the given slice doesn't have the same length as the original vertex buffer.
    pub fn colors(mut self, colors: &[Color]) -> Self {
        assert_eq!(self.0.vertexCount as usize, colors.len());

        self.0.colors = copy_slice_to_raylib_memory(colors) as *mut u8;
        self
    }

    /// Make the mesh indexed.
    /// Note that indexed mesh can't have more than `u16::MAX` vertices (raylib limitation).
    pub fn indices(mut self, indices: &[[u16; 3]]) -> Self {
        self.0.vertexCount = indices.len() as i32;
        self.0.indices = copy_slice_to_raylib_memory(indices) as *mut u16;
        self
    }

    pub fn build(mut self) -> Mesh {
        unsafe { ffi::UploadMesh(&mut self.0 as *mut _, false) };
        Mesh(self.0)
    }
}

/// Draw a 3d mesh with a material and a transform.
#[inline]
pub fn draw_mesh(_rl: &DrawHandle3D, mesh: &Mesh, material: &Material, transform: Matrix) {
    unsafe { ffi::DrawMesh(mesh.0, *material.get_ffi(), transform) }
}
/// Draw multiple mesh instances with the same material and different transforms.
/// 
/// Note that shaders need to be made specifically for instanced rendering.
#[inline]
pub fn draw_mesh_instanced(_rl: &DrawHandle3D, mesh: &Mesh, material: &Material, transforms: &[Matrix]) {
    unsafe { ffi::DrawMeshInstanced(mesh.0, *material.get_ffi(), transforms.as_ptr(), transforms.len() as i32) }
}
