use std::time::Duration;

pub(crate) fn duration_to_millis(duration: Duration) -> u64 {
    let seconds = duration.as_secs();
    let nanos = duration.subsec_nanos() as u64;
    let millis = (seconds * 1_000) + (nanos / 1_000_000);
    millis
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_to_millis() {
        let expected = 2000;
        let duration = Duration::from_millis(expected);
        let millis = duration_to_millis(duration);
        assert_eq!(expected, millis)
    }
}
