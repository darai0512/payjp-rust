use std::time::Duration;

fn is_status_client_error(status: u16) -> bool {
    (400..500).contains(&status)
}

/// Possible strategies for sending API requests, including retry behavior
/// and use of idempotency keys.
#[derive(Clone, Debug)]
pub enum RequestStrategy {
    /// Run the request once.
    Once,
    /// This strategy will retry the request up to the
    /// specified number of times using the same, random,
    /// idempotency key, up to n times.
    Retry(u32),
    /// This strategy will retry the request up to the
    /// specified number of times using the same, random,
    /// idempotency key with exponential backoff, up to n times.
    ExponentialBackoff(u32),
}

impl RequestStrategy {
    /// Decide if we should retry this request, or stop.
    pub fn test(
        &self,
        // Status would be much better as a newtype, but we want to be library
        // agnostic. We could reimplement our own `StatusCode`, but that
        // feels a bit like reinventing the wheel...
        status: Option<u16>,
        retry_count: u32,
    ) -> Outcome {
        use RequestStrategy::*;

        match (self, status, retry_count) {
            // a strategy of once should run once
            (Once, _, 0) => Outcome::Continue(None),

            // requests with idempotency keys that hit client
            // errors usually cannot be solved with retries
            (_, Some(c), _) if is_status_client_error(c) => Outcome::Stop,

            // a strategy of retry or exponential backoff should retry with
            // the appropriate delay if the number of retries is less than the max
            (Retry(n), _, x) if x < *n => Outcome::Continue(None),
            (ExponentialBackoff(n), _, x) if x < *n => {
                Outcome::Continue(Some(calculate_backoff(x)))
            }

            // unknown cases should be stopped to prevent infinite loops
            _ => Outcome::Stop,
        }
    }

    /// Extract the current idempotency key to use for the next request, if any.
    pub fn get_key(&self) -> Option<String> {
        match self {
            RequestStrategy::Once => None,
            #[cfg(feature = "uuid")]
            RequestStrategy::Retry(_) | RequestStrategy::ExponentialBackoff(_) => {
                Some(uuid::Uuid::new_v4().to_string())
            }
            #[cfg(not(feature = "uuid"))]
            RequestStrategy::Retry(_) | RequestStrategy::ExponentialBackoff(_) => None,
        }
    }
}

fn calculate_backoff(retry_count: u32) -> Duration {
    Duration::from_secs(2_u64.pow(retry_count))
}

/// Representation of whether to retry the API request, including a potential waiting period.
#[derive(PartialEq, Eq, Debug)]
pub enum Outcome {
    /// Do not retry the requests, return the last error.
    Stop,
    /// Send another request, either immediately or after sleeping for the given `Duration`
    /// if provided.
    Continue(Option<Duration>),
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{Outcome, RequestStrategy};

    #[test]
    fn test_once_strategy() {
        let strategy = RequestStrategy::Once;
        assert_eq!(strategy.get_key(), None);
        assert_eq!(strategy.test(None, 0), Outcome::Continue(None));
        assert_eq!(strategy.test(None, 1), Outcome::Stop);
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_uuid_idempotency() {
        use uuid::Uuid;
        let strategy = RequestStrategy::Retry(3);
        assert!(Uuid::parse_str(&strategy.get_key().unwrap()).is_ok());
    }

    #[test]
    #[cfg(not(feature = "uuid"))]
    fn test_uuid_idempotency() {
        let strategy = RequestStrategy::Retry(3);
        assert_eq!(strategy.get_key(), None);
    }

    #[test]
    fn test_retry_strategy() {
        let strategy = RequestStrategy::Retry(3);
        assert_eq!(strategy.test(None, 0), Outcome::Continue(None));
        assert_eq!(strategy.test(None, 1), Outcome::Continue(None));
        assert_eq!(strategy.test(None, 2), Outcome::Continue(None));
        assert_eq!(strategy.test(None, 3), Outcome::Stop);
        assert_eq!(strategy.test(None, 4), Outcome::Stop);
    }

    #[test]
    fn test_backoff_strategy() {
        let strategy = RequestStrategy::ExponentialBackoff(3);
        assert_eq!(strategy.test(None, 0), Outcome::Continue(Some(Duration::from_secs(1))));
        assert_eq!(strategy.test(None, 1), Outcome::Continue(Some(Duration::from_secs(2))));
        assert_eq!(strategy.test(None, 2), Outcome::Continue(Some(Duration::from_secs(4))));
        assert_eq!(strategy.test(None, 3), Outcome::Stop);
        assert_eq!(strategy.test(None, 4), Outcome::Stop);
    }

    #[test]
    fn test_retry_header() {
        let strategy = RequestStrategy::Retry(3);
        assert_eq!(strategy.test(None, 0), Outcome::Stop);
    }
}
