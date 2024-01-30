#![deny(clippy::all)]
use napi_derive::napi;
use memmap2::Mmap;

mod streams;

use streams::*;

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
  pub fn get_crashpad_info(&self)-> napi::Result<JsMinidumpCrashpadInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpCrashpadInfo>();

    match result {
      Ok(info) => Ok(JsMinidumpCrashpadInfo::from(info)),
      Err(err) => Err(napi::Error::from_reason(err.to_string()))
    }
  }

  #[napi]
  pub fn get_system_info(&self)-> napi::Result<JsMinidumpSystemInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpSystemInfo>();

    match result {
      Ok(info) => Ok(JsMinidumpSystemInfo::from(info)),
      Err(err) => Err(napi::Error::from_reason(err.to_string()))
    }
  }

  #[napi]
  pub fn get_misc_info(&self)-> napi::Result<JsMinidumpMiscInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpMiscInfo>().unwrap();

    Ok(JsMinidumpMiscInfo::from(result))
  }

  #[napi]
  pub fn get_exception(&self)-> napi::Result<JsMinidumpException> {
    let exception = &self.dump.get_stream::<minidump::MinidumpException>().unwrap();
    let system_info = &self.dump.get_stream::<minidump::MinidumpSystemInfo>().unwrap();

    let reason = exception.get_crash_reason(system_info.os, system_info.cpu);

    Ok(JsMinidumpException::from(reason))
  }
}
