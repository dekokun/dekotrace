use libc::pid_t;
use nix::sys::ptrace;
use nix::unistd::Pid;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let pid: pid_t = args[1].parse().unwrap();
    println!("{:?}", ptrace::attach(Pid::from_raw(pid)));
    println!("{:?}", ptrace::detach(Pid::from_raw(pid)));
}
