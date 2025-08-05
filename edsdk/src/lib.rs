//! # Safety (`unsafe`)
//!
//! The functions marked as `unsafe` in this module call into a C library (SDK).
//! In most cases, the pointers passed as arguments must be obtained from other
//! functions provided by the SDK. These pointers are considered valid as long
//! as they are used in accordance with the SDK’s rules, including:
//!
//! - They must be properly released once they are no longer needed.
//! - They must not be used or modified after being released.
//!
//! In some rare cases, the SDK may expect a pointer to a variable or structure
//! allocated by the user. However, the library usually provides helper functions
//! to allocate or initialize such objects safely.
//!
//! ⚠️ It is the caller's **full responsibility** to uphold these requirements.
//! Failure to do so may result in **undefined behavior** (segfaults, memory corruption, etc.).
//!
//! ## Pointer Type Safety
//!
//! From the SDK's point of view, a pointer is just a raw memory address. The C interface
//! does not enforce strong typing, and Rust’s static analysis cannot guarantee that
//! the pointer types match the SDK’s expectations.
//!
//! For example, `eds_delete_directory_item` expects an `EdsDirectoryItemRef`, which
//! is actually a type alias for `EdsBaseRef`. However, passing an `EdsCameraRef` (also
//! an alias for `EdsBaseRef`) by mistake will compile just fine, since both are the
//! same underlying type.
//!
//! **It is therefore critical to ensure that the correct type of pointer is passed
//! to each function**, as mismatches can lead to SDK-level errors such as
//! "invalid pointer" or undefined behavior.
//!
//! Always consult the SDK documentation and double-check that the pointer being passed
//! is of the exact expected kind.

// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Antoine Lanco
use std::slice;
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

pub async fn get_raw_setting<T>(
    in_ref: EdsRefWrapper<EdsBaseRef>,
    prop_id: EdsPropertyID,
) -> Result<T, EdsError>
where
    T: Default,
{
    let (_data_type, size) = eds_get_property_size(in_ref.clone(), prop_id, 0).await?;
    eds_get_property_data(in_ref, prop_id, 0, size).await
}

pub async fn get_setting<T, U>(
    in_ref: EdsRefWrapper<EdsBaseRef>,
    prop_id: EdsPropertyID,
) -> Result<U, EdsError>
where
    U: TryFromPrimitive<Primitive = T>,
    T: Default,
{
    let val = get_raw_setting(in_ref, prop_id).await?;
    U::try_from_primitive(val).map_err(|_| EdsError::PropertiesMismatch)
}

async fn set_setting<T, U>(
    in_ref: EdsRefWrapper<EdsBaseRef>,
    in_property_id: EdsPropertyID,
    value: U,
) -> Result<(), EdsError>
where
    U: 'static + Into<T> + TryFromPrimitive<Primitive = T>,
    T: 'static + Debug,
{
    let (data_type, in_property_size) =
        eds_get_property_size(in_ref.clone(), in_property_id, 0).await?;
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
    .await
}

pub async fn set_save_to(
    in_ref: EdsRefWrapper<EdsBaseRef>,
    save_to: EdsSaveTo,
) -> Result<(), EdsError> {
    set_setting(in_ref, EdsPropertyID::SaveTo, save_to).await
}

pub async fn set_evf_depth_of_field_preview(
    in_ref: EdsRefWrapper<EdsBaseRef>,
    evf: EdsEvfDepthOfFieldPreview,
) -> Result<(), EdsError> {
    set_setting(in_ref, EdsPropertyID::EvfDepthOfFieldPreview, evf).await
}

pub async fn set_output_device(
    camera_ref: EdsRefWrapper<EdsBaseRef>,
    output_device: EdsEvfOutputDevice,
) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::EvfOutputDevice, output_device).await
}

pub async fn set_evf_mode(
    camera_ref: EdsRefWrapper<EdsBaseRef>,
    mode: EdsEvfMode,
) -> Result<(), EdsError> {
    set_setting(camera_ref.clone(), EdsPropertyID::EvfMode, mode).await?;
    set_evf_depth_of_field_preview(camera_ref, EdsEvfDepthOfFieldPreview::Off).await
}

pub async fn get_white_balance(
    camera_ref: EdsRefWrapper<EdsBaseRef>,
) -> Result<EdsWhiteBalance, EdsError> {
    get_setting(camera_ref, EdsPropertyID::WhiteBalance).await
}
pub async fn get_quality(
    camera_ref: EdsRefWrapper<EdsBaseRef>,
) -> Result<EdsImageQuality, EdsError> {
    get_setting(camera_ref, EdsPropertyID::ImageQuality).await
}
pub async fn get_iso(camera_ref: EdsRefWrapper<EdsBaseRef>) -> Result<EdsISO, EdsError> {
    get_setting(camera_ref, EdsPropertyID::ISOSpeed).await
}
pub async fn get_tv(camera_ref: EdsRefWrapper<EdsBaseRef>) -> Result<EdsTv, EdsError> {
    get_setting(camera_ref, EdsPropertyID::Tv).await
}
pub async fn get_av(camera_ref: EdsRefWrapper<EdsBaseRef>) -> Result<EdsAv, EdsError> {
    get_setting(camera_ref, EdsPropertyID::Av).await
}

