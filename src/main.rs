use std::ffi::CString;

use anyhow::Result;
use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::waitpid;
use nix::unistd::{execv, fork, getpid, getuid, ForkResult};

fn main() -> Result<()> {
    // Start program
    println!("Main PID: {}, User: {}", getpid(), getuid());

    // unshare start
    unshare(
        CloneFlags::CLONE_FS
            | CloneFlags::CLONE_NEWCGROUP
            | CloneFlags::CLONE_NEWIPC
            | CloneFlags::CLONE_NEWNET
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWUSER
            | CloneFlags::CLONE_NEWUTS,
    )
    .expect("failed unshare");

    println!("Under unshare PID: {} User: {}", getpid(), getuid());

    // fork start (for run command)
    match unsafe { fork().expect("failed fork") } {
        ForkResult::Parent { child } => {
            println!(
                "I'm parent. PID: {} User: {} Child: {}",
                getpid(),
                getuid(),
                child
            );

            // wait child process
            waitpid(child, None).expect("failed waitpid");

            println!("exit");
        }
        ForkResult::Child => {
            println!("I'm child. PID: {} User: {}", getpid(), getuid());

            // Run command
            let path = CString::new("/bin/echo").expect("CString::new failed");
            let argv =
                ["echo", "Hello", "World"].map(|s| CString::new(s).expect("CString::new failed"));

            execv(&path, &argv).expect("failed execv");
        }
    }

    Ok(())
}
