This is a copy of the quad example from gfx.
I'm trying to adapt it to run in VR, as part of my efforts to add VR (XR) support to gfx through OpenXR.

# Current state
At this point, it fails to properly create an XR session because it's lacking required extensions on the vulkan instance that gfx creates.

# How to run the code
This assumes you have a local copy of the patched version of gfx with WIP support for XR, you can find it here: https://github.com/agrande/gfx/tree/gfx-xr
I only tested it with an Oculus Quest, but it should run on PCVR the same way.

## Oculus Quest
I'm compiling it for android with a patched version of cargo-apk (see PR: https://github.com/rust-windowing/android-ndk-rs/pull/138), in order to add the OpenXR loader library to the APK.
You need to download Oculus' OpenXR loader from their developper website to be able to test it on the Oculus Quest, and place it in a `runtime_libs` folder.
Then compile with `cargo-apk run --features vulkan,vr`.

## PCVR
I don't have the proper setup to test this, but it should work if you run with `cargo run --features vulkan,vr`.