pub async fn get_all<T>(
    camera_ref: EdsRefWrapper<EdsBaseRef>,
    prop_id: EdsPropertyID,
) -> Result<Vec<T>, EdsError>
where
    T: TryFrom<i32>,
{
    let desc = eds_get_property_desc(camera_ref, prop_id).await?;
    let mut res = vec![];
    for i in 0..desc.num_elements {
        let v = desc.prop_desc[i as usize];
        if let Ok(va) = v.try_into() {
            res.push(va);
        }
    }
    Ok(res)
}

pub async fn get_all_quality(
    camera_ref: EdsRefWrapper<EdsBaseRef>,
) -> Result<Vec<EdsImageQuality>, EdsError> {
    get_all(camera_ref, EdsPropertyID::ImageQuality).await
}

pub async fn get_all_white_balance(
    camera_ref: EdsRefWrapper<EdsBaseRef>,
) -> Result<Vec<EdsWhiteBalance>, EdsError> {
    get_all(camera_ref, EdsPropertyID::WhiteBalance).await
}

pub async fn get_all_iso(camera_ref: EdsRefWrapper<EdsBaseRef>) -> Result<Vec<EdsISO>, EdsError> {
    get_all(camera_ref, EdsPropertyID::ISOSpeed).await
}

pub async fn get_all_av(camera_ref: EdsRefWrapper<EdsBaseRef>) -> Result<Vec<EdsAv>, EdsError> {
    get_all(camera_ref, EdsPropertyID::Av).await
}

pub async fn get_all_tv(camera_ref: EdsRefWrapper<EdsBaseRef>) -> Result<Vec<EdsTv>, EdsError> {
    get_all(camera_ref, EdsPropertyID::Tv).await
}

pub async fn set_quality(
    camera_ref: EdsRefWrapper<EdsBaseRef>,
    value: EdsImageQuality,
) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::ImageQuality, value).await
}
pub async fn set_white_balance(
    camera_ref: EdsRefWrapper<EdsBaseRef>,
    value: EdsWhiteBalance,
) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::WhiteBalance, value).await
}
pub async fn set_iso(camera_ref: EdsRefWrapper<EdsBaseRef>, value: EdsISO) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::ISOSpeed, value).await
}
pub async fn set_av(camera_ref: EdsRefWrapper<EdsBaseRef>, value: EdsAv) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::Av, value).await
}
pub async fn set_tv(camera_ref: EdsRefWrapper<EdsBaseRef>, value: EdsTv) -> Result<(), EdsError> {
    set_setting(camera_ref, EdsPropertyID::Tv, value).await
}

pub enum Mode {
    Video,
    Photo,
}

pub async fn set_mode(camera_ref: EdsRefWrapper<EdsBaseRef>, mode: Mode) -> Result<(), EdsError> {
    let (evf_mode, output_device) = match mode {
        Mode::Video => (EdsEvfMode::Enable, EdsEvfOutputDevice::PC),
        Mode::Photo => (EdsEvfMode::Disable, EdsEvfOutputDevice::Z),
    };
    set_evf_mode(camera_ref.clone(), evf_mode).await?;
    set_output_device(camera_ref, output_device).await
}

