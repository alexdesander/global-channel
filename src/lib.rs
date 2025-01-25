/// Creates a new global/static channel with the given name, bounds and item type.
/// 
/// Usage example: global_channel!(name, None, u32); // Unbounded channel of u32
#[macro_export]
macro_rules! global_channel {
    ($channel_name:ident, $bounds:expr, $item:ty) => {
        paste::paste! {
            #[allow(non_upper_case_globals)]
            static [<__ $channel_name _TX>]: ::sync_unsafe_cell::SyncUnsafeCell<
                ::std::mem::MaybeUninit<::crossbeam_channel::Sender<$item>>
            > = ::sync_unsafe_cell::SyncUnsafeCell::new(::std::mem::MaybeUninit::uninit());

            #[allow(non_upper_case_globals)]
            static [<__ $channel_name _RX>]: ::sync_unsafe_cell::SyncUnsafeCell<
                ::std::mem::MaybeUninit<::crossbeam_channel::Receiver<$item>>
            > = ::sync_unsafe_cell::SyncUnsafeCell::new(::std::mem::MaybeUninit::uninit());

            // The setup function that initializes the channel exactly once
            #[allow(non_snake_case)]
            fn [<setup_channel_ $channel_name>](bounds: Option<usize>) {
                static INIT: ::std::sync::Once = ::std::sync::Once::new();
                INIT.call_once(|| {
                    let (tx, rx) = match bounds {
                        Some(b) => ::crossbeam_channel::bounded(b),
                        None => ::crossbeam_channel::unbounded(),
                    };
                    unsafe {
                        let tx_ref = &mut *[<__ $channel_name _TX>].get();
                        let rx_ref = &mut *[<__ $channel_name _RX>].get();
                        tx_ref.write(tx);
                        rx_ref.write(rx);
                    }
                });
            }

            // Public function to get the Sender
            #[allow(non_snake_case)]
            pub fn [<$channel_name _tx>]() -> &'static ::crossbeam_channel::Sender<$item> {
                [<setup_channel_ $channel_name>]($bounds);
                unsafe { &*(*[<__ $channel_name _TX>].get()).as_ptr() }
            }

            #[allow(non_snake_case)]
            pub fn [<$channel_name _rx>]() -> &'static ::crossbeam_channel::Receiver<$item> {
                [<setup_channel_ $channel_name>]($bounds);
                unsafe { &*(*[<__ $channel_name _RX>].get()).as_ptr() }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    global_channel!(test_channel, None, u32);

    #[test]
    fn test_channel() {
        let tx = test_channel_tx();
        let rx = test_channel_rx();
        tx.send(42).unwrap();
        assert_eq!(rx.recv().unwrap(), 42);
    }
}