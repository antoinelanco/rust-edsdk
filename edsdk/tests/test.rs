use edsdk::{EdsCameraCommand::*, EdsShutterButton::*, *};
use std::{
    fs::File,
    io::Write,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{sync::Mutex, time};

async fn _download_evf_aux(
    camera_ref: EdsRefWrapper<EdsBaseRef>,
    out_stream_image_ref: EdsRefWrapper<EdsEvfImageRef>,
    out_stream: EdsRefWrapper<EdsStreamRef>,
) -> Result<Vec<u8>, EdsError> {
    eds_download_evf_image(camera_ref, out_stream_image_ref).await?;
    let data = data_from_out_stream(out_stream).await?;
    Ok(data)
}

async fn _download_evf(
    camera_ref: EdsRefWrapper<EdsBaseRef>,
    nb_frame: u64,
) -> Result<(), EdsError> {
    let out_stream = eds_create_memory_stream(0)?;
    let out_stream_image_ref = unsafe { eds_create_evf_image_ref(out_stream.clone()).await? };

    for _ in 0..nb_frame {
        match _download_evf_aux(
            camera_ref.clone(),
            out_stream_image_ref.clone(),
            out_stream.clone(),
        )
        .await
        {
            Ok(data) => {
                println!("{}", data.len())
            }
            Err(err) => eprintln!("{err:?}"),
        }
    }

    eds_release(out_stream_image_ref).await?;
    eds_release(out_stream).await
}

// fn progress_handler(
//     percent: EdsUInt32,
//     _context: Arc<Mutex<ProgressContext>>,
//     _cancel: *mut EdsBool,
// ) -> Result<(), EdsError> {
//     println!("{}", percent);
//     if percent == 100 {
//         println!("Download is done !")
//     }
//     Ok(())
// }

async fn download(in_ref: EdsRefWrapper<EdsBaseRef>) -> Result<Vec<u8>, EdsError> {
    println!("Start Download");
    let dir_info = eds_get_directory_item_info(in_ref.clone()).await?;
    let out_stream = eds_create_memory_stream(dir_info.size)?;
    // let _progress_context = set_progress_callback!(out_stream, progress_handler);
    eds_download(in_ref.clone(), dir_info.size, out_stream.clone()).await?;
    eds_download_complete(in_ref.clone()).await?;
    let data = data_from_out_stream(out_stream.clone()).await?;
    eds_release(out_stream).await?;

    let path = format!("../images/{}", dir_info.get_sz_file_name());
    let mut file = File::create(path).unwrap();
    file.write_all(&data).unwrap();

    println!("End Download");

    Ok(data)
}

async fn get_event(term: Arc<AtomicBool>) {
    while term.load(Ordering::SeqCst) {
        if let Err(err) = eds_get_event() {
            eprintln!("{err:?}")
        }

        time::sleep(Duration::from_millis(50)).await;
    }
}

fn obj_handler(
    in_event: EdsObjectEvent,
    in_ref: EdsRefWrapper<EdsBaseRef>,
    _context: Arc<Mutex<ObjectContext>>,
) -> Result<(), EdsError> {
    tokio::spawn(async move {
        println!("{in_event:?}");
        let res = match in_event {
            EdsObjectEvent::DirItemRequestTransfer => match download(in_ref.clone()).await {
                Ok(_data) => Ok(()),
                Err(err) => {
                    let res = eds_download_cancel(in_ref.clone()).await;
                    eprintln!("{res:?}");
                    Err(err)
                }
            },
            _ => Ok(()),
        };

        match eds_release(in_ref).await {
            Ok(()) => res,
            Err(err) => Err(err),
        }
    });
    Ok(())
}

fn state_handler(
    event: EdsStateEvent,
    event_data: EdsUInt32,
    context: Arc<Mutex<StateContext>>,
) -> Result<(), EdsError> {
    println!("{event:?} : {event_data}");
    match event {
        EdsStateEvent::JobStatusChanged => {
            tokio::spawn(async move { context.lock().await.job_status = event_data });
        }
        _ => (),
    }
    Ok(())
}

fn property_handler(
    event: EdsPropertyEvent,
    property_id: EdsPropertyID,
    event_data: EdsUInt32,
    _context: Arc<Mutex<PropertyContext>>,
) -> Result<(), EdsError> {
    println!("{event:?} : {property_id:?} : {event_data}");
    Ok(())
}

#[tokio::test]
async fn test() -> Result<(), EdsError> {
    init(|| open_cam(core)).await
}

async fn init<T, Fut>(f: T) -> Result<(), EdsError>
where
    T: Fn() -> Fut,
    Fut: Future<Output = Result<(), EdsError>>,
{
    println!("== Initialize sdk ==");
    eds_initialize_sdk()?;
    match f().await {
        Ok(()) => (),
        Err(err) => eprintln!("{:?}", err),
    }
    println!("== Terminate sdk ==");
    eds_terminate_sdk()
}
fn camera_added_handler(_context: Arc<Mutex<CameraAddedContext>>) -> Result<(), EdsError> {
    println!("Camera added");
    Ok(())
}

async fn open_cam<T, Fut>(f: T) -> Result<(), EdsError>
where
    T: Fn(EdsRefWrapper<EdsBaseRef>) -> Fut,
    Fut: Future<Output = Result<(), EdsError>>,
{
    let camera_added_context = Arc::new(Mutex::new(CameraAddedContext {}));
    set_camera_added!(camera_added_context, camera_added_handler);

    let camera_list_ref = eds_get_camera_list()?;
    let num_of_camera = eds_get_child_count(camera_list_ref.clone()).await?;
    assert!(num_of_camera > 0, "No camera found");
    let camera_ref = eds_get_child_at_index(camera_list_ref.clone(), 0).await?;
    eds_release(camera_list_ref.clone()).await?;
    println!("== Open camera session ==");
    eds_open_session(camera_ref.clone()).await?;
    match f(camera_ref.clone()).await {
        Ok(()) => (),
        Err(err) => eprintln!("{:?}", err),
    }
    println!("== Close camera session ==");
    eds_close_session(camera_ref.clone()).await?;
    eds_release(camera_ref).await
}

async fn wait_job(state_contexe: Arc<Mutex<StateContext>>) {
    while state_contexe.lock().await.job_status > 0 {
        time::sleep(Duration::from_secs(1)).await;
    }
}

async fn core(camera_ref: EdsRefWrapper<EdsBaseRef>) -> Result<(), EdsError> {
    let _object_context = set_object_event_handler!(camera_ref, obj_handler);
    let state_context = set_state_event_handler!(camera_ref, state_handler);
    let _property_context = set_property_event_handler!(camera_ref, property_handler);

    set_save_to(camera_ref.clone(), EdsSaveTo::Host).await?;

    let in_capacity = TagEdsCapacity {
        number_of_free_clusters: 0x7FFFFFFF,
        bytes_per_sector: 0x1000,
        reset: true,
    };

    eds_set_capacity(camera_ref.clone(), in_capacity).await?;

    set_evf_depth_of_field_preview(camera_ref.clone(), EdsEvfDepthOfFieldPreview::Off).await?;
    let term = Arc::new(AtomicBool::new(true));
    tokio::spawn(get_event(term.clone()));

    let av = get_av(camera_ref.clone()).await?;
    println!("{av:?}");

    let tv = get_tv(camera_ref.clone()).await?;
    println!("{tv:?}");

    time::sleep(Duration::from_secs(1)).await;

    println!("== Mode photo ==");
    set_mode(camera_ref.clone(), Mode::Photo).await?;

    for _ in 0..2 {
        println!("== Shoot ==");
        eds_send_command(camera_ref.clone(), PressShutterButton, Completely).await?;
        eds_send_command(camera_ref.clone(), PressShutterButton, Off).await?;
        wait_job(state_context.clone()).await;
        time::sleep(Duration::from_secs(2)).await;
    }

    // highgui::destroy_all_windows().unwrap_or_default();

    // set_mode(camera_ref, Mode::Video)?;
    // time::sleep(Duration::from_secs(1)).await;

    // download_evf(camera_ref, 1000)?;

    println!("== Set term false ==");
    term.store(false, Ordering::SeqCst);

    println!("== Wait status job is zero ==");
    wait_job(state_context.clone()).await;

    time::sleep(Duration::from_secs(2)).await;

    Ok(())
}
