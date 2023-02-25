use ink_group::{InkGroupError, Member};

/// Verifies all member addresses are unique.
pub fn validate_unique_members(members: &[Member]) -> Result<(), InkGroupError> {
    for (a, b) in members.iter().zip(members.iter().skip(1)) {
        if a.addr == b.addr {
            return Err(InkGroupError::DuplicateMember { member: a.addr });
        }
    }

    Ok(())
}

/// Evaluate `$x:expr` and if not true return `Err($y:expr)`.
///
/// Used as `ensure!(expression_to_ensure, expression_to_return_on_false)`.
#[macro_export]
macro_rules! ensure {
    ( $x:expr, $y:expr $(,)? ) => {{
        if !$x {
            return Err($y.into());
        }
    }};
}
