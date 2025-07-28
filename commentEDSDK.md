In `EDSDKTypes.h`, lines 940–955, within the `EdsBatteryLevel2` enum, the value `0` is defined three times.

```
/*---------------------------------
 Battery level
---------------------------------*/
typedef enum
{
    kEdsBatteryLevel2_Empty = 0,
    kEdsBatteryLevel2_Error = 0,
    kEdsBatteryLevel2_BCLevel = 0,
    ...

} EdsBatteryLevel2;
```






Hello,

I’m currently interfacing your SDK with Rust on a Linux system. I’ve encountered a problem:

When I take a picture without downloading it, everything works fine.

However, when I download the image and then terminate the SDK, a segmentation fault occurs.

Could you help me understand what might be causing this?

Here is a simplified example:

Work
```
eds_initialize_sdk();
camera = get_camera();
eds_open_session(camera);
set_save_to(Host);
eds_set_capacity(...);
eds_send_command(camera, PressShutterButton, Completely);
eds_send_command(camera, PressShutterButton, Off);
eds_close_session(camera);
eds_terminate_sdk();
```

Segmentation fault 
```
eds_initialize_sdk()?;
camera = get_camera()?;
eds_open_session(camera)?;
set_object_event_handler!(camera_ref, callback)?;
set_save_to(Host)?;
eds_set_capacity(...);
eds_send_command(camera, PressShutterButton, Completely);
eds_send_command(camera, PressShutterButton, Off);
eds_close_session(camera);
eds_terminate_sdk();

callback :
    let dir_info = eds_get_directory_item_info(in_ref)?;
    let out_stream = eds_create_memory_stream(dir_info.size)?;
    eds_download(in_ref, dir_info.size, out_stream)?;
    eds_download_complete(in_ref)?;
    let image_ref = eds_create_image_ref(out_stream)?;
    let image_info = eds_get_image_info(image_ref, EdsImageSource::FullView)?;
    let pointer = eds_get_pointer(out_stream)? as *const u8;
    let length = eds_get_length(out_stream)?;
    let data = unsafe { slice::from_raw_parts(pointer, length as usize) }.to_vec();
    save(data);
```
If you want to see the real code to better understand the issue, I can upload it to a Git repository for you if you'd like.



Best regards,
Antoine