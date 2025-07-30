use std::ffi::CStr;

include!("sdk.rs");

fn type_check<T: Any>(t: &T, data_t: &EdsDataType) -> bool {
    use EdsDataType::*;
    let ty = t.type_id();
    match data_t {
        Unknown => false,
        Bool => ty.eq(&TypeId::of::<EdsBool>()),
        String => ty.eq(&TypeId::of::<std::string::String>()),
        Int8 => ty.eq(&TypeId::of::<EdsInt8>()),
        UInt8 => ty.eq(&TypeId::of::<EdsUInt8>()),
        Int16 => ty.eq(&TypeId::of::<EdsInt16>()),
        UInt16 => ty.eq(&TypeId::of::<EdsUInt16>()),
        Int32 => ty.eq(&TypeId::of::<EdsInt32>()),
        UInt32 => ty.eq(&TypeId::of::<EdsUInt32>()),
        Int64 => ty.eq(&TypeId::of::<EdsInt64>()),
        UInt64 => ty.eq(&TypeId::of::<EdsUInt64>()),
        Float => ty.eq(&TypeId::of::<EdsFloat>()),
        Double => ty.eq(&TypeId::of::<EdsDouble>()),
        ByteBlock => todo!(),
        Rational => todo!(),
        Point => todo!(),
        Rect => todo!(),
        Time => todo!(),
        BoolArray => todo!(),
        Int8Array => todo!(),
        Int16Array => todo!(),
        Int32Array => todo!(),
        Uint8Array => todo!(),
        Uint16Array => todo!(),
        Uint32Array => todo!(),
        RationalArray => todo!(),
        FocusInfo => todo!(),
        PictureStyleDesc => todo!(),
    }
}

fn dyn_cast(data_type: EdsDataType, ptr: *const c_void) -> Box<dyn Any> {
    use EdsDataType::*;
    match data_type {
        Bool => Box::new(ptr as usize != 0),
        Int8 => Box::new(ptr as EdsInt8),
        UInt8 => Box::new(ptr as EdsUInt8),
        Int16 => Box::new(ptr as EdsInt16),
        UInt16 => Box::new(ptr as EdsUInt16),
        Int32 => Box::new(ptr as EdsInt32),
        UInt32 => Box::new(ptr as EdsUInt32),
        Int64 => Box::new(ptr as EdsInt64),
        UInt64 => Box::new(ptr as EdsUInt64),
        Float => {
            let f32_ptr = ptr as *const f32;
            Box::new(unsafe { *f32_ptr } as EdsFloat)
        }
        Double => {
            let f64_ptr = ptr as *const f64;
            Box::new(unsafe { *f64_ptr } as EdsDouble)
        }
        String => {
            let c_str = unsafe { CStr::from_ptr(ptr as *const i8) };
            let str = match c_str.to_str() {
                Ok(str_slice) => str_slice.to_owned(),
                Err(err) => {
                    eprintln!("Fail to dynamique cast ptr in string : {}", err);
                    "".to_owned()
                }
            };
            Box::new(str)
        }
        Unknown => Box::new(ptr),
        _ => todo!("Dyn cast of {:?} not yet implement", data_type),
    }
}

pub fn get_raw_setting<U: 'static>(
    in_ref: EdsBaseRef,
    prop_id: EdsPropertyID,
) -> Result<U, EdsError> {
    let (data_type, size) = eds_get_property_size(in_ref, prop_id, 0)?;
    let ptr = eds_get_property_data(in_ref, prop_id, 0, size)?;
    let boxed = dyn_cast(data_type, ptr);
    let unboxed = boxed
        .downcast::<U>()
        .map_err(|_| EdsError::ErrPropertiesMismatch)?;
    Ok(*unboxed)
}

pub fn get_setting<T, U>(in_ref: EdsBaseRef, prop_id: EdsPropertyID) -> Result<U, EdsError>
where
    U: TryFromPrimitive<Primitive = T>,
    T: 'static,
{
    let (data_type, size) = eds_get_property_size(in_ref, prop_id, 0)?;
    let ptr = eds_get_property_data(in_ref, prop_id, 0, size)?;
    let boxed = dyn_cast(data_type, ptr);
    let unboxed = boxed
        .downcast::<T>()
        .map_err(|_| EdsError::ErrPropertiesMismatch)?;
    U::try_from_primitive(*unboxed).map_err(|_| EdsError::ErrPropertiesMismatch)
}

fn set_setting<T, U>(
    in_ref: EdsBaseRef,
    in_property_id: EdsPropertyID,
    value: U,
) -> Result<(), EdsError>
where
    U: 'static + Into<T> + TryFromPrimitive<Primitive = T>,
    T: 'static + Debug,
{
    let (data_type, in_property_size) = eds_get_property_size(in_ref, in_property_id, 0)?;
    let v: T = value.into();
    #[cfg(debug_assertions)]
    assert!(type_check(&v, &data_type), "{:?} == {:?}", v, data_type);
    let in_property_data = (&v as *const T) as *const EdsVoid;
    eds_set_property_data(
        in_ref,
        in_property_id,
        0,
        in_property_size,
        in_property_data,
    )
}

