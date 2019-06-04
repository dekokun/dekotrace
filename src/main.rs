use libc::pid_t;
use nix::sys::ptrace;
use nix::unistd::Pid;
use std::env;
use std::thread::sleep;
use std::{thread, time};
fn main() {
    let args: Vec<String> = env::args().collect();
    let pid: pid_t = args[1].parse().unwrap();
    println!("{:?}", ptrace::attach(Pid::from_raw(pid)));
    let ten_secs = time::Duration::from_secs(10);
    thread::sleep(ten_secs);
    println!("{:?}", ptrace::detach(Pid::from_raw(pid)));
}
