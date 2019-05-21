use libc::pid_t;
use nix::sys::ptrace;
use nix::unistd::Pid;
fn main() {
    let pid: pid_t = 19426;
    println!("{:?}", ptrace::attach(Pid::from_raw(pid)));
    println!("{:?}", ptrace::detach(Pid::from_raw(pid)));
}
