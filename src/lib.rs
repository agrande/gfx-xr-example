mod app;
use ndk_glue;

#[no_mangle]
#[cfg(target_os = "android")]
unsafe extern "C" fn ANativeActivity_onCreate(
    activity: *mut std::os::raw::c_void,
    saved_state: *mut std::os::raw::c_void,
    saved_state_size: usize,
) {
    ndk_glue::init(
        activity as _,
        saved_state as _,
        saved_state_size as _,
        android_main,
    );
}

pub fn android_main() {
    app::run_app();
}
