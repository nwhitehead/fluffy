// L1 runtime-check lowering (.design/lower/l1-runtime-checks.md). Self-contained,
// compiles and runs under `rustc`; the always-active contract checks fire on
// violation in every build profile (NOT debug-only).

/// The defined contract-violation behavior of the GENERATED program (not a
/// toolchain panic): the L1 program's intended abort with a structured, legible
/// diagnostic (§2.4 / §6). Always active in every build profile.
fn fluffy_contract_violation(kind: &str, text: &str) -> ! {
    panic!("fluffy L1 contract violation [{kind}]: {text}");
}

/// Always-active check: a plain `if !(cond)` so the contract is enforced in
/// EVERY build profile (a release-stripped assertion would not be; §6 demands
/// every profile).
macro_rules! fluffy_check {
    ($kind:literal, $text:literal, $cond:expr) => {
        if !($cond) {
            fluffy_contract_violation($kind, $text);
        }
    };
}

struct Account {
    balance: u64,
}

impl Account {
    fn well_formed(&self) -> bool {
        self.balance <= 1000000
    }
}

fn deposit(a: Account, amount: u64) -> Account {
    fluffy_check!("req", "a.balance + amount <= 1_000_000", a.balance + amount <= 1000000);
    let result = {
        Account { balance: a.balance + amount }
    };
    fluffy_check!("ens", "result.balance == a.balance + amount", result.balance == a.balance + amount);
    result
}
