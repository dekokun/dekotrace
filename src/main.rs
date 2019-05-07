extern crate nix;
use nix::sys::ptrace;
use nix::unistd::Pid;
fn main() {
    println!("{:?}", ptrace::attach(Pid::this()));
    println!("{:?}", ptrace::detach(Pid::this()));
}
