
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use lm3s6965::Interrupt;
use panic_halt as _;

#[rtfm::app(device = lm3s6965)]
const APP: () = {
    struct Resources {
        // A resource
        #[init(0)]
        shared: u32,
        
        #[cfg(feature = "feature_x")]
        x: u32,

        dummy: (),
    }

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        rtfm::pend(Interrupt::UART0);
        rtfm::pend(Interrupt::UART1);

        init::LateResources {
            #[cfg(feature = "feature_x")]
            x: 0,
            dummy: () // dummy such that we have at least one late resource
        }
    }

    // `shared` cannot be accessed from this context
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        debug::exit(debug::EXIT_SUCCESS);

        // error: no `resources` field in `idle::Context`
        // _cx.resources.shared += 1;

        loop {}
    }

    // `shared` can be accessed from this context
    #[task(binds = UART0, resources = [#[cfg(feature = "feature_x")] x, shared])]
    fn uart0(cx: uart0::Context) {
        let shared: &mut u32 = cx.resources.shared;
        *shared += 1;

        hprintln!("UART0: shared = {}", shared).unwrap();

        #[cfg(feature = "feature_x")]
        {
            hprintln!("UART0: X = {}", shared).unwrap();
        }
    }

    // `shared` can be accessed from this context
    #[task(binds = UART1, resources = [shared])]
    fn uart1(cx: uart1::Context) {
        *cx.resources.shared += 1;

        hprintln!("UART1: shared = {}", cx.resources.shared).unwrap();
    }
};