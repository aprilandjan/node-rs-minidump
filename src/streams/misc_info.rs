use minidump;
use napi_derive::napi;
use napi::bindgen_prelude::BigInt;

/// Miscellaneous process information
///
/// This struct matches the [Microsoft struct][msdn] of the same name.
///
/// [msdn]: https://docs.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_misc_info
#[napi(object)]
pub struct JsMinidumpMiscInfo {
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
  // MISC_INFO_4
  pub build_string: Option<Vec<u16>>, // [u16, 260],
  pub dbg_bld_str: Option<Vec<u16>>, // [u16; 40],
  // MISC_INFO_5
  pub xstate_data: Option<MinidumpMiscInfoXStateConfigFeature>,
  pub process_cookie: Option<u32>,
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

#[napi(object)]
pub struct MinidumpMiscInfoXStateFeature {
  pub offset: u32,
  pub size: u32,
}

#[napi(object)]
pub struct MinidumpMiscInfoXStateConfigFeature {
  pub size_of_info: u32,
  pub context_size: u32,
  // how to convert to napi compatible BigInt? https://napi.rs/docs/concepts/values#bigint
  // need to enable 'napi6' in cargo.toml napi feature list
  pub enabled_features: BigInt,
  pub features: Vec<MinidumpMiscInfoXStateFeature>, // [XSTATE_FEATURE; 64],
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

impl From<&minidump::MinidumpMiscInfo> for JsMinidumpMiscInfo {
  fn from(value: &minidump::MinidumpMiscInfo) -> Self {
    match &value.raw {
      minidump::RawMiscInfo::MiscInfo(info) => {JsMinidumpMiscInfo::from(info)},
      minidump::RawMiscInfo::MiscInfo2(info) => {JsMinidumpMiscInfo::from(info)},
      minidump::RawMiscInfo::MiscInfo3(info) => {JsMinidumpMiscInfo::from(info)},
      minidump::RawMiscInfo::MiscInfo4(info) => {JsMinidumpMiscInfo::from(info)},
      minidump::RawMiscInfo::MiscInfo5(info) => {JsMinidumpMiscInfo::from(info)},
    }
  }
}

impl From<&minidump::format::XSTATE_FEATURE> for MinidumpMiscInfoXStateFeature {
  fn from(value: &minidump::format::XSTATE_FEATURE) -> Self {
    MinidumpMiscInfoXStateFeature {
      offset: value.offset,
      size: value.size,
    }
  }
}

impl From<&minidump::format::XSTATE_CONFIG_FEATURE_MSC_INFO> for MinidumpMiscInfoXStateConfigFeature {
  fn from(value: &minidump::format::XSTATE_CONFIG_FEATURE_MSC_INFO) -> Self {
    MinidumpMiscInfoXStateConfigFeature {
      size_of_info: value.size_of_info,
      context_size: value.context_size,
      enabled_features: BigInt::from(value.enabled_features),
      features: value.features.into_iter().map(|feature| MinidumpMiscInfoXStateFeature::from(&feature)).collect(),
    }
  }
}

impl From<&minidump::format::MINIDUMP_MISC_INFO> for JsMinidumpMiscInfo {
  fn from(info: &minidump::format::MINIDUMP_MISC_INFO) -> Self {
    JsMinidumpMiscInfo {
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
      // MISC_INFO_4
      build_string: None,
      dbg_bld_str: None,
      // MISC_INFO_5
      xstate_data: None,
      process_cookie: None,
    }
  }
}

impl From<&minidump::format::MINIDUMP_MISC_INFO_2> for JsMinidumpMiscInfo {
  fn from(info: &minidump::format::MINIDUMP_MISC_INFO_2) -> Self {
    JsMinidumpMiscInfo {
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
      // MISC_INFO_4
      build_string: None,
      dbg_bld_str: None,
      // MISC_INFO_5
      xstate_data: None,
      process_cookie: None,
    }
  }
}

impl From<&minidump::format::MINIDUMP_MISC_INFO_3> for JsMinidumpMiscInfo {
  fn from(info: &minidump::format::MINIDUMP_MISC_INFO_3) -> Self {
    JsMinidumpMiscInfo {
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
      // MISC_INFO_4
      build_string: None,
      dbg_bld_str: None,
      // MISC_INFO_5
      xstate_data: None,
      process_cookie: None,
    }
  }
}

impl From<&minidump::format::MINIDUMP_MISC_INFO_4> for JsMinidumpMiscInfo {
  fn from(info: &minidump::format::MINIDUMP_MISC_INFO_4) -> Self {
    JsMinidumpMiscInfo {
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
      // MISC_INFO_4
      build_string: Some(info.build_string.to_vec()),
      dbg_bld_str: Some(info.dbg_bld_str.to_vec()),
      // MISC_INFO_5
      xstate_data: None,
      process_cookie: None,
    }
  }
}

impl From<&minidump::format::MINIDUMP_MISC_INFO_5> for JsMinidumpMiscInfo {
  fn from(info: &minidump::format::MINIDUMP_MISC_INFO_5) -> Self {

    JsMinidumpMiscInfo {
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
      // MISC_INFO_4
      build_string: Some(info.build_string.to_vec()),
      dbg_bld_str: Some(info.dbg_bld_str.to_vec()),
      // MISC_INFO_5
      xstate_data: Some(MinidumpMiscInfoXStateConfigFeature::from(&info.xstate_data)),
      process_cookie: Some(info.process_cookie),
    }
  }
}
