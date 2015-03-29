use std::any::{ Any, TypeId };
use std::marker::Reflect;

use { GenericEvent, IdleArgs };

/// When background tasks should be performed
pub trait IdleEvent: Sized + Reflect {
    /// Creates an idle event.
    fn from_idle_args(args: &IdleArgs) -> Option<Self>;
    /// Creates an update event with delta time.
    fn from_dt(dt: f64) -> Option<Self> {
        IdleEvent::from_idle_args(&IdleArgs { dt: dt })
    }
    /// Calls closure if this is an idle event.
    fn idle<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&IdleArgs) -> U;
    /// Returns idle arguments.
    fn idle_args(&self) -> Option<IdleArgs> {
        self.idle(|args| args.clone())
    }
}

impl<T> IdleEvent for T where T: GenericEvent + Reflect {
    fn from_idle_args(args: &IdleArgs) -> Option<Self> {
        let id = TypeId::of::<Box<IdleEvent>>();
        GenericEvent::from_args(id, args as &Any)
    }

    fn idle<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&IdleArgs) -> U
    {
        let id = TypeId::of::<Box<IdleEvent>>();
        if self.event_id() != id {
            return None;
        }
        self.with_args(|any| {
            if let Some(args) = any.downcast_ref::<IdleArgs>() {
                Some(f(args))
            } else {
                panic!("Expected IdleArgs")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_event_idle() {
        use Event;
        use IdleArgs;

        let x: Option<Event> = IdleEvent::from_idle_args(
            &IdleArgs {
                dt: 1.0,
            }
        );
        let y: Option<Event> = x.clone().unwrap().idle(|args|
            IdleEvent::from_idle_args(args)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_event_idle(bencher: &mut Bencher) {
        use Event;
        use IdleArgs;

        let args = IdleArgs {
            dt: 1.0,
        };
        bencher.iter(|| {
            let _: Option<Event> = IdleEvent::from_idle_args(&args);
        });
    }
}
