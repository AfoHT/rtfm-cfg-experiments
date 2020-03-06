#![feature(prelude_import)]
//! examples/resource.rs
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
#[prelude_import]
use core::prelude::v1::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
use cortex_m_semihosting::{debug, hprintln};
use lm3s6965::Interrupt;
use panic_halt as _;
#[allow(non_snake_case)]
fn init(_: init::Context) -> init::LateResources {
    rtfm::pend(Interrupt::UART0);
    rtfm::pend(Interrupt::UART1);
    init::LateResources { dummy: () }
}
#[allow(non_snake_case)]
fn idle(_cx: idle::Context) -> ! {
    use rtfm::Mutex as _;
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
#[allow(non_snake_case)]
fn uart0(cx: uart0::Context) {
    use rtfm::Mutex as _;
    let shared: &mut u32 = cx.resources.shared;
    *shared += 1;
    ::cortex_m_semihosting::export::hstdout_fmt(::core::fmt::Arguments::new_v1(
        &["UART0: shared = ", "\n"],
        &match (&shared,) {
            (arg0,) => [::core::fmt::ArgumentV1::new(
                arg0,
                ::core::fmt::Display::fmt,
            )],
        },
    ))
    .unwrap();
}
#[allow(non_snake_case)]
fn uart1(cx: uart1::Context) {
    use rtfm::Mutex as _;
    *cx.resources.shared += 1;
    ::cortex_m_semihosting::export::hstdout_fmt(::core::fmt::Arguments::new_v1(
        &["UART1: shared = ", "\n"],
        &match (&cx.resources.shared,) {
            (arg0,) => [::core::fmt::ArgumentV1::new(
                arg0,
                ::core::fmt::Display::fmt,
            )],
        },
    ))
    .unwrap();
}
/// Resources initialized at runtime
#[allow(non_snake_case)]
pub struct initLateResources {
    pub dummy: (),
    pub x: u32,
}
#[allow(non_snake_case)]
///Initialization function
pub mod init {
    #[doc(inline)]
    pub use super::initLateResources as LateResources;
    /// Execution context
    pub struct Context {
        /// Core (Cortex-M) peripherals
        pub core: rtfm::export::Peripherals,
    }
    impl Context {
        #[inline(always)]
        pub unsafe fn new(core: rtfm::export::Peripherals) -> Self {
            Context { core }
        }
    }
}
#[allow(non_snake_case)]
///Idle loop
pub mod idle {
    /// Execution context
    pub struct Context {}
    impl Context {
        #[inline(always)]
        pub unsafe fn new(priority: &rtfm::export::Priority) -> Self {
            Context {}
        }
    }
}
#[allow(non_snake_case)]
///Resources `uart0` has access to
pub struct uart0Resources<'a> {
    pub shared: &'a mut u32,
    #[doc(hidden)]
    pub __marker__: core::marker::PhantomData<&'a ()>,
}
#[allow(non_snake_case)]
///Hardware task
pub mod uart0 {
    #[doc(inline)]
    pub use super::uart0Resources as Resources;
    /// Execution context
    pub struct Context<'a> {
        /// Resources this task has access to
        pub resources: Resources<'a>,
    }
    impl<'a> Context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtfm::export::Priority) -> Self {
            Context {
                resources: Resources::new(priority),
            }
        }
    }
}
#[allow(non_snake_case)]
///Resources `uart1` has access to
pub struct uart1Resources<'a> {
    pub shared: &'a mut u32,
}
#[allow(non_snake_case)]
///Hardware task
pub mod uart1 {
    #[doc(inline)]
    pub use super::uart1Resources as Resources;
    /// Execution context
    pub struct Context<'a> {
        /// Resources this task has access to
        pub resources: Resources<'a>,
    }
    impl<'a> Context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtfm::export::Priority) -> Self {
            Context {
                resources: Resources::new(priority),
            }
        }
    }
}
/// Implementation details
const APP: () = { # [ doc = r" Always include the device crate which contains the vector table" ] use lm3s6965 as _ ; # [ allow ( non_upper_case_globals ) ] static mut shared : u32 = 0 ; # [ allow ( non_snake_case ) ] # [ no_mangle ] unsafe fn UART0 ( ) { const PRIORITY : u8 = 1u8 ; rtfm :: export :: run ( PRIORITY , | | { crate :: uart0 ( uart0 :: Context :: new ( & rtfm :: export :: Priority :: new ( PRIORITY ) ) ) } ) ; } impl < 'a > uart0Resources < 'a > { # [ inline ( always ) ] unsafe fn new ( priority : & 'a rtfm :: export :: Priority ) -> Self { uart0Resources { shared : & mut shared , __marker__ : core :: marker :: PhantomData , } } } # [ allow ( non_snake_case ) ] # [ no_mangle ] unsafe fn UART1 ( ) { const PRIORITY : u8 = 1u8 ; rtfm :: export :: run ( PRIORITY , | | { crate :: uart1 ( uart1 :: Context :: new ( & rtfm :: export :: Priority :: new ( PRIORITY ) ) ) } ) ; } impl < 'a > uart1Resources < 'a > { # [ inline ( always ) ] unsafe fn new ( priority : & 'a rtfm :: export :: Priority ) -> Self { uart1Resources { shared : & mut shared , } } } # [ no_mangle ] unsafe extern "C" fn main ( ) -> ! { rtfm :: export :: assert_send :: < u32 > ( ) ; rtfm :: export :: interrupt :: disable ( ) ; let mut core : rtfm :: export :: Peripherals = core :: mem :: transmute ( ( ) ) ; let _ = [ ( ) ; ( ( 1 << lm3s6965 :: NVIC_PRIO_BITS ) - 1u8 as usize ) ] ; core . NVIC . set_priority ( lm3s6965 :: Interrupt :: UART0 , rtfm :: export :: logical2hw ( 1u8 , lm3s6965 :: NVIC_PRIO_BITS ) ) ; rtfm :: export :: NVIC :: unmask ( lm3s6965 :: Interrupt :: UART0 ) ; let _ = [ ( ) ; ( ( 1 << lm3s6965 :: NVIC_PRIO_BITS ) - 1u8 as usize ) ] ; core . NVIC . set_priority ( lm3s6965 :: Interrupt :: UART1 , rtfm :: export :: logical2hw ( 1u8 , lm3s6965 :: NVIC_PRIO_BITS ) ) ; rtfm :: export :: NVIC :: unmask ( lm3s6965 :: Interrupt :: UART1 ) ; let late = init ( init :: Context :: new ( core . into ( ) ) ) ; x . as_mut_ptr ( ) . write ( late . x ) ; rtfm :: export :: interrupt :: enable ( ) ; idle ( idle :: Context :: new ( & rtfm :: export :: Priority :: new ( 0 ) ) ) } };