pub fn set_save_to(in_ref: EdsBaseRef, save_to: EdsSaveTo) -> Result<(), EdsError> {
    set_setting(in_ref, EdsPropertyID::SaveTo, save_to)
}

pub fn set_evf_depth_of_field_preview(
    in_ref: EdsBaseRef,
    evf: EdsEvfDepthOfFieldPreview,
) -> Result<(), EdsError> {
    set_setting(in_ref, EdsPropertyID::EvfDepthOfFieldPreview, evf)
}

pub fn set_output_device(
    camera_ref: EdsBaseRef,
    output_device: EdsEvfOutputDevice,
) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::EvfOutputDevice, output_device)
}

pub fn set_evf_mode(camera_ref: EdsBaseRef, mode: EdsEvfMode) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::EvfMode, mode)?;
    set_evf_depth_of_field_preview(camera_ref, EdsEvfDepthOfFieldPreview::Off)
}

pub fn get_white_balance(camera_ref: EdsBaseRef) -> Result<EdsWhiteBalance, EdsError> {
    get_setting(camera_ref, EdsPropertyID::WhiteBalance)
}
pub fn get_quality(camera_ref: EdsBaseRef) -> Result<EdsImageQuality, EdsError> {
    get_setting(camera_ref, EdsPropertyID::ImageQuality)
}
pub fn get_iso(camera_ref: EdsBaseRef) -> Result<ISO, EdsError> {
    get_setting(camera_ref, EdsPropertyID::ISOSpeed)
}
pub fn get_tv(camera_ref: EdsBaseRef) -> Result<Tv, EdsError> {
    get_setting(camera_ref, EdsPropertyID::Tv)
}
pub fn get_av(camera_ref: EdsBaseRef) -> Result<Av, EdsError> {
    get_setting(camera_ref, EdsPropertyID::Av)
}

pub fn get_all<T>(camera_ref: EdsBaseRef, prop_id: EdsPropertyID) -> Result<Vec<T>, EdsError>
where
    T: TryFrom<i32>,
{
    let desc = eds_get_property_desc(camera_ref, prop_id)?;
    let mut res = vec![];
    for i in 0..desc.num_elements {
        let v = desc.prop_desc[i as usize];
        if let Ok(va) = v.try_into() {
            res.push(va);
        }
    }
    Ok(res)
}
pub fn get_all_quality(camera_ref: EdsBaseRef) -> Result<Vec<EdsImageQuality>, EdsError> {
    get_all(camera_ref, EdsPropertyID::ImageQuality)
}
pub fn get_all_white_balance(camera_ref: EdsBaseRef) -> Result<Vec<EdsWhiteBalance>, EdsError> {
    get_all(camera_ref, EdsPropertyID::WhiteBalance)
}
pub fn get_all_iso(camera_ref: EdsBaseRef) -> Result<Vec<ISO>, EdsError> {
    get_all(camera_ref, EdsPropertyID::ISOSpeed)
}
pub fn get_all_av(camera_ref: EdsBaseRef) -> Result<Vec<Av>, EdsError> {
    get_all(camera_ref, EdsPropertyID::Av)
}
pub fn get_all_tv(camera_ref: EdsBaseRef) -> Result<Vec<Tv>, EdsError> {
    get_all(camera_ref, EdsPropertyID::Tv)
}

pub fn set_quality(camera_ref: EdsBaseRef, value: EdsImageQuality) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::ImageQuality, value)
}
pub fn set_white_balance(camera_ref: EdsBaseRef, value: EdsWhiteBalance) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::WhiteBalance, value)
}
pub fn set_iso(camera_ref: EdsBaseRef, value: ISO) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::ISOSpeed, value)
}
pub fn set_av(camera_ref: EdsBaseRef, value: Av) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::Av, value)
}
pub fn set_tv(camera_ref: EdsBaseRef, value: Tv) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::Tv, value)
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct StateContext {
    pub job_status: EdsUInt32,
}
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct ObjectContext {}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PropertyContext {}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct ProgressContext {}
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct CameraAddedContext {}