pub async fn data_from_out_stream(
    out_stream: EdsRefWrapper<EdsStreamRef>,
) -> Result<Vec<u8>, EdsError> {
    let wrapped_live_ptr = eds_get_pointer(out_stream.clone()).await?;
    let live_ptr_len = eds_get_length(out_stream).await?;
    let live_ptr = wrapped_live_ptr.0.lock().await.0.0 as *const u8;
    let data = unsafe { slice::from_raw_parts(live_ptr, live_ptr_len as usize).to_vec() };
    Ok(data)
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
        use std::sync::Arc;
        use tokio::sync::Mutex;
        use $crate::{EdsBaseRef, EdsError, EdsObjectEvent, EdsVoid};
        let object_context = Arc::new(Mutex::new(ObjectContext::default()));
        {
            extern "C" fn wrapper(
                in_event: EdsObjectEvent,
                in_ref: EdsBaseRef,
                context: *mut EdsVoid,
            ) -> EdsError {
                let arc = unsafe { Arc::from_raw(context as *const Mutex<ObjectContext>) };
                let result = $func(in_event, EdsRefWrapper::new(in_ref), arc.clone());
                std::mem::forget(arc);
                match result {
                    Ok(()) => EdsError::Ok,
                    Err(err) => err,
                }
            }
            let object_handler: EdsObjectEventHandler =
                Some(wrapper as extern "C" fn(_, _, _) -> _);
            let object_context = Arc::into_raw(object_context.clone()) as *mut EdsVoid;
            eds_set_object_event_handler(
                $camera_ref.clone(),
                EdsObjectEvent::All,
                object_handler,
                object_context,
            )
            .await?;
        }

        object_context
    }};
}
#[macro_export]
macro_rules! set_state_event_handler {
    ($camera_ref:ident, $func:ident) => {{
        use std::sync::Arc;
        use tokio::sync::Mutex;
        use $crate::{EdsBaseRef, EdsError, EdsStateEvent, EdsVoid};
        let state_context = Arc::new(Mutex::new(StateContext::default()));
        {
            extern "C" fn wrapper(
                in_event: EdsStateEvent,
                event_data: EdsUInt32,
                context: *mut EdsVoid,
            ) -> EdsError {
                let arc = unsafe { Arc::from_raw(context as *const Mutex<StateContext>) };
                let result = $func(in_event, event_data, arc.clone());
                std::mem::forget(arc);
                match result {
                    Ok(()) => EdsError::Ok,
                    Err(err) => err,
                }
            }
            let state_handler: EdsStateEventHandler = Some(wrapper as extern "C" fn(_, _, _) -> _);
            let state_context = Arc::into_raw(state_context.clone()) as *mut EdsVoid;
            eds_set_camera_state_event_handler(
                $camera_ref.clone(),
                EdsStateEvent::All,
                state_handler,
                state_context,
            )
            .await?;
        };
        state_context
    }};
}
#[macro_export]
macro_rules! set_property_event_handler {
    ($camera_ref:ident, $func:ident) => {{
        use std::sync::Arc;
        use tokio::sync::Mutex;
        use $crate::{EdsBaseRef, EdsError, EdsVoid};
        let property_context = Arc::new(Mutex::new(PropertyContext {}));
        {
            extern "C" fn wrapper(
                in_event: EdsPropertyEvent,
                id: EdsPropertyID,
                event_data: EdsUInt32,
                context: *mut EdsVoid,
            ) -> EdsError {
                let arc = unsafe { Arc::from_raw(context as *const Mutex<PropertyContext>) };
                let result = $func(in_event, id, event_data, arc.clone());
                std::mem::forget(arc);
                match result {
                    Ok(()) => EdsError::Ok,
                    Err(err) => err,
                }
            }
            let property_handler: EdsPropertyEventHandler =
                Some(wrapper as extern "C" fn(_, _, _, _) -> _);
            let property_context = Arc::into_raw(property_context.clone()) as *mut EdsVoid;
            eds_set_property_event_handler(
                $camera_ref.clone(),
                EdsPropertyEvent::All,
                property_handler,
                property_context,
            )
            .await?;
        };
        property_context
    }};
}
#[macro_export]
macro_rules! set_progress_callback {
    ($camera_ref:ident, $func:ident) => {{
        use std::sync::Arc;
        use tokio::sync::Mutex;
        use $crate::{EdsBaseRef, EdsError, EdsVoid};
        let progress_context = Arc::new(Mutex::new(ProgressContext {}));
        {
            extern "C" fn wrapper(
                percent: EdsUInt32,
                context: *mut EdsVoid,
                cancel: *mut EdsBool,
            ) -> EdsError {
                let arc = unsafe { Arc::from_raw(context as *const Mutex<ProgressContext>) };
                let result = $func(percent, arc.clone(), cancel);
                std::mem::forget(arc);
                match result {
                    Ok(()) => EdsError::Ok,
                    Err(err) => err,
                }
            }
            let progress_callback: EdsProgressCallback =
                Some(wrapper as extern "C" fn(_, _, _) -> _);

            let progress_context = Arc::into_raw(progress_context.clone()) as *mut EdsVoid;

            eds_set_progress_callback(
                $camera_ref.clone(),
                progress_callback,
                EdsProgressOption::Periodically,
                progress_context,
            )
            .await?;
        };
        progress_context
    }};
}
#[macro_export]
macro_rules! set_camera_added {
    ($ctx:ident, $func:ident) => {{
        use std::sync::Arc;
        use tokio::sync::Mutex;
        use $crate::{EdsBaseRef, EdsError, EdsVoid};

        {
            extern "C" fn wrapper(context: *mut EdsVoid) -> EdsError {
                let arc = unsafe { Arc::from_raw(context as *const Mutex<CameraAddedContext>) };
                let result = $func(arc.clone());
                std::mem::forget(arc);
                match result {
                    Ok(()) => EdsError::Ok,
                    Err(err) => err,
                }
            }
            let camera_added_callback: EdsCameraAddedHandler =
                Some(wrapper as extern "C" fn(_) -> _);

            let camera_added_context = Arc::into_raw($ctx.clone()) as *mut EdsVoid;

            unsafe {
                eds_set_camera_added_handler(camera_added_callback, camera_added_context)?;
            }
        }
    }};
}
