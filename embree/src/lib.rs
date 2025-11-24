#[derive(Debug)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Embree error")
    }
}

impl std::error::Error for Error {}

pub struct Device {
    handle: embree4_sys::RTCDevice,
}

impl Device {
    pub fn try_new() -> Result<Self, Error> {
        let handle = unsafe { embree4_sys::rtcNewDevice(std::ptr::null_mut()) };
        if handle.is_null() {
            Err(Error)
        } else {
            Ok(Device { handle })
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { embree4_sys::rtcReleaseDevice(self.handle) }
    }
}

pub struct Scene<'a> {
    device: &'a Device,
    handle: embree4_sys::RTCScene,
}

impl Scene<'_> {
    pub fn try_new<'a>(device: &'a Device) -> Result<Scene<'a>, Error> {
        let handle = unsafe { embree4_sys::rtcNewScene(device.handle) };
        if handle.is_null() {
            Err(Error)
        } else {
            Ok(Scene { device, handle })
        }
    }

    pub fn add_geometry(&self, positions: &[(f32, f32, f32)], indices: &[(u32, u32, u32)]) -> Result<(), Error> {
        let geometry = Geometry::try_new(self.device, embree4_sys::RTCGeometryType::TRIANGLE)?;

        let vertex_buffer = unsafe {
            embree4_sys::rtcSetNewGeometryBuffer(
                geometry.handle,
                embree4_sys::RTCBufferType::VERTEX,
                0,
                embree4_sys::RTCFormat::FLOAT3,
                3 * size_of::<f32>(),
                positions.len(),
            )
        };
        if vertex_buffer.is_null() {
            return Err(Error);
        }

        let vertex_buffer =
            unsafe { std::slice::from_raw_parts_mut(vertex_buffer as *mut (f32, f32, f32), positions.len()) };
        vertex_buffer.copy_from_slice(positions);

        let index_buffer = unsafe {
            embree4_sys::rtcSetNewGeometryBuffer(
                geometry.handle,
                embree4_sys::RTCBufferType::INDEX,
                0,
                embree4_sys::RTCFormat::UINT3,
                3 * size_of::<u32>(),
                indices.len(),
            )
        };
        if index_buffer.is_null() {
            return Err(Error);
        }

        let index_buffer =
            unsafe { std::slice::from_raw_parts_mut(index_buffer as *mut (u32, u32, u32), indices.len()) };
        index_buffer.copy_from_slice(indices);

        unsafe {
            embree4_sys::rtcCommitGeometry(geometry.handle);
            embree4_sys::rtcAttachGeometry(self.handle, geometry.handle);
        }

        Ok(())
    }
}

impl Drop for Scene<'_> {
    fn drop(&mut self) {
        unsafe { embree4_sys::rtcReleaseScene(self.handle) }
    }
}

pub struct CommittedScene<'a> {
    scene: Scene<'a>,
}

impl CommittedScene<'_> {
    pub fn intersect_1(&self, origin: &(f32, f32, f32), direction: &(f32, f32, f32)) -> Option<RayHit> {
        let ray = embree4_sys::RTCRay {
            org_x: origin.0,
            org_y: origin.1,
            org_z: origin.2,
            dir_x: direction.0,
            dir_y: direction.1,
            dir_z: direction.2,
            tnear: 0.0,
            tfar: f32::INFINITY,
            ..Default::default()
        };
        let mut ray_hit = embree4_sys::RTCRayHit {
            ray,
            hit: Default::default(),
        };

        unsafe { embree4_sys::rtcIntersect1(self.scene.handle, &mut ray_hit, std::ptr::null_mut()) }

        if ray_hit.hit.geomID != embree4_sys::RTC_INVALID_GEOMETRY_ID {
            Some(RayHit {
                geometry_id: ray_hit.hit.geomID,
                primitive_id: ray_hit.hit.primID,
                u: ray_hit.hit.u,
                v: ray_hit.hit.v,
            })
        } else {
            None
        }
    }

    pub fn bounds(&self) -> Bounds {
        let mut bounds = embree4_sys::RTCBounds::default();
        unsafe { embree4_sys::rtcGetSceneBounds(self.scene.handle, &mut bounds) }
        Bounds {
            lower_x: bounds.lower_x,
            lower_y: bounds.lower_y,
            lower_z: bounds.lower_z,
            upper_x: bounds.upper_x,
            upper_y: bounds.upper_y,
            upper_z: bounds.upper_z,
        }
    }
}

impl Drop for CommittedScene<'_> {
    fn drop(&mut self) {
        unsafe { embree4_sys::rtcReleaseScene(self.scene.handle) }
    }
}

pub fn commit_scene(scene: Scene) -> CommittedScene {
    unsafe { embree4_sys::rtcCommitScene(scene.handle) }
    CommittedScene { scene }
}

struct Geometry<'a> {
    _device: std::marker::PhantomData<&'a Device>,
    handle: embree4_sys::RTCGeometry,
}

impl Geometry<'_> {
    pub fn try_new<'a>(device: &'a Device, geometry_type: embree4_sys::RTCGeometryType) -> Result<Geometry<'a>, Error> {
        let handle = unsafe { embree4_sys::rtcNewGeometry(device.handle, geometry_type) };
        if handle.is_null() {
            Err(Error)
        } else {
            Ok(Geometry {
                _device: std::marker::PhantomData,
                handle,
            })
        }
    }
}

impl Drop for Geometry<'_> {
    fn drop(&mut self) {
        unsafe { embree4_sys::rtcReleaseGeometry(self.handle) }
    }
}

pub struct RayHit {
    pub geometry_id: u32,
    pub primitive_id: u32,
    pub u: f32,
    pub v: f32,
}

pub struct Bounds {
    pub lower_x: f32,
    pub lower_y: f32,
    pub lower_z: f32,
    pub upper_x: f32,
    pub upper_y: f32,
    pub upper_z: f32,
}
