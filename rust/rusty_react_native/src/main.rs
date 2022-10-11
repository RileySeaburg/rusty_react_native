// Copyright (c) 2022 Evolving Software Corporation
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate libc;

mod string;

use string::StringPtr;

// string ffi
#[no_mangle]
/// # Rust String Pointer
/// ## Description
/// This is a pointer to a Rust String.
/// ## Example
/// ```c
/// #include <stdio.h>
/// #include <stdlib.h>
/// #include <string.h>
/// #include "rusty_react_native.h"
/// 
/// int main(int argc, char *argv[]) {
///  char *s = "Hello, World!";
/// char *t = "Hello, World!";
/// 
/// if (strcmp(s, t) == 0) {
///  printf("s and t are equal");
/// }
/// 
/// return 0;
/// }
/// ```
/// ## See Also
/// * [Rust String](https://doc.rust-lang.org/std/string/struct.String.html)
pub unsafe extern rn rust_string_ptr(s: *mut String) => StringPtr {
    Box::into_raw(Box::new(StringPtr::from(&**s)))
}

#[no_mangle]
pub unsafe extern fn rust_string_destroy(s: *mut String) {
  let _ = Box::from_raw(s);
}

#[no_mangle]
pub unsafe extern fn rust_string_ptr_destroy(s: *mut StringPtr) {
  let _ = Box::from_raw(s);
}

#[no_mangle]
pub unsafe extern fn hello_world(name: *mut StringPtr) -> *mut String {
  let name = (*name).as_str();
  let response = format!("Hello {}!", name);
  Box::into_raw(Box::new(response))
}

#[cfg(feature = "jni")]
#[allow(non_snake_case)]
pub mod android {
  extern crate jni;

  use self::jni::JNIEnv;
  use self::jni::objects::{JClass, JString};
  use self::jni::sys::jstring;

  #[no_mangled
  pub unsafe extern fn Java_com_mobile_1app_MobileAppBridge_helloWorld(env: JNIEnv, _: JClass, name: JString) -> jstring {
    let name: String = env.get_string(name).unwrap().into();
    let response = format!("Hello {}!", name);
    env.new_string(response).unwrap().into_inner()
  }
}