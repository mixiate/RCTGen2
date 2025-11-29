#![allow(unsafe_code)]

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
    pub fn try_new(device: &Device) -> Result<Scene<'_>, Error> {
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
            unsafe { std::slice::from_raw_parts_mut(vertex_buffer.cast::<(f32, f32, f32)>(), positions.len()) };
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
            unsafe { std::slice::from_raw_parts_mut(index_buffer.cast::<(u32, u32, u32)>(), indices.len()) };
        index_buffer.copy_from_slice(indices);

        unsafe { embree4_sys::rtcSetGeometryOccludedFilterFunction(geometry.handle, Some(occlusion_filter)) };

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

        unsafe { embree4_sys::rtcIntersect1(self.scene.handle, &raw mut ray_hit, std::ptr::null_mut()) }

        if ray_hit.hit.geomID == embree4_sys::RTC_INVALID_GEOMETRY_ID {
            return None;
        }

        let mut position = [0.0_f32; 3];
        let interpolate_arguments = embree4_sys::RTCInterpolateArguments {
            geometry: unsafe { embree4_sys::rtcGetGeometry(self.scene.handle, ray_hit.hit.geomID) },
            primID: ray_hit.hit.primID,
            u: ray_hit.hit.u,
            v: ray_hit.hit.v,
            bufferType: embree4_sys::RTCBufferType::VERTEX,
            bufferSlot: 0,
            P: (&raw mut position).cast(),
            dPdu: std::ptr::null_mut(),
            dPdv: std::ptr::null_mut(),
            ddPdudu: std::ptr::null_mut(),
            ddPdvdv: std::ptr::null_mut(),
            ddPdudv: std::ptr::null_mut(),
            valueCount: 3,
        };
        unsafe { embree4_sys::rtcInterpolate(&raw const interpolate_arguments) }

        if ray_hit.hit.geomID == embree4_sys::RTC_INVALID_GEOMETRY_ID {
            None
        } else {
            Some(RayHit {
                geometry_id: ray_hit.hit.geomID,
                primitive_id: ray_hit.hit.primID,
                position,
                u: ray_hit.hit.u,
                v: ray_hit.hit.v,
            })
        }
    }

    pub fn occluded_1(&self, origin: &(f32, f32, f32), direction: &(f32, f32, f32)) -> bool {
        let mut arguments = embree4_sys::RTCOccludedArguments {
            flags: embree4_sys::RTCRayQueryFlags::INCOHERENT,
            feature_mask: embree4_sys::RTCFeatureFlags::RTC_FEATURE_FLAG_ALL,
            context: std::ptr::null_mut(),
            filter: None,
            occluded: None,
        };

        let mut ray = embree4_sys::RTCRay {
            org_x: origin.0,
            org_y: origin.1,
            org_z: origin.2,
            dir_x: direction.0,
            dir_y: direction.1,
            dir_z: direction.2,
            tnear: 1e-5,
            tfar: f32::INFINITY,
            ..Default::default()
        };
        unsafe { embree4_sys::rtcOccluded1(self.scene.handle, &raw mut ray, &raw mut arguments) }
        ray.tfar <= 0.0
    }

    pub fn bounds(&self) -> Bounds {
        let mut bounds = embree4_sys::RTCBounds::default();
        unsafe { embree4_sys::rtcGetSceneBounds(self.scene.handle, &raw mut bounds) }
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
    pub fn try_new(device: &Device, geometry_type: embree4_sys::RTCGeometryType) -> Result<Geometry<'_>, Error> {
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
    pub position: [f32; 3],
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

unsafe extern "C" fn occlusion_filter(args: *const embree4_sys::RTCFilterFunctionNArguments) {
    if unsafe { (*args).N } != 1 {
        return;
    }

    let ray = unsafe { *((*args).ray as *const embree4_sys::RTCRay) };
    let hit = unsafe { *((*args).hit as *const embree4_sys::RTCHit) };
    if hit.Ng_x * ray.dir_x + hit.Ng_y * ray.dir_y + hit.Ng_z * ray.dir_z > 0.0 {
        unsafe { *(*args).valid = 0 };
    }
}
