use edsdk::{EdsCameraCommand::*, EdsShutterButton::*, *};
use std::{
    fs::File,
    io::Write,
    slice,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};
use tokio::{sync::Mutex, time};

fn download_evf_aux(
    camera_ref: EdsBaseRef,
    out_stream_image_ref: EdsEvfImageRef,
    out_stream: EdsStreamRef,
) -> Result<Vec<u8>, EdsError> {
    eds_download_evf_image(camera_ref, out_stream_image_ref)?;
    let live_ptr = eds_get_pointer(out_stream)? as *const u8;
    let live_ptr_len = eds_get_length(out_stream)?;
    let data = unsafe { slice::from_raw_parts(live_ptr, live_ptr_len as usize) }.to_vec();
    Ok(data)
}

fn download_evf(camera_ref: EdsBaseRef, nb_frame: u64) -> Result<(), EdsError> {
    let out_stream = eds_create_memory_stream(0)?;
    let out_stream_image_ref = eds_create_evf_image_ref(out_stream)?;

    for _ in 0..nb_frame {
        match download_evf_aux(camera_ref, out_stream_image_ref, out_stream) {
            Ok(data) => {
                println!("get frame");
                let path = "images/".to_string();
                let file_name = format!("{}frame.jpeg", path);

                let _ = File::create(&file_name).map(|mut buffer| buffer.write_all(&data));
            }
            Err(err) => eprintln!("{err:?}"),
        }
    }

    Ok(())
    // eds_release(out_stream_image_ref)?;
    // eds_release(out_stream)
}

fn download(in_ref: EdsBaseRef) -> Result<(), EdsError> {
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
    let path = "images/".to_string();
    let file_name = format!("{}image.jpeg", path);

    File::create(&file_name)
        .map(|mut buffer| buffer.write_all(&data))
        .unwrap()
        .unwrap();
    Ok(())
}

async fn get_event(term: Arc<AtomicBool>) {
    println!("Loop thread id: {:?}", thread::current().id());
    while term.load(Ordering::SeqCst) {
        if let Err(err) = eds_get_event() {
            eprintln!("{err:?}")
        }
        time::sleep(Duration::from_millis(50)).await;
    }
}

enum Mode {
    Video,
    Photo,
}

fn set_mode(camera_ref: EdsBaseRef, mode: Mode) -> Result<(), EdsError> {
    let (evf_mode, output_device) = match mode {
        Mode::Video => (1, EdsEvfOutputDevice::PC),
        Mode::Photo => (0, EdsEvfOutputDevice::Z),
    };
    set_evf_mode(camera_ref, evf_mode)?;
    set_output_device(camera_ref, output_device)
}

fn obj_handler(
    in_event: EdsObjectEvent,
    in_ref: EdsBaseRef,
    _ctx: Arc<Mutex<ObjectContext>>,
) -> EdsError {
    println!("Event obj thread id: {:?}", thread::current().id());

    println!("{in_event:?}");
    match in_event {
        EdsObjectEvent::DirItemRequestTransfer => match download(in_ref) {
            Ok(()) => EdsError::ErrOk,
            Err(err) => err,
        },
        _ => EdsError::ErrOk,
    }
}

fn state_handler(
    event: EdsStateEvent,
    event_data: EdsUInt32,
    context: Arc<Mutex<StateContext>>,
) -> EdsError {
    println!("Event state thread id: {:?}", thread::current().id());

    println!("{event:?} : {event_data}");
    match event {
        EdsStateEvent::JobStatusChanged => {
            tokio::spawn(async move { context.lock().await.job_status = event_data });
        }
        _ => (),
    }
    EdsError::ErrOk
}

#[tokio::main]
async fn main() -> Result<(), EdsError> {
    println!("== Start ==");
    println!("Main thread id: {:?}", thread::current().id());

    let context_status = Arc::new(Mutex::new(StateContext { job_status: 0 }));
    let object_context = Arc::new(Mutex::new(ObjectContext {}));

    println!("== Init ==");
    eds_initialize_sdk()?;
    let camera_list_ref = eds_get_camera_list()?;
    let num_of_camera = eds_get_child_count(camera_list_ref)?;
    assert!(num_of_camera > 0, "No camera found");
    let camera_ref = eds_get_child_at_index(camera_list_ref, 0)?;
    eds_open_session(camera_ref)?;

    set_object_event_handler!(camera_ref, object_context, obj_handler);
    set_state_event_handler!(camera_ref, context_status, state_handler);

    set_save_to(camera_ref, EdsSaveTo::Host)?;

    let in_capacity = TagEdsCapacity {
        number_of_free_clusters: 0x7FFFFFFF,
        bytes_per_sector: 0x1000,
        reset: true,
    };
    eds_set_capacity(camera_ref, in_capacity)?;

    set_evf_depth_of_field_preview(camera_ref, EdsEvfDepthOfFieldPreview::Off)?;
    let term = Arc::new(AtomicBool::new(true));
    tokio::spawn(get_event(term.clone()));

    // let info: TagEdsDeviceInfo = eds_get_device_info(camera_ref)?;
    // println!("{info:?}");

    // let camera_child_count = eds_get_child_count(camera_ref)?;
    // println!("Camera child count {camera_child_count}");

    // let volume_ref = eds_get_child_at_index(camera_ref, 0)?;

    // eds_format_volume(volume_ref)?;

    // let volume_info = eds_get_volume_info(volume_ref)?;
    // println!("{volume_info:?}");

    // eds_close_session(camera_ref)?;
    // exit(0);

    time::sleep(Duration::from_secs(1)).await;

    println!("== Mode photo ==");
    set_mode(camera_ref, Mode::Photo)?;

    println!("== Shoot ==");
    eds_send_command(camera_ref, PressShutterButton, Completely.into())?;
    eds_send_command(camera_ref, PressShutterButton, Off.into())?;

    time::sleep(Duration::from_secs(1)).await;

    // println!("== Mode video ==");
    // set_mode(camera_ref, Mode::Video)?;

    // println!("== Download evf ==");
    // download_evf(camera_ref, 1000)?;

    println!("== Set term false ==");
    term.store(false, Ordering::SeqCst);

    println!("== Wait status job is zero ==");
    while context_status.lock().await.job_status > 0 {
        time::sleep(Duration::from_secs(1)).await;
    }
    println!("== Close camera session ==");
    eds_close_session(camera_ref)?;
    // println!("== End ==");
    // eds_terminate_sdk()?;
    Ok(())
}
