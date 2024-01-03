#![deny(clippy::all)]
use napi_derive::napi;
use memmap2::Mmap;

use crashpad_info::MinidumpCrashpadInfo;
use system_info::MinidumpSystemInfo;
use misc_info::MiscInfo;

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
    let dump_result = minidump::Minidump::read_path(path);

    let dump = match dump_result {
      Ok(dump) => dump,
      Err(_) => {
        return Err(napi::Error::new(
          napi::Status::InvalidArg,
          "read minidump file failed".to_owned(),
        ))
      }
    };

    Ok(Minidump {
      dump,
    })
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
  pub fn get_misc_info(&self)-> napi::Result<MiscInfo> {
    let result = &self.dump.get_stream::<minidump::MinidumpMiscInfo>();

    let misc_info = match result {
      Ok(info) => info,
      Err(_) => {
        return Err(napi::Error::new(
          napi::Status::GenericFailure,
          "get miscInfo stream from dump file failed".to_owned(),
        ))
      }
    };

    println!("misc info: {:?}", misc_info);

    let mut output = MiscInfo {
      size_of_info: None,
    };

    match &misc_info.raw {
      minidump::RawMiscInfo::MiscInfo(info) => {output.size_of_info = Some(info.size_of_info)},
      minidump::RawMiscInfo::MiscInfo2(info) => {output.size_of_info = Some(info.size_of_info)},
      minidump::RawMiscInfo::MiscInfo3(info) => {output.size_of_info = Some(info.size_of_info)},
      minidump::RawMiscInfo::MiscInfo4(info) => {output.size_of_info = Some(info.size_of_info)},
      minidump::RawMiscInfo::MiscInfo5(info) => {output.size_of_info = Some(info.size_of_info)},
    }

    Ok(output)
  }
}
