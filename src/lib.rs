#![deny(clippy::all)]
use napi_derive::napi;
use memmap2::Mmap;

mod streams;

use streams::*;

#[napi]
pub struct JsMinidump {
  /// !IMPORTANT: should keep it `private` to prevent napi convertion
  dump: minidump::Minidump<'static, Mmap>,
}

#[napi]
impl JsMinidump {
  /// custom constructor for napi
  /// see https://napi.rs/docs/concepts/class#custom-constructor
  #[napi(constructor)]
  pub fn new(path: String) -> Result::<Self, napi::Error> {
    let dump = minidump::Minidump::read_path(path).unwrap();

    Ok(JsMinidump {
      dump,
    })
  }

  /// instance method for napi
  /// see https://napi.rs/docs/concepts/class#class-method
  #[napi]
  pub fn get_crashpad_info(&self)-> napi::Result<JsMinidumpCrashpadInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpCrashpadInfo>().unwrap();

    Ok(JsMinidumpCrashpadInfo::from(result))
  }

  #[napi]
  pub fn get_system_info(&self)-> napi::Result<JsMinidumpSystemInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpSystemInfo>().unwrap();

    Ok(JsMinidumpSystemInfo::from(result))
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

  // #[napi]
  pub fn get_module_list(&self)-> napi::Result<JsMinidumpModuleList> {
    let result = &self.dump.get_stream::<minidump::MinidumpModuleList>().unwrap();

    Ok(JsMinidumpModuleList::from(result))
  }
}
