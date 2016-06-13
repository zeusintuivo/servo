/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! This module contains traits in script used generically in the rest of Servo.
//! The traits are here instead of in script so that these modules won't have
//! to depend on script.

#![feature(custom_derive, plugin)]
#![plugin(heapsize_plugin, plugins, serde_macros)]
#![deny(unsafe_code)]

extern crate app_units;
extern crate canvas_traits;
extern crate devtools_traits;
extern crate euclid;
extern crate gfx_traits;
extern crate heapsize;
extern crate ipc_channel;
extern crate libc;
extern crate msg;
extern crate net_traits;
extern crate offscreen_gl_context;
extern crate profile_traits;
extern crate serde;
extern crate style_traits;
extern crate time;
extern crate url;
extern crate util;

use canvas_traits::{CanvasMsg, FromLayoutMsg, CanvasData};
use ipc_channel::ipc::{IpcReceiver, IpcSender};
use libc::c_void;
use msg::constellation_msg::{FrameId, FrameType, Key, KeyModifiers, KeyState, LoadData};
use msg::constellation_msg::{NavigationDirection, PanicMsg, PipelineId};
use msg::constellation_msg::{PipelineNamespaceId, SubpageId, WindowSizeData};
use msg::constellation_msg::{WebDriverCommandMsg, WindowSizeType};
use msg::webdriver_msg::WebDriverScriptCommand;
use net_traits::ResourceThreads;
use net_traits::bluetooth_thread::BluetoothMethodMsg;
use net_traits::image_cache_thread::ImageCacheThread;
use net_traits::response::HttpsState;
use profile_traits::mem;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use url::Url;
use util::ipc::OptionalOpaqueIpcSender;

pub struct HTMLCanvasData {
    pub ipc_renderer: Option<IpcSender<CanvasMsg>>,
    pub width: u32,
    pub height: u32,
}
