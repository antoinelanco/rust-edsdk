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
        Unknown => Box::new(ptr),
        _ => todo!("Not yet implement {:?}", data_type),
    }
}

pub fn get_setting(
    in_ref: EdsBaseRef,
    prop_id: EdsPropertyID,
    in_param: EdsInt32,
) -> Result<Box<dyn Any>, EdsError> {
    let (data_type, size) = eds_get_property_size(in_ref, prop_id, in_param)?;
    let ptr = eds_get_property_data(in_ref, prop_id, in_param, size)?;
    let data = dyn_cast(data_type, ptr);
    Ok(data)
}

fn set_setting<T: Any + Debug>(
    in_ref: EdsBaseRef,
    in_property_id: EdsPropertyID,
    in_param: EdsInt32,
    value: T,
) -> Result<(), EdsError> {
    let (data_type, in_property_size) = eds_get_property_size(in_ref, in_property_id, in_param)?;
    #[cfg(debug_assertions)]
    assert!(
        type_check(&value, &data_type),
        "{:?} == {:?}",
        value,
        data_type
    );
    let in_property_data = (&value as *const T) as *const EdsVoid;
    eds_set_property_data(
        in_ref,
        in_property_id,
        in_param,
        in_property_size,
        in_property_data,
    )
}

pub fn set_save_to(in_ref: EdsBaseRef, save_to: EdsSaveTo) -> Result<(), EdsError> {
    set_setting::<u32>(in_ref, EdsPropertyID::SaveTo, 0, save_to.into())
}

pub fn set_evf_depth_of_field_preview(
    in_ref: EdsBaseRef,
    evf: EdsEvfDepthOfFieldPreview,
) -> Result<(), EdsError> {
    set_setting::<u32>(in_ref, EdsPropertyID::EvfDepthOfFieldPreview, 0, evf.into())
}

pub fn set_output_device(
    camera_ref: EdsBaseRef,
    output_device: EdsEvfOutputDevice,
) -> Result<(), EdsError> {
    set_setting::<u32>(
        camera_ref,
        EdsPropertyID::EvfOutputDevice,
        0,
        output_device.into(),
    )
}

pub fn set_evf_mode(camera_ref: EdsBaseRef, mode: u32) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::EvfMode, 0, mode)?;
    set_evf_depth_of_field_preview(camera_ref, EdsEvfDepthOfFieldPreview::Off)
}

#[repr(C)]
pub struct StateContext {
    pub job_status: EdsUInt32,
}
#[repr(C)]
pub struct ObjectContext {}

#[repr(C)]
pub struct PropertyContext {}

#[macro_export]
macro_rules! set_object_event_handler {
    ($camera_ref:ident, $ctx:ident, $func:ident) => {{
        use crate::{EdsBaseRef, EdsError, EdsObjectEvent, EdsVoid};
        use std::sync::Arc;
        use tokio::sync::Mutex;

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
            let object_context = Arc::into_raw($ctx.clone()) as *mut EdsVoid;
            eds_set_object_event_handler(
                $camera_ref,
                EdsObjectEvent::All,
                object_handler,
                object_context,
            )?;
        }
    }};
}

#[macro_export]
macro_rules! set_state_event_handler {
    ($camera_ref:ident, $ctx:ident, $func:ident) => {{
        use crate::{EdsBaseRef, EdsError, EdsStateEvent, EdsVoid};
        use std::sync::Arc;
        use tokio::sync::Mutex;

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
            let state_context = Arc::into_raw($ctx.clone()) as *mut EdsVoid;
            eds_set_camera_state_event_handler(
                $camera_ref,
                EdsStateEvent::All,
                state_handler,
                state_context,
            )?;
        }
    }};
}

#[macro_export]
macro_rules! set_property_event_handler {
    ($camera_ref:ident, $ctx:ident, $func:ident) => {{
        use crate::{EdsBaseRef, EdsError, EdsVoid};
        use std::sync::Arc;
        use tokio::sync::Mutex;

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
            let property_context = Arc::into_raw($ctx.clone()) as *mut EdsVoid;
            eds_set_property_event_handler(
                $camera_ref,
                EdsPropertyEvent::All,
                property_handler,
                property_context,
            )?;
        }
    }};
}
