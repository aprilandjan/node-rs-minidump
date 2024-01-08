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
  // MISC_INFO_3
  pub process_integrity_level: Option<u32>,
  pub process_execute_flags: Option<u32>,
  pub protected_process: Option<u32>,
  pub time_zone_id: Option<u32>,
  pub time_zone: Option<MinidumpMiscInfoTimeZone>, // minidump::format::TIME_ZONE_INFORMATION,
  // // MISC_INFO_4
  // // pub build_string: [u16; 260], // MAX_PATH
  // // pub dbg_bld_str: [u16; 40],
  // // MISC_INFO_5
  // // pub xstate_data: XSTATE_CONFIG_FEATURE_MSC_INFO,
  // pub process_cookie: u32,
}

/// Settings for a time zone
///
/// This struct matches the [Microsoft struct][msdn] of the same name.
///
/// [msdn]: https://docs.microsoft.com/en-us/windows/win32/api/timezoneapi/ns-timezoneapi-time_zone_information
#[napi(object)]
pub struct MinidumpMiscInfoTimeZone {
  pub bias: i32,
  pub standard_name: Vec<u16>, // [u16, 32]. it is in fact the array type, see https://doc.rust-lang.org/std/primitive.array.html
  pub standard_date: MinidumpMiscInfoSystemTime,
  pub standard_bias: i32,
  pub daylight_name: Vec<u16>, // [u16; 32],
  pub daylight_date: MinidumpMiscInfoSystemTime,
  pub daylight_bias: i32,
}

/// A date and time
///
/// This struct matches the [Microsoft struct][msdn] of the same name.
///
/// [msdn]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms724950(v=vs.85).aspx
#[napi(object)]
pub struct MinidumpMiscInfoSystemTime {
  pub year: u16,
  pub month: u16,
  pub day_of_week: u16,
  pub day: u16,
  pub hour: u16,
  pub minute: u16,
  pub second: u16,
  pub milliseconds: u16,
}

impl From<&minidump::format::SYSTEMTIME> for MinidumpMiscInfoSystemTime {
  fn from(value: &minidump::format::SYSTEMTIME) -> Self {
    MinidumpMiscInfoSystemTime {
      year: value.year,
      month: value.month,
      day_of_week: value.day_of_week,
      day: value.day,
      hour: value.hour,
      minute: value.minute,
      second: value.second,
      milliseconds: value.milliseconds,
    }
  }
}

impl From<&minidump::format::TIME_ZONE_INFORMATION> for MinidumpMiscInfoTimeZone {
  fn from(value: &minidump::format::TIME_ZONE_INFORMATION) -> Self {
    MinidumpMiscInfoTimeZone {
      bias: value.bias,
      standard_name: value.standard_name.to_vec(),
      standard_date: MinidumpMiscInfoSystemTime::from(&value.standard_date),
      standard_bias: value.standard_bias,
      daylight_name: value.daylight_name.to_vec(),
      daylight_date: MinidumpMiscInfoSystemTime::from(&value.daylight_date),
      daylight_bias: value.daylight_bias,
    }
  }
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
      // MISC_INFO_3
      process_integrity_level: None,
      process_execute_flags: None,
      protected_process: None,
      time_zone_id: None,
      time_zone: None,
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
      // MISC_INFO_3
      process_integrity_level: None,
      process_execute_flags: None,
      protected_process: None,
      time_zone_id: None,
      time_zone: None,
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
      // MISC_INFO_3
      process_integrity_level: Some(info.process_integrity_level),
      process_execute_flags: Some(info.process_execute_flags),
      protected_process: Some(info.protected_process),
      time_zone_id: Some(info.time_zone_id),
      time_zone: Some(MinidumpMiscInfoTimeZone::from(&info.time_zone)),
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
      // MISC_INFO_3
      process_integrity_level: None,
      process_execute_flags: None,
      protected_process: None,
      time_zone_id: None,
      time_zone: None,
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
      // MISC_INFO_3
      process_integrity_level: None,
      process_execute_flags: None,
      protected_process: None,
      time_zone_id: None,
      time_zone: None,
    }
  }
}
