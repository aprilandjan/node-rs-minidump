#![deny(clippy::all)]
use napi_derive::napi;
use memmap2::Mmap;

mod streams;
use streams::*;

fn convert_to_napi_error(err: minidump::Error) -> napi::Error {
  napi::Error::from_reason(err.to_string())
}

#[napi(js_name = "Minidump")]
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
    let dump = minidump::Minidump::read_path(path).map_err(convert_to_napi_error)?;

    Ok(JsMinidump {
      dump,
    })
  }

  /// instance method for napi
  /// see https://napi.rs/docs/concepts/class#class-method
  #[napi]
  pub fn get_crashpad_info(&self)-> napi::Result<JsMinidumpCrashpadInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpCrashpadInfo>().map_err(convert_to_napi_error)?;

    Ok(JsMinidumpCrashpadInfo::from(result))
  }

  #[napi]
  pub fn get_system_info(&self)-> napi::Result<JsMinidumpSystemInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpSystemInfo>().map_err(convert_to_napi_error)?;

    Ok(JsMinidumpSystemInfo::from(result))
  }

  #[napi]
  pub fn get_misc_info(&self)-> napi::Result<JsMinidumpMiscInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpMiscInfo>().map_err(convert_to_napi_error)?;

    Ok(JsMinidumpMiscInfo::from(result))
  }

  #[napi]
  pub fn get_exception(&self)-> napi::Result<JsMinidumpException> {
    let exception = &self.dump.get_stream::<minidump::MinidumpException>().map_err(convert_to_napi_error)?;
    let system_info = &self.dump.get_stream::<minidump::MinidumpSystemInfo>().map_err(convert_to_napi_error)?;
    let reason = exception.get_crash_reason(system_info.os, system_info.cpu);

    Ok(JsMinidumpException::from(reason))
  }

  // #[napi]
  pub fn get_module_list(&self)-> napi::Result<JsMinidumpModuleList> {
    let result = &self.dump.get_stream::<minidump::MinidumpModuleList>().map_err(convert_to_napi_error)?;

    Ok(JsMinidumpModuleList::from(result))
  }
}
