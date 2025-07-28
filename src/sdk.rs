include!("link.rs");
use std::{
    any::{Any, TypeId},
    ptr::null_mut,
};

fn ret(code: EdsError) -> Result<(), EdsError> {
    match code {
        EdsError::ErrOk => Ok(()),
        err => Err(err),
    }
}

macro_rules! check_call {
    ($call:expr) => {{
        let code: EdsError = unsafe { $call };

        match code {
            EdsError::ErrOk => Ok(()),
            err => {
                println!(
                    "Call to `{}` failed with code {:?}",
                    stringify!($call),
                    code
                );
                Err(err)
            }
        }
    }};
}

pub fn eds_initialize_sdk() -> Result<(), EdsError> {
    check_call!(EdsInitializeSDK())
}
pub fn eds_terminate_sdk() -> Result<(), EdsError> {
    check_call!(EdsTerminateSDK())
}
pub fn eds_release(in_ref: EdsBaseRef) -> Result<(), EdsError> {
    check_call!(EdsRelease(in_ref))
}
pub fn eds_retain(in_ref: EdsBaseRef) -> Result<(), EdsError> {
    check_call!(EdsRetain(in_ref))
}
pub fn eds_get_camera_list() -> Result<EdsCameraListRef, EdsError> {
    let mut out_camera_list_ref = null_mut();
    check_call!(EdsGetCameraList(&mut out_camera_list_ref))?;
    Ok(out_camera_list_ref)
}
pub fn eds_get_child_count(in_ref: EdsCameraListRef) -> Result<EdsUInt32, EdsError> {
    let mut out_count = EdsUInt32::default();
    check_call!(EdsGetChildCount(in_ref, &mut out_count))?;
    Ok(out_count)
}
pub fn eds_get_child_at_index(
    in_ref: EdsBaseRef,
    in_index: EdsInt32,
) -> Result<EdsBaseRef, EdsError> {
    let mut out_ref = null_mut();
    check_call!(EdsGetChildAtIndex(in_ref, in_index, &mut out_ref))?;
    Ok(out_ref)
}
pub fn eds_get_device_info(in_camera_ref: EdsBaseRef) -> Result<EdsDeviceInfo, EdsError> {
    let mut out_device_info = EdsDeviceInfo::default();
    check_call!(EdsGetDeviceInfo(in_camera_ref, &mut out_device_info))?;
    Ok(out_device_info)
}
pub fn eds_get_parent(in_ref: EdsBaseRef) -> Result<EdsBaseRef, EdsError> {
    let mut out_parent_ref = null_mut();
    check_call!(EdsGetParent(in_ref, &mut out_parent_ref))?;
    Ok(out_parent_ref)
}
pub fn eds_get_property_size(
    in_ref: EdsBaseRef,
    in_property_id: EdsPropertyID,
    in_param: EdsInt32,
) -> Result<(EdsDataType, EdsUInt32), EdsError> {
    let mut out_data_type = EdsDataType::default();
    let mut out_size = EdsUInt32::default();
    ret(unsafe {
        EdsGetPropertySize(
            in_ref,
            in_property_id,
            in_param,
            &mut out_data_type,
            &mut out_size,
        )
    })?;
    Ok((out_data_type, out_size))
}
pub fn eds_get_property_data(
    in_ref: EdsBaseRef,
    in_property_id: EdsPropertyID,
    in_param: EdsInt32,
    in_property_size: EdsUInt32,
) -> Result<*mut EdsVoid, EdsError> {
    let out_property_data = null_mut();
    ret(unsafe {
        EdsGetPropertyData(
            in_ref,
            in_property_id,
            in_param,
            in_property_size,
            out_property_data,
        )
    })?;
    Ok(out_property_data)
}
pub fn eds_set_property_data(
    in_ref: EdsBaseRef,
    in_property_id: EdsPropertyID,
    in_param: EdsInt32,
    in_property_size: EdsUInt32,
    in_property_data: *const EdsVoid,
) -> Result<(), EdsError> {
    ret(unsafe {
        EdsSetPropertyData(
            in_ref,
            in_property_id,
            in_param,
            in_property_size,
            in_property_data,
        )
    })
}
pub fn eds_get_property_desc(
    in_ref: EdsBaseRef,
    in_property_id: EdsPropertyID,
) -> Result<EdsPropertyDesc, EdsError> {
    let mut out_property_desc = EdsPropertyDesc::default();
    check_call!(EdsGetPropertyDesc(
        in_ref,
        in_property_id,
        &mut out_property_desc
    ))?;
    Ok(out_property_desc)
}
pub fn eds_open_session(in_camera_ref: EdsCameraRef) -> Result<(), EdsError> {
    check_call!(EdsOpenSession(in_camera_ref))
}
pub fn eds_close_session(in_camera_ref: EdsCameraRef) -> Result<(), EdsError> {
    check_call!(EdsCloseSession(in_camera_ref))
}
pub fn eds_send_command(
    in_camera_ref: EdsCameraRef,
    in_command: EdsCameraCommand,
    in_param: EdsInt32,
) -> Result<(), EdsError> {
    check_call!(EdsSendCommand(in_camera_ref, in_command, in_param))
}
pub fn eds_send_status_command(
    in_camera_ref: EdsCameraRef,
    in_status_command: EdsCameraStatusCommand,
    in_param: EdsInt32,
) -> Result<(), EdsError> {
    check_call!(EdsSendStatusCommand(
        in_camera_ref,
        in_status_command,
        in_param
    ))
}
pub fn eds_set_capacity(
    in_camera_ref: EdsCameraRef,
    in_capacity: EdsCapacity,
) -> Result<(), EdsError> {
    check_call!(EdsSetCapacity(in_camera_ref, in_capacity))
}
pub fn eds_get_volume_info(in_volume_ref: EdsVolumeRef) -> Result<EdsVolumeInfo, EdsError> {
    let mut out_volume_info = EdsVolumeInfo::default();
    check_call!(EdsGetVolumeInfo(in_volume_ref, &mut out_volume_info))?;
    Ok(out_volume_info)
}
pub fn eds_format_volume(in_volume_ref: EdsVolumeRef) -> Result<(), EdsError> {
    check_call!(EdsFormatVolume(in_volume_ref))
}
pub fn eds_get_directory_item_info(
    in_dir_item_ref: EdsDirectoryItemRef,
) -> Result<EdsDirectoryItemInfo, EdsError> {
    let mut out_dir_item_info = EdsDirectoryItemInfo::default();
    check_call!(EdsGetDirectoryItemInfo(
        in_dir_item_ref,
        &mut out_dir_item_info
    ))?;
    Ok(out_dir_item_info)
}
pub fn eds_delete_directory_item(in_dir_item_ref: EdsDirectoryItemRef) -> Result<(), EdsError> {
    check_call!(EdsDeleteDirectoryItem(in_dir_item_ref))
}
pub fn eds_download(
    in_dir_item_ref: EdsDirectoryItemRef,
    in_read_size: EdsUInt64,
    out_stream: EdsStreamRef,
) -> Result<(), EdsError> {
    check_call!(EdsDownload(in_dir_item_ref, in_read_size, out_stream))
}
pub fn eds_download_cancel(in_dir_item_ref: EdsDirectoryItemRef) -> Result<(), EdsError> {
    check_call!(EdsDownloadCancel(in_dir_item_ref))
}
pub fn eds_download_complete(in_dir_item_ref: EdsDirectoryItemRef) -> Result<(), EdsError> {
    check_call!(EdsDownloadComplete(in_dir_item_ref))
}
pub fn eds_download_thumbnail(
    in_dir_item_ref: EdsDirectoryItemRef,
    out_stream: EdsStreamRef,
) -> Result<(), EdsError> {
    check_call!(EdsDownloadThumbnail(in_dir_item_ref, out_stream))
}
pub fn eds_get_attribute(
    in_dir_item_ref: EdsDirectoryItemRef,
) -> Result<EdsFileAttributes, EdsError> {
    let mut out_file_attribute = EdsFileAttributes::default();
    check_call!(EdsGetAttribute(in_dir_item_ref, &mut out_file_attribute))?;
    Ok(out_file_attribute)
}

