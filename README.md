Here is a Rust binding for the EDSDK. I’m encountering several issues.

First, I get a segfault when the SDK is terminated, but only if I download the captured image. If I don’t download the image, the SDK shuts down cleanly.

Also, when images are not downloaded, the pointer to the image is lost, which eventually causes the camera to stop shooting and display "PC FULL".

Is there a way to free this memory region without accessing the pointer directly, in order to unblock the camera without requiring mechanical intervention?

My tests were performed on Ubuntu with a Canon EOS 1100D.

To understand the issue, I invite you to take a look at the `tests/camera.rs` file.
It's not necessarily useful to look at the main.rs file.


Ok test : `LD_LIBRARY_PATH=native cargo test --package edsdk --test camera --verbose -- ok_test --exact --show-output;` -> Ok

Ko test : `LD_LIBRARY_PATH=native cargo test --package edsdk --test camera --verbose -- ko_test --exact --show-output;` -> Segfault