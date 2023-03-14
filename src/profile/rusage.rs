use libc::{getrusage, rusage, RUSAGE_CHILDREN};
use serde::Serialize;
use std::{ops::Sub, time::Duration};

#[derive(Serialize, Clone, Copy, Debug)]
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
    pub fn default() -> Rusage {
        Rusage {
            user_time: Duration::new(0, 0),
            system_time: Duration::new(0, 0),
            max_rss: 0,
            ixrss: 0,
            idrss: 0,
            isrss: 0,
            minflt: 0,
            majflt: 0,
            nswap: 0,
            inblock: 0,
            oublock: 0,
            msgsnd: 0,
            msgrcv: 0,
            nsignals: 0,
            nvcsw: 0,
            nivcsw: 0,
        }
    }

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

impl Sub for Rusage {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self <= rhs {
            Rusage::default()
        } else {
            Rusage {
                user_time: self.user_time - rhs.user_time,
                system_time: self.system_time - rhs.system_time,
                max_rss: self.max_rss - rhs.max_rss,
                ixrss: self.ixrss - rhs.ixrss,
                idrss: self.idrss - rhs.idrss,
                isrss: self.isrss - rhs.isrss,
                minflt: self.minflt - rhs.minflt,
                majflt: self.majflt - rhs.majflt,
                nswap: self.nswap - rhs.nswap,
                inblock: self.inblock - rhs.inblock,
                oublock: self.oublock - rhs.oublock,
                msgsnd: self.msgsnd - rhs.msgsnd,
                msgrcv: self.msgrcv - rhs.msgrcv,
                nsignals: self.nsignals - rhs.nsignals,
                nvcsw: self.nvcsw - rhs.nvcsw,
                nivcsw: self.nivcsw - rhs.nivcsw,
            }
        }
    }
}

impl PartialOrd for Rusage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.user_time.cmp(&other.user_time) != std::cmp::Ordering::Equal {
            return Some(self.user_time.cmp(&other.user_time));
        }
        if self.system_time.cmp(&other.system_time) != std::cmp::Ordering::Equal {
            return Some(self.system_time.cmp(&other.system_time));
        }
        if self.max_rss.cmp(&other.max_rss) != std::cmp::Ordering::Equal {
            return Some(self.max_rss.cmp(&other.max_rss));
        }
        if self.ixrss.cmp(&other.ixrss) != std::cmp::Ordering::Equal {
            return Some(self.ixrss.cmp(&other.ixrss));
        }
        if self.idrss.cmp(&other.idrss) != std::cmp::Ordering::Equal {
            return Some(self.idrss.cmp(&other.idrss));
        }
        if self.isrss.cmp(&other.isrss) != std::cmp::Ordering::Equal {
            return Some(self.isrss.cmp(&other.isrss));
        }
        if self.minflt.cmp(&other.minflt) != std::cmp::Ordering::Equal {
            return Some(self.minflt.cmp(&other.minflt));
        }
        if self.majflt.cmp(&other.majflt) != std::cmp::Ordering::Equal {
            return Some(self.majflt.cmp(&other.majflt));
        }
        if self.nswap.cmp(&other.nswap) != std::cmp::Ordering::Equal {
            return Some(self.nswap.cmp(&other.nswap));
        }
        if self.inblock.cmp(&other.inblock) != std::cmp::Ordering::Equal {
            return Some(self.inblock.cmp(&other.inblock));
        }
        if self.oublock.cmp(&other.oublock) != std::cmp::Ordering::Equal {
            return Some(self.oublock.cmp(&other.oublock));
        }
        if self.msgsnd.cmp(&other.msgsnd) != std::cmp::Ordering::Equal {
            return Some(self.msgsnd.cmp(&other.msgsnd));
        }
        if self.msgrcv.cmp(&other.msgrcv) != std::cmp::Ordering::Equal {
            return Some(self.msgrcv.cmp(&other.msgrcv));
        }
        if self.nsignals.cmp(&other.nsignals) != std::cmp::Ordering::Equal {
            return Some(self.nsignals.cmp(&other.nsignals));
        }
        if self.nvcsw.cmp(&other.nvcsw) != std::cmp::Ordering::Equal {
            return Some(self.nvcsw.cmp(&other.nvcsw));
        }
        if self.nivcsw.cmp(&other.nivcsw) != std::cmp::Ordering::Equal {
            return Some(self.nivcsw.cmp(&other.nivcsw));
        }
        Some(std::cmp::Ordering::Equal)
    }
}

