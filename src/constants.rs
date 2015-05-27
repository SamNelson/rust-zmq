// macros from https://github.com/zeromq/zeromq3-x/blob/master/include/zmq.h

/*  Socket types */
pub const PAIR: i32 = 0;
pub const PUB: i32 = 1;
pub const SUB: i32 = 2;
pub const REQ: i32 = 3;
pub const REP: i32 = 4;
pub const DEALER: i32 = 5;
pub const ROUTER: i32 = 6;
pub const PULL: i32 = 7;
pub const PUSH: i32 = 8;
pub const XPUB: i32 = 9;
pub const XSUB: i32 = 10;

/*  Socket options. */
pub const AFFINITY: i32 = 4;
pub const IDENTITY: i32 = 5;
pub const SUBSCRIBE: i32 = 6;
pub const UNSUBSCRIBE: i32 = 7;
pub const RATE: i32 = 8;
pub const RECOVERY_IVL: i32 = 9;
pub const SNDBUF: i32 = 11;
pub const RCVBUF: i32 = 12;
pub const RCVMORE: i32 = 13;
pub const FD: i32 = 14;
pub const EVENTS: i32 = 15;
pub const TYPE: i32 = 16;
pub const LINGER: i32 = 17;
pub const RECONNECT_IVL: i32 = 18;
pub const BACKLOG: i32 = 19;
pub const RECONNECT_IVL_MAX: i32 = 21;
pub const MAXMSGSIZE: i32 = 22;
pub const SNDHWM: i32 = 23;
pub const RCVHWM: i32 = 24;
pub const MULTICAST_HOPS: i32 = 25;
pub const RCVTIMEO: i32 = 27;
pub const SNDTIMEO: i32 = 28;
pub const IPV4ONLY: i32 = 31;
pub const LAST_ENDPOINT: i32 = 32;
pub const ROUTER_MANDATORY: i32 = 33;
pub const TCP_KEEPALIVE: i32 = 34;
pub const TCP_KEEPALIVE_CNT: i32 = 35;
pub const TCP_KEEPALIVE_IDLE: i32 = 36;
pub const TCP_KEEPALIVE_INTVL: i32 = 37;
pub const TCP_ACCEPT_FILTER: i32 = 38;
pub const DELAY_ATTACH_ON_CONNECT: i32 = 39;
pub const XPUB_VERBOSE: i32 = 40;

/*  Message options  */
pub const MORE: i32 = 1;

/*  Send/recv options.  */
pub const DONTWAIT: i32 = 1;
pub const SNDMORE: i32 = 2;