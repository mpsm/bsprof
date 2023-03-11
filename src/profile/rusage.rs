use libc::{getrusage, rusage, RUSAGE_CHILDREN};
use serde::Serialize;
use std::time::Duration;

#[derive(Serialize)]
pub struct Rusage {
    user_time: Duration,
    system_time: Duration,
    max_rss: u64,
    ixrss: u64,
    idrss: u64,
    isrss: u64,
    minflt: u64,
    majflt: u64,
    nswap: u64,
    inblock: u64,
    oublock: u64,
    msgsnd: u64,
    msgrcv: u64,
    nsignals: u64,
    nvcsw: u64,
    nivcsw: u64,
}

impl Rusage {
    pub fn from_libc_rusage(rusage: &rusage) -> Rusage {
        Rusage {
            user_time: Duration::new(
                rusage.ru_utime.tv_sec as u64,
                (rusage.ru_utime.tv_usec * 1000) as u32,
            ),
            system_time: Duration::new(
                rusage.ru_stime.tv_sec as u64,
                (rusage.ru_stime.tv_usec * 1000) as u32,
            ),
            max_rss: rusage.ru_maxrss as u64,
            ixrss: rusage.ru_ixrss as u64,
            idrss: rusage.ru_idrss as u64,
            isrss: rusage.ru_isrss as u64,
            minflt: rusage.ru_minflt as u64,
            majflt: rusage.ru_majflt as u64,
            nswap: rusage.ru_nswap as u64,
            inblock: rusage.ru_inblock as u64,
            oublock: rusage.ru_oublock as u64,
            msgsnd: rusage.ru_msgsnd as u64,
            msgrcv: rusage.ru_msgrcv as u64,
            nsignals: rusage.ru_nsignals as u64,
            nvcsw: rusage.ru_nvcsw as u64,
            nivcsw: rusage.ru_nivcsw as u64,
        }
    }
}

fn get_rusage() -> rusage {
    let mut rusage = rusage {
        ru_utime: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_maxrss: 0,
        ru_ixrss: 0,
        ru_idrss: 0,
        ru_isrss: 0,
        ru_minflt: 0,
        ru_majflt: 0,
        ru_nswap: 0,
        ru_inblock: 0,
        ru_oublock: 0,
        ru_msgsnd: 0,
        ru_msgrcv: 0,
        ru_nsignals: 0,
        ru_nvcsw: 0,
        ru_nivcsw: 0,
    };
    unsafe {
        getrusage(RUSAGE_CHILDREN, &mut rusage);
    }
    rusage
}

pub fn get_process_rusage() -> Rusage {
    Rusage::from_libc_rusage(&get_rusage())
}