impl PartialEq for Rusage {
    fn eq(&self, other: &Self) -> bool {
        self.user_time == other.user_time
            && self.system_time == other.system_time
            && self.max_rss == other.max_rss
            && self.ixrss == other.ixrss
            && self.idrss == other.idrss
            && self.isrss == other.isrss
            && self.minflt == other.minflt
            && self.majflt == other.majflt
            && self.nswap == other.nswap
            && self.inblock == other.inblock
            && self.oublock == other.oublock
            && self.msgsnd == other.msgsnd
            && self.msgrcv == other.msgrcv
            && self.nsignals == other.nsignals
            && self.nvcsw == other.nvcsw
            && self.nivcsw == other.nivcsw
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp_user() {
        let mut a = Rusage::default();
        let mut b = Rusage::default();

        a.user_time = std::time::Duration::from_secs(1);
        b.user_time = std::time::Duration::from_secs(2);

        assert!(a < b);
        assert!(b > a);
        assert!(a != b);
        assert!(a <= b);
        assert!(b >= a);
        assert!(a <= a);
        assert!(b >= b);
    }

    #[test]
    fn test_cmp_system() {
        let mut a = Rusage::default();
        let mut b = Rusage::default();

        a.system_time = std::time::Duration::from_secs(1);
        b.system_time = std::time::Duration::from_secs(2);

        assert!(a < b);
        assert!(b > a);
        assert!(a != b);
        assert!(a <= b);
        assert!(b >= a);
        assert!(a <= a);
        assert!(b >= b);
    }

    #[test]
    fn test_sub() {
        let mut a = Rusage::default();
        let mut b = Rusage::default();

        a.user_time = std::time::Duration::from_secs(1);
        a.system_time = std::time::Duration::from_secs(1);
        a.max_rss = 1;
        a.ixrss = 1;
        a.idrss = 1;
        a.isrss = 1;
        a.minflt = 1;
        a.majflt = 1;
        a.nswap = 1;
        a.inblock = 1;
        a.oublock = 1;
        a.msgsnd = 1;
        a.msgrcv = 1;
        a.nsignals = 1;
        a.nvcsw = 1;
        a.nivcsw = 1;

        b.user_time = std::time::Duration::from_secs(2);
        b.system_time = std::time::Duration::from_secs(2);
        b.max_rss = 2;
        b.ixrss = 2;
        b.idrss = 2;
        b.isrss = 2;
        b.minflt = 2;
        b.majflt = 2;
        b.nswap = 2;
        b.inblock = 2;
        b.oublock = 2;
        b.msgsnd = 2;
        b.msgrcv = 2;
        b.nsignals = 2;
        b.nvcsw = 2;
        b.nivcsw = 2;

        let c = b - a;

        assert_eq!(c.user_time, std::time::Duration::from_secs(1));
        assert_eq!(c.system_time, std::time::Duration::from_secs(1));
        assert_eq!(c.max_rss, 1);
        assert_eq!(c.ixrss, 1);
        assert_eq!(c.idrss, 1);
        assert_eq!(c.isrss, 1);
        assert_eq!(c.minflt, 1);
        assert_eq!(c.majflt, 1);
        assert_eq!(c.nswap, 1);
        assert_eq!(c.inblock, 1);
        assert_eq!(c.oublock, 1);
        assert_eq!(c.msgsnd, 1);
        assert_eq!(c.msgrcv, 1);
        assert_eq!(c.nsignals, 1);
        assert_eq!(c.nvcsw, 1);
        assert_eq!(c.nivcsw, 1);
    }

    #[test]
    fn test_sub_greater() {
        let mut a = Rusage::default();
        let mut b = Rusage::default();

        a.user_time = std::time::Duration::from_secs(2);
        a.system_time = std::time::Duration::from_secs(2);
        a.max_rss = 2;
        a.ixrss = 2;
        a.idrss = 2;
        a.isrss = 2;
        a.minflt = 2;
        a.majflt = 2;
        a.nswap = 2;
        a.inblock = 2;
        a.oublock = 2;
        a.msgsnd = 2;
        a.msgrcv = 2;
        a.nsignals = 2;
        a.nvcsw = 2;
        a.nivcsw = 2;

        b.user_time = std::time::Duration::from_secs(1);
        b.system_time = std::time::Duration::from_secs(1);
        b.max_rss = 1;
        b.ixrss = 1;
        b.idrss = 1;
        b.isrss = 1;
        b.minflt = 1;
        b.majflt = 1;
        b.nswap = 1;
        b.inblock = 1;
        b.oublock = 1;
        b.msgsnd = 1;
        b.msgrcv = 1;
        b.nsignals = 1;
        b.nvcsw = 1;
        b.nivcsw = 1;

        let c = b - a;

        assert_eq!(c.user_time, std::time::Duration::from_secs(0));
        assert_eq!(c.system_time, std::time::Duration::from_secs(0));
        assert_eq!(c.max_rss, 0);
        assert_eq!(c.ixrss, 0);
        assert_eq!(c.idrss, 0);
        assert_eq!(c.isrss, 0);
        assert_eq!(c.minflt, 0);
        assert_eq!(c.majflt, 0);
        assert_eq!(c.nswap, 0);
        assert_eq!(c.inblock, 0);
        assert_eq!(c.oublock, 0);
        assert_eq!(c.msgsnd, 0);
        assert_eq!(c.msgrcv, 0);
        assert_eq!(c.nsignals, 0);
        assert_eq!(c.nvcsw, 0);
        assert_eq!(c.nivcsw, 0);
    }
}
