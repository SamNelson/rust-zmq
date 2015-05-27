use socket::ZMQSocket;
use socket::Socket;

#[link(name = "czmq")]
extern  "C" {
    fn zctx_new() -> *mut ZMQContext;
    fn zsocket_new(ctx: *mut ZMQContext,  socket_type: i32) -> *mut ZMQSocket;
}

#[repr(C)]
struct ZMQContext;
pub struct Context {
    ctx: *mut ZMQContext
}

impl Context {

    pub fn new() -> Context {
        let ctx: *mut ZMQContext = unsafe {
            zctx_new()
        };
        Context { ctx: ctx }
    }

    pub fn socket(&mut self, socket_type: i32) -> Socket {
        //let sock: *mut ZMQSocket = unsafe {
        let sock = unsafe {
            zsocket_new(self.ctx, socket_type)
        };
        Socket { sock: sock }
    }

}
