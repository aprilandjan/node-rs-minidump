use napi_derive::napi;
use minidump;

// https://napi.rs/docs/concepts/reference
#[napi]
pub struct MinidumpException {

}

#[napi]
impl MinidumpException {
  #[napi]
  pub fn context(&self) { // the `&self` indicates that this is instance method
    //
  }

  #[napi]
  pub fn get_crash_address(&self) {
    //
  }

  #[napi]
  pub fn get_crash_reason(&self) {

  }

  #[napi]
  pub fn get_crashing_thread_id(&self) {

  }
}

impl From<&minidump::MinidumpException<'_>> for MinidumpException {
  fn from(value: &minidump::MinidumpException) -> Self {
    MinidumpException {
      // TODO:
    }
  }
}
