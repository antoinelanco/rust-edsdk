use std::{
    ptr::null_mut,
    slice,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use edsdk::{
    EdsBaseRef, EdsCameraCommand, EdsError, EdsEvfOutputDevice, EdsImageSource, EdsObjectEvent,
    EdsObjectEventHandler, EdsSaveTo, EdsShutterButton, EdsVoid, TagEdsCapacity, eds_close_session,
    eds_create_image_ref, eds_create_memory_stream, eds_download, eds_download_complete,
    eds_get_camera_list, eds_get_child_at_index, eds_get_directory_item_info, eds_get_event,
    eds_get_image_info, eds_get_length, eds_get_pointer, eds_initialize_sdk, eds_open_session,
    eds_release, eds_send_command, eds_set_capacity, eds_set_object_event_handler,
    eds_terminate_sdk, set_evf_mode, set_output_device, set_save_to,
};
use tokio::time;

async fn get_event(term: Arc<AtomicBool>) {
    while term.load(Ordering::SeqCst) {
        if let Err(err) = eds_get_event() {
            eprintln!("{err:?}")
        }
        time::sleep(Duration::from_millis(50)).await;
    }
}

fn download(in_ref: EdsBaseRef) -> Result<Vec<u8>, EdsError> {
    let dir_info = eds_get_directory_item_info(in_ref)?;
    let out_stream = eds_create_memory_stream(dir_info.size)?;
    eds_download(in_ref, dir_info.size, out_stream)?;
    eds_download_complete(in_ref)?;
    let image_ref = eds_create_image_ref(out_stream)?;
    let image_info = eds_get_image_info(image_ref, EdsImageSource::FullView)?;
    println!("{image_info:?}");
    let pointer = eds_get_pointer(out_stream)? as *const u8;
    let length = eds_get_length(out_stream)?;
    let data = unsafe { slice::from_raw_parts(pointer, length as usize) }.to_vec();

    eds_release(in_ref)?;
    eds_release(out_stream)?;

    println!("get image");
    Ok(data)
}

extern "C" fn wrapper(
    in_event: EdsObjectEvent,
    in_ref: EdsBaseRef,
    _ctx: *mut EdsVoid,
) -> EdsError {
    println!("{in_event:?}");
    match in_event {
        EdsObjectEvent::DirItemRequestTransfer => match download(in_ref) {
            Ok(data) => {
                println!("{}", data.len());
                EdsError::ErrOk
            }
            Err(err) => err,
        },
        _ => EdsError::ErrOk,
    }
}

#[tokio::test]
/// In this example, I do not download the image, and the SDK termination does not cause a segfault.
async fn ok_test() -> Result<(), EdsError> {
    eds_initialize_sdk()?;
    let camera_list_ref = eds_get_camera_list()?;
    let camera_ref = eds_get_child_at_index(camera_list_ref, 0)?;
    eds_open_session(camera_ref)?;
    set_save_to(camera_ref, EdsSaveTo::Host)?;
    let in_capacity = TagEdsCapacity {
        number_of_free_clusters: 0x7FFFFFFF,
        bytes_per_sector: 0x1000,
        reset: true,
    };
    eds_set_capacity(camera_ref, in_capacity)?;

    set_evf_mode(camera_ref, 0)?;
    set_output_device(camera_ref, EdsEvfOutputDevice::Z)?;

    let term = Arc::new(AtomicBool::new(true));
    tokio::spawn(get_event(term.clone()));

    eds_send_command(
        camera_ref,
        EdsCameraCommand::PressShutterButton,
        EdsShutterButton::Completely.into(),
    )?;
    eds_send_command(
        camera_ref,
        EdsCameraCommand::PressShutterButton,
        EdsShutterButton::Off.into(),
    )?;

    time::sleep(Duration::from_secs(2)).await;
    term.store(false, Ordering::SeqCst);
    time::sleep(Duration::from_secs(2)).await;

    eds_close_session(camera_ref)?;
    eds_terminate_sdk()?;

    Ok(())
}

#[tokio::test]
/// However, in this example, downloading the image results in a segfault when the SDK is terminated.
async fn ko_test() -> Result<(), EdsError> {
    eds_initialize_sdk()?;
    let camera_list_ref = eds_get_camera_list()?;
    let camera_ref = eds_get_child_at_index(camera_list_ref, 0)?;
    eds_open_session(camera_ref)?;
    set_save_to(camera_ref, EdsSaveTo::Host)?;
    let in_capacity = TagEdsCapacity {
        number_of_free_clusters: 0x7FFFFFFF,
        bytes_per_sector: 0x1000,
        reset: true,
    };
    eds_set_capacity(camera_ref, in_capacity)?;

    set_evf_mode(camera_ref, 0)?;
    set_output_device(camera_ref, EdsEvfOutputDevice::Z)?;

    let object_handler: EdsObjectEventHandler = Some(wrapper as unsafe extern "C" fn(_, _, _) -> _);
    eds_set_object_event_handler(camera_ref, EdsObjectEvent::All, object_handler, null_mut())?;

    let term = Arc::new(AtomicBool::new(true));
    tokio::spawn(get_event(term.clone()));

    eds_send_command(
        camera_ref,
        EdsCameraCommand::PressShutterButton,
        EdsShutterButton::Completely.into(),
    )?;
    eds_send_command(
        camera_ref,
        EdsCameraCommand::PressShutterButton,
        EdsShutterButton::Off.into(),
    )?;

    time::sleep(Duration::from_secs(2)).await;
    term.store(false, Ordering::SeqCst);
    time::sleep(Duration::from_secs(2)).await;

    eds_close_session(camera_ref)?;
    eds_terminate_sdk()?; // segfault

    Ok(())
}