pub fn eds_set_attribute(
    in_dir_item_ref: EdsDirectoryItemRef,
    in_file_attribute: EdsFileAttributes,
) -> Result<(), EdsError> {
    check_call!(EdsSetAttribute(in_dir_item_ref, in_file_attribute))
}
pub fn eds_set_meta_image(
    in_dir_item_ref: EdsDirectoryItemRef,
    in_meta_type: EdsUInt32,
    in_meta_data_size: EdsUInt32,
    in_meta_data: *const EdsVoid,
) -> Result<(), EdsError> {
    ret(unsafe {
        EdsSetMetaImage(
            in_dir_item_ref,
            in_meta_type,
            in_meta_data_size,
            in_meta_data,
        )
    })
}
pub fn eds_create_file_stream(
    in_file_name: *const EdsChar,
    in_create_disposition: EdsFileCreateDisposition,
    in_desired_access: EdsAccess,
) -> Result<EdsStreamRef, EdsError> {
    let mut out_stream = null_mut();
    ret(unsafe {
        EdsCreateFileStream(
            in_file_name,
            in_create_disposition,
            in_desired_access,
            &mut out_stream,
        )
    })?;
    Ok(out_stream)
}
pub fn eds_create_memory_stream(in_buffer_size: EdsUInt64) -> Result<EdsStreamRef, EdsError> {
    let mut out_stream = null_mut();
    check_call!(EdsCreateMemoryStream(in_buffer_size, &mut out_stream))?;
    Ok(out_stream)
}
pub fn eds_create_file_stream_ex(
    in_file_name: *const EdsChar,
    in_create_disposition: EdsFileCreateDisposition,
    in_desired_access: EdsAccess,
) -> Result<EdsStreamRef, EdsError> {
    let mut out_stream = null_mut();
    ret(unsafe {
        EdsCreateFileStreamEx(
            in_file_name,
            in_create_disposition,
            in_desired_access,
            &mut out_stream,
        )
    })?;
    Ok(out_stream)
}
pub fn eds_create_memory_stream_from_pointer(
    in_user_buffer: *mut EdsVoid,
    in_buffer_size: EdsUInt64,
) -> Result<EdsStreamRef, EdsError> {
    let mut out_stream = null_mut();
    ret(unsafe {
        EdsCreateMemoryStreamFromPointer(in_user_buffer, in_buffer_size, &mut out_stream)
    })?;
    Ok(out_stream)
}
pub fn eds_get_pointer(in_stream: EdsStreamRef) -> Result<*mut EdsVoid, EdsError> {
    let mut out_pointer = null_mut();
    check_call!(EdsGetPointer(in_stream, &mut out_pointer))?;
    Ok(out_pointer)
}
pub fn eds_read(
    in_stream_ref: EdsStreamRef,
    in_read_size: EdsUInt64,
) -> Result<(EdsUInt64, *mut EdsVoid), EdsError> {
    let out_buffer = null_mut();
    let mut out_read_size = EdsUInt64::default();
    check_call!(EdsRead(
        in_stream_ref,
        in_read_size,
        out_buffer,
        &mut out_read_size
    ))?;
    Ok((out_read_size, out_buffer))
}
pub fn eds_write(
    in_stream_ref: EdsStreamRef,
    in_write_size: EdsUInt64,
    in_buffer: *const EdsVoid,
) -> Result<EdsUInt64, EdsError> {
    let mut out_written_size = EdsUInt64::default();
    ret(unsafe {
        EdsWrite(
            in_stream_ref,
            in_write_size,
            in_buffer,
            &mut out_written_size,
        )
    })?;
    Ok(out_written_size)
}
pub fn eds_seek(
    in_stream_ref: EdsStreamRef,
    in_seek_offset: EdsInt64,
    in_seek_origin: EdsSeekOrigin,
) -> Result<(), EdsError> {
    check_call!(EdsSeek(in_stream_ref, in_seek_offset, in_seek_origin))
}
pub fn eds_get_position(in_stream_ref: EdsStreamRef) -> Result<EdsUInt64, EdsError> {
    let mut out_position = EdsUInt64::default();
    check_call!(EdsGetPosition(in_stream_ref, &mut out_position))?;
    Ok(out_position)
}
pub fn eds_get_length(in_stream_ref: EdsStreamRef) -> Result<EdsUInt64, EdsError> {
    let mut out_length = EdsUInt64::default();
    check_call!(EdsGetLength(in_stream_ref, &mut out_length))?;
    Ok(out_length)
}
pub fn eds_copy_data(
    in_stream_ref: EdsStreamRef,
    in_write_size: EdsUInt64,
) -> Result<EdsStreamRef, EdsError> {
    let out_stream_ref = null_mut();
    check_call!(EdsCopyData(in_stream_ref, in_write_size, out_stream_ref))?;
    Ok(out_stream_ref)
}
pub fn eds_set_progress_callback(
    in_ref: EdsBaseRef,
    in_progress_callback: EdsProgressCallback,
    in_progress_option: EdsProgressOption,
    in_context: *mut EdsVoid,
) -> Result<(), EdsError> {
    ret(unsafe {
        EdsSetProgressCallback(in_ref, in_progress_callback, in_progress_option, in_context)
    })
}
pub fn eds_create_image_ref(in_stream_ref: EdsStreamRef) -> Result<EdsImageRef, EdsError> {
    let mut out_image_ref = null_mut();
    check_call!(EdsCreateImageRef(in_stream_ref, &mut out_image_ref))?;
    Ok(out_image_ref)
}
pub fn eds_get_image_info(
    in_image_ref: EdsImageRef,
    in_image_source: EdsImageSource,
) -> Result<EdsImageInfo, EdsError> {
    let mut out_image_info = EdsImageInfo::default();
    check_call!(EdsGetImageInfo(
        in_image_ref,
        in_image_source,
        &mut out_image_info
    ))?;
    Ok(out_image_info)
}
pub fn eds_get_image(
    in_image_ref: EdsImageRef,
    in_image_source: EdsImageSource,
    in_image_type: EdsTargetImageType,
    in_src_rect: EdsRect,
    in_dst_size: EdsSize,
) -> Result<EdsStreamRef, EdsError> {
    let out_stream_ref = null_mut();
    ret(unsafe {
        EdsGetImage(
            in_image_ref,
            in_image_source,
            in_image_type,
            in_src_rect,
            in_dst_size,
            out_stream_ref,
        )
    })?;
    Ok(out_stream_ref)
}
pub fn eds_create_evf_image_ref(in_stream_ref: EdsStreamRef) -> Result<EdsEvfImageRef, EdsError> {
    let mut out_evf_image_ref = null_mut();
    check_call!(EdsCreateEvfImageRef(in_stream_ref, &mut out_evf_image_ref))?;
    Ok(out_evf_image_ref)
}
pub fn eds_download_evf_image(
    in_camera_ref: EdsCameraRef,
    in_evf_image_ref: EdsEvfImageRef,
) -> Result<(), EdsError> {
    check_call!(EdsDownloadEvfImage(in_camera_ref, in_evf_image_ref))
}
pub fn eds_set_camera_added_handler(
    in_camera_added_handler: EdsCameraAddedHandler,
    in_context: *mut EdsVoid,
) -> Result<(), EdsError> {
    check_call!(EdsSetCameraAddedHandler(
        in_camera_added_handler,
        in_context
    ))
}
pub fn eds_set_property_event_handler(
    in_camera_ref: EdsCameraRef,
    in_evnet: EdsPropertyEvent,
    in_property_event_handler: EdsPropertyEventHandler,
    in_context: *mut EdsVoid,
) -> Result<(), EdsError> {
    ret(unsafe {
        EdsSetPropertyEventHandler(
            in_camera_ref,
            in_evnet,
            in_property_event_handler,
            in_context,
        )
    })
}
pub fn eds_set_object_event_handler(
    in_camera_ref: EdsCameraRef,
    in_evnet: EdsObjectEvent,
    in_object_event_handler: EdsObjectEventHandler,
    in_context: *mut EdsVoid,
) -> Result<(), EdsError> {
    ret(unsafe {
        EdsSetObjectEventHandler(in_camera_ref, in_evnet, in_object_event_handler, in_context)
    })
}
pub fn eds_set_camera_state_event_handler(
    in_camera_ref: EdsCameraRef,
    in_evnet: EdsStateEvent,
    in_state_event_handler: EdsStateEventHandler,
    in_context: *mut EdsVoid,
) -> Result<(), EdsError> {
    ret(unsafe {
        EdsSetCameraStateEventHandler(in_camera_ref, in_evnet, in_state_event_handler, in_context)
    })
}
pub fn eds_create_stream(in_stream: *mut EdsIStream) -> Result<EdsStreamRef, EdsError> {
    let mut out_stream_ref = null_mut();
    check_call!(EdsCreateStream(in_stream, &mut out_stream_ref))?;
    Ok(out_stream_ref)
}
pub fn eds_get_event() -> Result<(), EdsError> {
    check_call!(EdsGetEvent())
}
pub fn eds_set_frame_point(
    in_camera_ref: EdsCameraRef,
    in_framepoint: EdsPoint,
    in_lock_af_frame: EdsBool,
) -> Result<(), EdsError> {
    check_call!(EdsSetFramePoint(
        in_camera_ref,
        in_framepoint,
        in_lock_af_frame
    ))
}
