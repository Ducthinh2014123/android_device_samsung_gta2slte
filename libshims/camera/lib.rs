/*
 * Copyright (C) 2017 The LineageOS Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![no_std]

use core::ffi::c_void;

// The stock camera HAL looks up these C++ symbols by their Itanium ABI names.
#[export_name = "_ZN7android16CameraParameters7KEY_OISE"]
pub static KEY_OIS: [u8; 4] = *b"ois\0";

#[export_name = "_ZN7android16CameraParameters17KEY_OIS_SUPPORTEDE"]
pub static KEY_OIS_SUPPORTED: [u8; 14] = *b"ois-supported\0";

#[export_name = "_ZN7android16CameraParameters23KEY_SUPPORTED_OIS_MODESE"]
pub static KEY_SUPPORTED_OIS_MODES: [u8; 10] = *b"ois-modes\0";

#[export_name = "_ZN7android16CameraParameters38KEY_SUPPORTED_EFFECT_PREVIEW_FPS_RANGEE"]
pub static KEY_SUPPORTED_EFFECT_PREVIEW_FPS_RANGE: [u8; 28] = *b"effect-available-fps-values\0";

#[export_name = "_ZN7android16CameraParameters13OIS_CENTERINGE"]
pub static OIS_CENTERING: [u8; 14] = *b"ois-centering\0";

#[export_name = "_ZN7android16CameraParameters7OIS_OFFE"]
pub static OIS_OFF: [u8; 4] = *b"off\0";

#[export_name = "_ZN7android16CameraParameters13OIS_ON_SINE_XE"]
pub static OIS_ON_SINE_X: [u8; 13] = *b"ois-on-sin-x\0";

#[export_name = "_ZN7android16CameraParameters13OIS_ON_SINE_YE"]
pub static OIS_ON_SINE_Y: [u8; 13] = *b"ois-on-sin-y\0";

#[export_name = "_ZN7android16CameraParameters12OIS_ON_STILLE"]
pub static OIS_ON_STILL: [u8; 13] = *b"ois-on-still\0";

#[export_name = "_ZN7android16CameraParameters12OIS_ON_VIDEOE"]
pub static OIS_ON_VIDEO: [u8; 13] = *b"ois-on-video\0";

#[export_name = "_ZN7android16CameraParameters11OIS_ON_ZOOME"]
pub static OIS_ON_ZOOM: [u8; 12] = *b"ois-on-zoom\0";

#[export_name = "_ZN7android16CameraParameters26PIXEL_FORMAT_YUV420SP_NV21E"]
pub static PIXEL_FORMAT_YUV420SP_NV21: [u8; 9] = *b"yuv420sp\0";

#[export_name = "_ZNK7android16CameraParameters8getInt64EPKc"]
pub extern "C" fn get_int64(_this: *const c_void, _key: *const u8) -> i32 {
    -1
}
