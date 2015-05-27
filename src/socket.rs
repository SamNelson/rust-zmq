use libc::{c_char, c_int};
use std::ffi::CStr;
use std::str;
use std::ffi::CString;

#[link(name = "czmq")]
extern  "C" {
    fn zsocket_bind(socket: *mut ZMQSocket, address: *const c_char) -> i32; // TODO, does rust handle optional/variable arguments?
    fn zsocket_connect(socket: *mut ZMQSocket, address: *const c_char) -> i32; // TODO, does rust handle optional/variable arguments?
    fn zstr_recv(socket: *mut ZMQSocket) -> *const c_char;
    fn zstr_send(socket: *mut ZMQSocket, message: *const c_char) -> c_int;
}

#[repr(C)]
pub struct ZMQSocket;
pub struct Socket{
    pub sock: *mut ZMQSocket,
}

impl Socket {
    pub fn bind(&self, address: &str)-> i32 {
        let address = CString::new(address.as_bytes()).unwrap().as_ptr();
        unsafe {
            zsocket_bind(self.sock, address)
        }
    }

    pub fn connect(&self, address: &str)-> i32 {
        let address = CString::new(address.as_bytes()).unwrap().as_ptr();
        unsafe {
            zsocket_connect(self.sock, address)
        }
    }

    pub fn send(&self, message: &str) {
        let msg = CString::new(message.as_bytes()).unwrap().as_ptr();
 
       unsafe {
            zstr_send(self.sock, msg);
        }
    }

    pub fn recv(&self) -> String {
        unsafe {
            let c_buf = zstr_recv(self.sock);
            str::from_utf8(CStr::from_ptr(c_buf).to_bytes_with_nul()).unwrap().to_string()
        }
    }
}
