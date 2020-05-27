mod test;

use super::*;
use std::panic;
use std::sync::atomic::{AtomicIsize, Ordering};
use util::mem_util::*;

static NUM_PASS: AtomicIsize = AtomicIsize::new(0);
static NUM_FAIL: AtomicIsize = AtomicIsize::new(0);

const GREEN_OK: &str = "\x1b[0;32mok\x1b[0m";
const RED_FAILED: &str = "\x1b[0;31mFAILED\x1b[0m";

pub fn unit_test(cmd: usize, func_name_ptr: *const c_char, index: i32) -> Result<isize> {
    let ut_cmd = UTCmd::from_raw(cmd)?;

    match ut_cmd {
        UTCmd::GetCount => {
            let num_all = inventory::iter::<TestCase>.into_iter().count();
            Ok(num_all as isize)
        }
        UTCmd::GetPass => Ok(NUM_PASS.load(Ordering::SeqCst)),
        UTCmd::RunByName => {
            let func_name = unsafe {
                from_user::clone_cstring_safely(func_name_ptr)?
                    .to_string_lossy()
                    .into_owned()
            };
            run_test_by_name(&func_name).map(|_| 0)
        }
        UTCmd::RunByIndex => run_test_by_index(index).map(|_| 0),
    }
}

fn run_test_by_name(name: &str) -> Result<()> {
    let case_option = inventory::iter::<TestCase>
        .into_iter()
        .find(|case| case.name() == name);

    if let Some(case) = case_option {
        run_test(&case);
        Ok(())
    } else {
        eprintln!("The input function is not a unit test.");
        return_errno!(EINVAL);
    }
}

fn run_test_by_index(index: i32) -> Result<()> {
    run_test(
        inventory::iter::<TestCase>
            .into_iter()
            .nth(index as usize)
            .ok_or_else(|| errno!(EINVAL, "invalid index"))?,
    );
    Ok(())
}

fn run_test(test_case: &TestCase) {
    let test_name = test_case.name();
    if panic::catch_unwind(|| test_case.func()()).is_ok() ^ test_case.should_panic() {
        eprintln!("test {} ... {}", test_name, GREEN_OK);
        NUM_PASS.fetch_add(1, Ordering::SeqCst);
    } else {
        eprintln!("test {} ... {}", test_name, RED_FAILED);
        NUM_FAIL.fetch_add(1, Ordering::SeqCst);
    }
}

enum UTCmd {
    GetCount,
    GetPass,
    RunByName,
    RunByIndex,
}

impl UTCmd {
    pub fn from_raw(cmd: usize) -> Result<Self> {
        match cmd {
            0 => Ok(Self::GetCount),
            1 => Ok(Self::GetPass),
            2 => Ok(Self::RunByName),
            3 => Ok(Self::RunByIndex),
            _ => return_errno!(EINVAL, "invalid unit test command"),
        }
    }
}

inventory::collect!(TestCase);

#[derive(Debug)]
pub struct TestCase {
    name: String,
    func: fn() -> (),
    should_panic: bool,
}

impl TestCase {
    pub fn new(name: String, func: fn() -> (), should_panic: bool) -> Self {
        Self {
            name,
            func,
            should_panic,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn func(&self) -> fn() -> () {
        self.func
    }

    pub fn should_panic(&self) -> bool {
        self.should_panic
    }
}