#[macro_export]
macro_rules! set_object_event_handler {
    ($camera_ref:ident, $func:ident) => {{
        use crate::{EdsBaseRef, EdsError, EdsObjectEvent, EdsVoid};
        use std::sync::Arc;
        use tokio::sync::Mutex;
        let object_context = Arc::new(Mutex::new(ObjectContext::default()));
        unsafe {
            extern "C" fn wrapper(
                in_event: EdsObjectEvent,
                in_ref: EdsBaseRef,
                context: *mut EdsVoid,
            ) -> EdsError {
                let arc = unsafe { Arc::from_raw(context as *const Mutex<ObjectContext>) };
                let result = $func(in_event, in_ref, arc.clone());
                std::mem::forget(arc);
                result
            }
            let object_handler: EdsObjectEventHandler =
                Some(wrapper as unsafe extern "C" fn(_, _, _) -> _);
            let object_context = Arc::into_raw(object_context.clone()) as *mut EdsVoid;
            eds_set_object_event_handler(
                $camera_ref,
                EdsObjectEvent::All,
                object_handler,
                object_context,
            )?;
        }

        object_context
    }};
}
#[macro_export]
macro_rules! set_state_event_handler {
    ($camera_ref:ident, $func:ident) => {{
        use crate::{EdsBaseRef, EdsError, EdsStateEvent, EdsVoid};
        use std::sync::Arc;
        use tokio::sync::Mutex;
        let state_context = Arc::new(Mutex::new(StateContext::default()));
        unsafe {
            extern "C" fn wrapper(
                in_event: EdsStateEvent,
                event_data: EdsUInt32,
                context: *mut EdsVoid,
            ) -> EdsError {
                let arc = unsafe { Arc::from_raw(context as *const Mutex<StateContext>) };
                let result = $func(in_event, event_data, arc.clone());
                std::mem::forget(arc);
                result
            }
            let state_handler: EdsStateEventHandler =
                Some(wrapper as unsafe extern "C" fn(_, _, _) -> _);
            let state_context = Arc::into_raw(state_context.clone()) as *mut EdsVoid;
            eds_set_camera_state_event_handler(
                $camera_ref,
                EdsStateEvent::All,
                state_handler,
                state_context,
            )?;
        };
        state_context
    }};
}
#[macro_export]
macro_rules! set_property_event_handler {
    ($camera_ref:ident, $func:ident) => {{
        use crate::{EdsBaseRef, EdsError, EdsVoid};
        use std::sync::Arc;
        use tokio::sync::Mutex;
        let property_context = Arc::new(Mutex::new(PropertyContext {}));
        unsafe {
            extern "C" fn wrapper(
                in_event: EdsPropertyEvent,
                id: EdsPropertyID,
                event_data: EdsUInt32,
                context: *mut EdsVoid,
            ) -> EdsError {
                let arc = unsafe { Arc::from_raw(context as *const Mutex<PropertyContext>) };
                let result = $func(in_event, id, event_data, arc.clone());
                std::mem::forget(arc);
                result
            }
            let property_handler: EdsPropertyEventHandler =
                Some(wrapper as unsafe extern "C" fn(_, _, _, _) -> _);
            let property_context = Arc::into_raw(property_context.clone()) as *mut EdsVoid;
            eds_set_property_event_handler(
                $camera_ref,
                EdsPropertyEvent::All,
                property_handler,
                property_context,
            )?;
        };
        property_context
    }};
}
#[macro_export]
macro_rules! set_progress_callback {
    ($camera_ref:ident, $func:ident) => {{
        use crate::{EdsBaseRef, EdsError, EdsVoid};
        use std::sync::Arc;
        use tokio::sync::Mutex;
        let progress_context = Arc::new(Mutex::new(ProgressContext {}));
        unsafe {
            extern "C" fn wrapper(
                percent: EdsUInt32,
                context: *mut EdsVoid,
                cancel: *mut EdsBool,
            ) -> EdsError {
                let arc = unsafe { Arc::from_raw(context as *const Mutex<ProgressContext>) };
                let result = $func(percent, arc.clone(), cancel);
                std::mem::forget(arc);
                result
            }
            let progress_callback: EdsProgressCallback =
                Some(wrapper as unsafe extern "C" fn(_, _, _) -> _);

            let progress_context = Arc::into_raw(progress_context.clone()) as *mut EdsVoid;
            eds_set_progress_callback(
                $camera_ref,
                progress_callback,
                EdsProgressOption::Periodically,
                progress_context,
            )?;
        };
        progress_context
    }};
}
#[macro_export]
macro_rules! set_camera_added {
    ($ctx:ident, $func:ident) => {{
        use crate::{EdsBaseRef, EdsError, EdsVoid};
        use std::sync::Arc;
        use tokio::sync::Mutex;

        unsafe {
            extern "C" fn wrapper(context: *mut EdsVoid) -> EdsError {
                let arc = unsafe { Arc::from_raw(context as *const Mutex<CameraAddedContext>) };
                let result = $func(arc.clone());
                std::mem::forget(arc);
                result
            }
            let camera_added_callback: EdsCameraAddedHandler =
                Some(wrapper as unsafe extern "C" fn(_) -> _);

            let camera_added_context = Arc::into_raw($ctx.clone()) as *mut EdsVoid;

            eds_set_camera_added_handler(camera_added_callback, camera_added_context)?;
        }
    }};
}
