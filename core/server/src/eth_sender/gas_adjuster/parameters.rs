//! `parameters` module provides methods to get configurable parameters of `GasAdjuster`.
//!
//! Currently the following types of parameters are provided:
//! - Maximum gas price renewal interval: interval between updates of the upper limit for
//!   gas price suggested by `GasAdjuster`.
//! - Maximum gas price scale: multiplier to be applied to the average gas price to
//!   calculate the upper limit for gas price in `GasAdjuster`.
//!
//! The module uses a child module `parameters_impl` which contains two implementations
//! for functions declared in module: one for the actual usage, and one for tests.
//! While the actual implementation obtains the values from the environment variables,
//! the test one uses hard-coded values for better test behavior predictability.

// Built-in deps.
use std::time::Duration;

/// Obtains the interval for renewing the maximum gas price.
///
/// This value is not cached internally, as it may be changed for the already running
/// server by an administrator. This may be required if existing settings aren't flexible
/// enough to match the current network price.
pub fn get_max_price_interval() -> Duration {
    parameters_impl::get_max_price_interval()
}

/// Obtains the scaling factor for the maximum gas price.
///
/// This value is not cached internally, as it may be changed for the already running
/// server by an administrator. This may be required if existing settings aren't flexible
/// enough to match the current network price.
pub fn get_max_price_scale() -> f64 {
    parameters_impl::get_max_price_scale()
}

// Actual methods implementation for non-test purposes.
#[cfg(not(test))]
mod parameters_impl {
    // Built-in deps.
    use std::time::Duration;
    // Workspace deps
    use models::config_options::parse_env;

    /// Name of the environment variable responsible for the `max_gas_price` renewing interval.
    const MAX_GAS_PRICE_RENEWAL_INTERVAL_VAR: &'static str = "ETH_MAX_GAS_PRICE_RENEWAL_INTERVAL";
    /// Name of the environment variable responsible for the `max_gas_price` scaling multiplier.
    const MAX_GAS_PRICE_SCALE_FACTOR_VAR: &'static str = "ETH_MAX_GAS_PRICE_SCALE_FACTOR";

    /// Obtains the interval for renewing the maximum gas price.
    ///
    /// This value is not cached internally, as it may be changed for the already running
    /// server by an administrator. This may be required if existing settings aren't flexible
    /// enough to match the current network price.
    pub fn get_max_price_interval() -> Duration {
        let renew_interval: u64 = parse_env(MAX_GAS_PRICE_RENEWAL_INTERVAL_VAR);

        Duration::from_secs(renew_interval)
    }

    /// Obtains the scaling factor for the maximum gas price.
    ///
    /// This value is not cached internally, as it may be changed for the already running
    /// server by an administrator. This may be required if existing settings aren't flexible
    /// enough to match the current network price.
    pub fn get_max_price_scale() -> f64 {
        parse_env(MAX_GAS_PRICE_SCALE_FACTOR_VAR)
    }
}

// Hard-coded implementation for tests.
#[cfg(test)]
mod parameters_impl {
    // Built-in deps.
    use std::time::Duration;

    /// `get_max_price_interval` version for tests not looking for an environment variable value
    /// but using a zero interval instead.
    pub fn get_max_price_interval() -> Duration {
        Duration::from_secs(0)
    }

    /// `get_max_price_scale` version for tests not looking for an environment variable value
    /// but using a fixed scale factor (1.5) instead.
    pub fn get_max_price_scale() -> f64 {
        1.5f64
    }
}
