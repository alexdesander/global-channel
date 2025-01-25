pub extern crate crossbeam_channel;
pub use paste;
pub use sync_unsafe_cell;

/// Creates a new global/static channel with the given name, bounds, and item type.
///
/// # Example
///
/// ```rust
/// use global_channel::global_channel;
///
/// global_channel!(my_channel, None, u32);
///
/// fn main() {
///     let tx = my_channel_tx();
///     let rx = my_channel_rx();
///     tx.send(42).unwrap();
///     assert_eq!(rx.recv().unwrap(), 42);
/// }
#[macro_export]
macro_rules! global_channel {
    ($channel_name:ident, $bounds:expr, $item:ty) => {
        $crate::paste::paste! {
            #[allow(non_upper_case_globals)]
            static [<__ $channel_name _TX>]: $crate::sync_unsafe_cell::SyncUnsafeCell<
                ::std::mem::MaybeUninit<$crate::crossbeam_channel::Sender<$item>>
            > = $crate::sync_unsafe_cell::SyncUnsafeCell::new(::std::mem::MaybeUninit::uninit());

            #[allow(non_upper_case_globals)]
            static [<__ $channel_name _RX>]: $crate::sync_unsafe_cell::SyncUnsafeCell<
                ::std::mem::MaybeUninit<$crate::crossbeam_channel::Receiver<$item>>
            > = $crate::sync_unsafe_cell::SyncUnsafeCell::new(::std::mem::MaybeUninit::uninit());

            // Initializes the channel once per process
            #[allow(non_snake_case)]
            fn [<setup_channel_ $channel_name>](bounds: Option<usize>) {
                static INIT: ::std::sync::Once = ::std::sync::Once::new();
                INIT.call_once(|| {
                    let (tx, rx) = match bounds {
                        Some(b) => $crate::crossbeam_channel::bounded(b),
                        None => $crate::crossbeam_channel::unbounded(),
                    };
                    unsafe {
                        let tx_ref = &mut *[<__ $channel_name _TX>].get();
                        let rx_ref = &mut *[<__ $channel_name _RX>].get();
                        tx_ref.write(tx);
                        rx_ref.write(rx);
                    }
                });
            }

            #[allow(non_snake_case)]
            #[inline]
            pub fn [<$channel_name _tx>]() -> &'static $crate::crossbeam_channel::Sender<$item> {
                [<setup_channel_ $channel_name>]($bounds);
                unsafe { &*(*[<__ $channel_name _TX>].get()).as_ptr() }
            }

            #[allow(non_snake_case)]
            #[inline]
            pub fn [<$channel_name _rx>]() -> &'static $crate::crossbeam_channel::Receiver<$item> {
                [<setup_channel_ $channel_name>]($bounds);
                unsafe { &*(*[<__ $channel_name _RX>].get()).as_ptr() }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    global_channel!(test_channel, None, u32);

    #[test]
    fn test_basic_usage() {
        let tx = test_channel_tx();
        let rx = test_channel_rx();
        tx.send(42).unwrap();
        assert_eq!(rx.recv().unwrap(), 42);
    }
}
