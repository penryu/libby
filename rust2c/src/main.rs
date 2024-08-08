#![warn(clippy::pedantic)]
#![deny(clippy::all)]
#![allow(clippy::module_name_repetitions, non_camel_case_types, unexpected_cfgs)]

cfg_if::cfg_if! {
    if #[cfg(all(feature = "shared", feature = "dlopen"))] {
        compile_error!("features `shared` and `dlopen` are mutually exclusive");
    } else if #[cfg(feature = "shared")] {
        #[path = "mp_shared.rs"]
        mod mp;
    } else if #[cfg(feature = "dlopen")] {
        #[path = "mp_dlopen.rs"]
        mod mp;
    } else {
        compile_error!("Please select the `shared` or `dlopen` feature, or see the Makefile.");
    }
}

use anyhow::{Context, Result};

use crate::mp::MpInt;

#[cfg(not(tarpaulin_include))]
fn main() -> Result<()> {
    println!("{}\n", MpInt::gmp_version());

    let native_int: u64 = std::env::args()
        .nth(1)
        .context("missing integer argument")?
        .parse()
        .context("must be a non-negative integer")?;

    let big_int = MpInt::from(native_int);
    let fact = big_int.factorial();

    println!("{big_int}! -> {fact}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::{ensure, Result};

    use super::{main, MpInt};

    #[test]
    fn test_sanity() {
        let one = MpInt::from(1);
        assert_eq!(one, MpInt::from(0).factorial());
        assert_eq!(one, MpInt::from(1).factorial());
        assert_eq!(MpInt::from(120), MpInt::from(5).factorial());
    }

    #[test]
    fn test_identity() {
        let forty_two = MpInt::from(42);
        assert_eq!(&forty_two, &forty_two);
    }

    #[test]
    fn test_the_answer() {
        let answer_bang = "1405006117752879898543142606244511569936384000000000";
        assert_eq!(answer_bang, MpInt::from(42).factorial().to_string());
    }

    #[test]
    fn test_version_string() {
        assert!(MpInt::gmp_version().starts_with("using gmp"));
    }
}
