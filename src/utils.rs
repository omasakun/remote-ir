use std::{
  thread::{sleep, spawn},
  time::Duration,
};

use esp_idf_svc::sys::*;
use log::{info, warn};

pub fn initialize_nvs() {
  unsafe {
    let result = nvs_flash_init();
    if result == ESP_ERR_NVS_NO_FREE_PAGES || result == ESP_ERR_NVS_NEW_VERSION_FOUND {
      warn!("failed to initialize nvs flash, erasing...");
      esp_nofail!(nvs_flash_erase());
      esp_nofail!(nvs_flash_init());
    } else {
      esp_nofail!(result);
    }
  }
}

pub fn spawn_heap_logger() {
  spawn(move || loop {
    sleep(Duration::from_millis(5000));
    unsafe {
      info!(
        "free heap: {} (min: {})",
        esp_get_free_heap_size(),
        esp_get_minimum_free_heap_size()
      );
    }
  });
}
