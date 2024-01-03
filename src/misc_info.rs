use napi_derive::napi;

#[napi(object)]
pub struct MiscInfo {
  // MISC_INFO_1
  pub size_of_info: Option<u32>,
  // pub flags1: u32,
  // pub process_id: u32,
  // pub process_create_time: u32,
  // pub process_user_time: u32,
  // pub process_kernel_time: u32,
  // // MISC_INFO_2
  // pub processor_max_mhz: u32,
  // pub processor_current_mhz: u32,
  // pub processor_mhz_limit: u32,
  // pub processor_max_idle_state: u32,
  // pub processor_current_idle_state: u32,
  // // MISC_INFO_3
  // pub process_integrity_level: u32,
  // pub process_execute_flags: u32,
  // pub protected_process: u32,
  // pub time_zone_id: u32,
  // // pub time_zone: minidump::format::TIME_ZONE_INFORMATION,
  // // MISC_INFO_4
  // // pub build_string: [u16; 260], // MAX_PATH
  // // pub dbg_bld_str: [u16; 40],
  // // MISC_INFO_5
  // // pub xstate_data: XSTATE_CONFIG_FEATURE_MSC_INFO,
  // pub process_cookie: u32,
}
