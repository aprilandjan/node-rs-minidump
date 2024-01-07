use minidump;
use napi_derive::napi;

/// Miscellaneous process information
///
/// This struct matches the [Microsoft struct][msdn] of the same name.
///
/// [msdn]: https://docs.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_misc_info
#[napi(object)]
pub struct MinidumpMiscInfo {
  // MISC_INFO
  pub size_of_info: u32,
  pub flags1: u32,
  pub process_id: u32,
  pub process_create_time: u32,
  pub process_user_time: u32,
  pub process_kernel_time: u32,
  // MISC_INFO_2
  // TODO: this is somehow ugly, but rs-napi seems not support structured enum, had to combine them all together...
  // see https://github.com/napi-rs/napi-rs/issues/507
  pub processor_max_mhz: Option<u32>,
  pub processor_current_mhz: Option<u32>,
  pub processor_mhz_limit: Option<u32>,
  pub processor_max_idle_state: Option<u32>,
  pub processor_current_idle_state: Option<u32>,
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

impl From<&minidump::MinidumpMiscInfo> for MinidumpMiscInfo {
  fn from(value: &minidump::MinidumpMiscInfo) -> Self {
    match &value.raw {
      minidump::RawMiscInfo::MiscInfo(info) => {MinidumpMiscInfo::from(info)},
      minidump::RawMiscInfo::MiscInfo2(info) => {MinidumpMiscInfo::from(info)},
      minidump::RawMiscInfo::MiscInfo3(info) => {MinidumpMiscInfo::from(info)},
      minidump::RawMiscInfo::MiscInfo4(info) => {MinidumpMiscInfo::from(info)},
      minidump::RawMiscInfo::MiscInfo5(info) => {MinidumpMiscInfo::from(info)},
    }
  }
}

impl From<&minidump::format::MINIDUMP_MISC_INFO> for MinidumpMiscInfo {
  fn from(info: &minidump::format::MINIDUMP_MISC_INFO) -> Self {
    MinidumpMiscInfo {
      // MISC_INFO
      size_of_info: info.size_of_info,
      flags1: info.flags1,
      process_id: info.process_id,
      process_create_time: info.process_create_time,
      process_user_time: info.process_user_time,
      process_kernel_time: info.process_kernel_time,
      // MISC_INFO_2
      processor_max_mhz: None,
      processor_current_mhz: None,
      processor_mhz_limit: None,
      processor_max_idle_state: None,
      processor_current_idle_state: None,
    }
  }
}

impl From<&minidump::format::MINIDUMP_MISC_INFO_2> for MinidumpMiscInfo {
  fn from(info: &minidump::format::MINIDUMP_MISC_INFO_2) -> Self {
    MinidumpMiscInfo {
      // MISC_INFO
      size_of_info: info.size_of_info,
      flags1: info.flags1,
      process_id: info.process_id,
      process_create_time: info.process_create_time,
      process_user_time: info.process_user_time,
      process_kernel_time: info.process_kernel_time,
      // MISC_INFO_2
      processor_max_mhz: Some(info.processor_max_mhz),
      processor_current_mhz: Some(info.processor_current_mhz),
      processor_mhz_limit: Some(info.processor_mhz_limit),
      processor_max_idle_state: Some(info.processor_max_idle_state),
      processor_current_idle_state: Some(info.processor_current_idle_state),
    }
  }
}

impl From<&minidump::format::MINIDUMP_MISC_INFO_3> for MinidumpMiscInfo {
  fn from(info: &minidump::format::MINIDUMP_MISC_INFO_3) -> Self {
    MinidumpMiscInfo {
      // MISC_INFO
      size_of_info: info.size_of_info,
      flags1: info.flags1,
      process_id: info.process_id,
      process_create_time: info.process_create_time,
      process_user_time: info.process_user_time,
      process_kernel_time: info.process_kernel_time,
      // MISC_INFO_2
      processor_max_mhz: Some(info.processor_max_mhz),
      processor_current_mhz: Some(info.processor_current_mhz),
      processor_mhz_limit: Some(info.processor_mhz_limit),
      processor_max_idle_state: Some(info.processor_max_idle_state),
      processor_current_idle_state: Some(info.processor_current_idle_state),
    }
  }
}

impl From<&minidump::format::MINIDUMP_MISC_INFO_4> for MinidumpMiscInfo {
  fn from(info: &minidump::format::MINIDUMP_MISC_INFO_4) -> Self {
    MinidumpMiscInfo {
      // MISC_INFO
      size_of_info: info.size_of_info,
      flags1: info.flags1,
      process_id: info.process_id,
      process_create_time: info.process_create_time,
      process_user_time: info.process_user_time,
      process_kernel_time: info.process_kernel_time,
      // MISC_INFO_2
      processor_max_mhz: Some(info.processor_max_mhz),
      processor_current_mhz: Some(info.processor_current_mhz),
      processor_mhz_limit: Some(info.processor_mhz_limit),
      processor_max_idle_state: Some(info.processor_max_idle_state),
      processor_current_idle_state: Some(info.processor_current_idle_state),
    }
  }
}

impl From<&minidump::format::MINIDUMP_MISC_INFO_5> for MinidumpMiscInfo {
  fn from(info: &minidump::format::MINIDUMP_MISC_INFO_5) -> Self {
    MinidumpMiscInfo {
      // MISC_INFO
      size_of_info: info.size_of_info,
      flags1: info.flags1,
      process_id: info.process_id,
      process_create_time: info.process_create_time,
      process_user_time: info.process_user_time,
      process_kernel_time: info.process_kernel_time,
      // MISC_INFO_2
      processor_max_mhz: Some(info.processor_max_mhz),
      processor_current_mhz: Some(info.processor_current_mhz),
      processor_mhz_limit: Some(info.processor_mhz_limit),
      processor_max_idle_state: Some(info.processor_max_idle_state),
      processor_current_idle_state: Some(info.processor_current_idle_state),
    }
  }
}
