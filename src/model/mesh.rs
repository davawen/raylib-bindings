use super::{DrawHandle3D, Material};

use crate::ffi;
use crate::prelude::{Vector3, Image, Matrix};

#[derive(Debug)]
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

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe { ffi::UnloadMesh(self.0) }
    }
}

impl DrawHandle3D {
    /// Draw a 3d mesh with a material and a transform.
    #[inline]
    pub fn mesh(&mut self, mesh: &Mesh, material: &Material, transform: Matrix) {
        unsafe { ffi::DrawMesh(mesh.0, *material.get_ffi(), transform) }
    }
    /// Draw multiple mesh instances with the same material and different transforms.
    /// 
    /// Note that shaders need to be made specifically for instanced rendering.
    #[inline]
    pub fn mesh_instanced(&mut self, mesh: &Mesh, material: &Material, transforms: &[Matrix]) {
        unsafe { ffi::DrawMeshInstanced(mesh.0, *material.get_ffi(), transforms.as_ptr(), transforms.len() as i32) }
    }
}

