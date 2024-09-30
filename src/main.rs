#![allow(non_upper_case_globals)]

mod utils;

use esp_idf_svc::{log::EspLogger, sys};
use log::info;

use crate::utils::{initialize_nvs, spawn_heap_logger};

fn main() {
  // It is necessary to call this function once.
  // Otherwise some patches to the runtime implemented by esp-idf-sys might not link properly.
  // See https://github.com/esp-rs/esp-idf-template/issues/71
  sys::link_patches();

  // Bind the log crate to the ESP Logging facilities
  EspLogger::initialize_default();

  initialize_nvs();

  info!("Hello, world!");
  spawn_heap_logger();
}
