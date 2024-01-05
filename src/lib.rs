#![deny(clippy::all)]
use napi_derive::napi;
use memmap2::Mmap;

use crashpad_info::MinidumpCrashpadInfo;
use system_info::MinidumpSystemInfo;
use misc_info::MinidumpMiscInfo;

// TODO: so, what's appropriate way to declare these 'mod' files?
mod crashpad_info;
mod system_info;
mod misc_info;

#[napi]
pub struct Minidump {
  /// !IMPORTANT: should keep it `private` to prevent napi convertion
  dump: minidump::Minidump<'static, Mmap>,
}

#[napi]
impl Minidump {
  /// custom constructor for napi
  /// see https://napi.rs/docs/concepts/class#custom-constructor
  #[napi(constructor)]
  pub fn new(path: String) -> Result::<Self, napi::Error> {
    let result = minidump::Minidump::read_path(path);

    match result {
      Ok(dump) => Ok(Minidump {
        dump
      }),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  /// instance method for napi
  /// see https://napi.rs/docs/concepts/class#class-method
  #[napi]
  pub fn get_crashpad_info(&self)-> napi::Result<MinidumpCrashpadInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpCrashpadInfo>();

    match result {
      Ok(info) => Ok(MinidumpCrashpadInfo::from(info)),
      Err(err) => Err(napi::Error::from_reason(err.to_string()))
    }
  }

  #[napi]
  pub fn get_system_info(&self)-> napi::Result<MinidumpSystemInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpSystemInfo>();

    match result {
      Ok(info) => Ok(MinidumpSystemInfo::from(info)),
      Err(err) => Err(napi::Error::from_reason(err.to_string()))
    }
  }

  #[napi]
  pub fn get_misc_info(&self)-> napi::Result<MinidumpMiscInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpMiscInfo>();

    match result {
      Ok(info) => Ok(MinidumpMiscInfo::from(info)),
      Err(err) => Err(napi::Error::from_reason(err.to_string()))
    }
  }
}
