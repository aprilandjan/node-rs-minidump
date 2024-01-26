use napi_derive::napi;
use minidump::{self, CrashReason};

// https://napi.rs/docs/concepts/reference
#[napi]
pub struct JsMinidumpException {
  pub crash_reason: String,
}

impl From<minidump::CrashReason> for JsMinidumpException {
  fn from(reason: minidump::CrashReason) -> Self {
    let crash_reason = match reason {
      CrashReason::MacGeneral(..) => "MacGeneral()",
      CrashReason::MacBadAccessKern(..) => "MacBadAccessKern",
      // TODO: lots of transformation here...
      _ => "Unknown",
    }.to_owned();
    JsMinidumpException {
      crash_reason,
    }
  }
}
