Here is a Rust binding for the EDSDK. I’m encountering several issues.

First, I get a segfault when the SDK is terminated, but only if I download the captured image. If I don’t download the image, the SDK shuts down cleanly.

Also, when images are not downloaded, the pointer to the image is lost, which eventually causes the camera to stop shooting and display "PC FULL".

Is there a way to free this memory region without accessing the pointer directly, in order to unblock the camera without requiring mechanical intervention?

My tests were performed on Ubuntu with a Canon EOS 1100D.

To understand the issue, I invite you to take a look at the `tests/camera.rs` file.
It's not necessarily useful to look at the main.rs file.


Ok test : `LD_LIBRARY_PATH=native cargo test --package edsdk --test camera --verbose -- ok_test --exact --show-output;` -> Ok

Ko test : `LD_LIBRARY_PATH=native cargo test --package edsdk --test camera --verbose -- ko_test --exact --show-output;` -> Segfault



Small additional remark:

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


Moreover, in `EDSDKTypes.h`, the compilation target `TARGET_OS_LINUX` doesn’t work on Ubuntu (at least on my system). I replaced it with ` __linux__`, which works for me and I believe should work for all Linux systems.