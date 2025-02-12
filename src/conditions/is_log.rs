use crate::event::Event;

pub(crate) const fn check_is_log(e: Event) -> (bool, Event) {
    (matches!(e, Event::Log(_)), e)
}

pub(crate) fn check_is_log_with_context(e: Event) -> (Result<(), String>, Event) {
    let (result, event) = check_is_log(e);
    if result {
        (Ok(()), event)
    } else {
        (Err("event is not a log type".to_string()), event)
    }
}

#[cfg(test)]
mod test {
    use super::check_is_log;
    use crate::event::{
        metric::{Metric, MetricKind, MetricValue},
        Event,
    };

    #[test]
    fn is_log_basic() {
        assert!(check_is_log(Event::from("just a log")).0);
        assert!(
            !check_is_log(Event::from(Metric::new(
                "test metric",
                MetricKind::Incremental,
                MetricValue::Counter { value: 1.0 },
            )))
            .0,
        );
    }
}
