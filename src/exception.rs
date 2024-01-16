use napi_derive::napi;
use minidump;

#[napi]
pub struct MinidumpException {
  exception: minidump::MinidumpException,
  system_info: minidump::MinidumpSystemInfo,
}

#[napi]
impl MinidumpException {
  // #[napi]
  // pub fn context(&self) { // the `&self` indicates that this is instance method
  //   //
  // }

  // #[napi]
  // pub fn get_crash_address(&self) {
  //   //
  // }

  // #[napi]
  // pub fn get_crashing_thread_id(&self) {

  // }

  /// there's too much enum defines for `get_crash_reason` method exposed in `minidump`
  /// so here we simplify it
  #[napi]
  pub fn get_crash_reason(&self) {

  }

  pub fn new (exception: &minidump::MinidumpException, system_info: &minidump::MinidumpSystemInfo) -> Self {
    // TODO: what's the appropriate way to create a structure that holds borrowed member?
    Self {
      exception,
      system_info,
    }
  }
}
