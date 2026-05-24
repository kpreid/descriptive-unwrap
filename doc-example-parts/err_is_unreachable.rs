use descriptive_unwrap::ResultUnwrapExt as _;
use std::net::IpAddr;

let constant_addr: IpAddr = "192,168.0.1".parse().err_is_unreachable();
