use embedded_hal::delay::DelayNs;
use rp235x_hal::{Timer, arch, pac::QMI, timer::CopyableTimer0};

// APS6404L-3SQR-ZR QSPI PSRAM constraints
const MAX_PSRAM_FREQ: u32 = 133_000_000;
// Min CE# HIGH between subsequent burst operations in nanoseconds
const MIN_CPH_NS: f32 = 18f32;
// Max CE# low pulse width in nanoseconds
const MAX_CEM_NS: f32 = 8000f32;

const ENTER_QUAD_MODE: u8 = 0x35;
const EXIT_QUAD_MODE: u8 = 0xF5;
const READ_ID: u8 = 0x9F;
const RESET_ENABLE: u8 = 0x66;
const RESET: u8 = 0x99;
const FAST_QUAD_READ: u8 = 0xEB;
const QUAD_WRITE: u8 = 0x38;

pub const BASE_ADDRESS: usize = 0x11000000;

#[unsafe(link_section = ".data")]
#[inline(never)]
pub fn read_id(qmi: &QMI) -> (u8, u8, u8) {
    critical_section::with(|_cs| {
        qmi.direct_csr().write(|w| unsafe {
            w.clkdiv().bits(30);
            w.en().set_bit();
            w
        });
        while qmi.direct_csr().read().busy().bit_is_set() {
            arch::nop();
        }

        // Exit quad mode...
        qmi.direct_csr().modify(|_r, w| w.assert_cs1n().set_bit());
        while qmi.direct_csr().read().txempty().bit_is_clear() {
            arch::nop();
        }
        qmi.direct_tx().write(|w| unsafe {
            w.oe().set_bit();
            w.iwidth().q();
            w.data().bits(EXIT_QUAD_MODE as u16);
            w
        });

        while qmi.direct_csr().read().busy().bit_is_set() {
            arch::nop();
        }
        let _ = qmi.direct_rx().read().bits();
        qmi.direct_csr().modify(|_, w| w.assert_cs1n().clear_bit());

        // Read IDs...
        qmi.direct_csr().modify(|_, w| w.assert_cs1n().set_bit());
        let mut kgd = 0;
        let mut mf_id = 0;
        let mut density = 0;
        for i in 0..=6 {
            while qmi.direct_csr().read().txempty().bit_is_clear() {
                arch::nop();
            }
            if i == 0 {
                qmi.direct_tx().write(|w| unsafe { w.bits(READ_ID as u32) });
            } else {
                qmi.direct_tx().write(|w| unsafe { w.bits(0x00) });
            }

            while qmi.direct_csr().read().busy().bit_is_set() {
                arch::nop();
            }
            if i == 4 {
                mf_id = qmi.direct_rx().read().bits();
            } else if i == 5 {
                kgd = qmi.direct_rx().read().bits();
            } else if i == 6 {
                density = qmi.direct_rx().read().bits();
            } else {
                let _ = qmi.direct_rx().read().bits();
            }
        }
        qmi.direct_csr().modify(|_, w| {
            w.assert_cs1n().clear_bit();
            w.en().clear_bit();
            w
        });

        (mf_id as u8, kgd as u8, density as u8)
    })
}

#[unsafe(link_section = ".data")]
#[inline(never)]
pub fn init(qmi: &QMI, timer: &mut Timer<CopyableTimer0>, system_clock_freq: u32) {
    critical_section::with(|_cs| {
        // enable QMI with a starting clkdiv
        qmi.direct_csr().write(|w| unsafe {
            w.clkdiv().bits(30);
            w.en().set_bit();
            w
        });
        while qmi.direct_csr().read().busy().bit_is_set() {
            arch::nop();
        }

        // Issue RESET_ENABLE, RESET, then ENTER_QUAD_MODE
        for command in [RESET_ENABLE, RESET, ENTER_QUAD_MODE] {
            qmi.direct_csr().modify(|_, w| w.assert_cs1n().set_bit());
            while qmi.direct_csr().read().txempty().bit_is_clear() {
                arch::nop();
            }
            qmi.direct_tx().write(|w| unsafe { w.bits(command as u32) });

            while qmi.direct_csr().read().busy().bit_is_set() {
                arch::nop();
            }
            let _ = qmi.direct_rx().read().bits();
            qmi.direct_csr().modify(|_, w| w.assert_cs1n().clear_bit());

            if command == RESET {
                // tRST >= 50ns...
                timer.delay_ns(100);
            }
        }

        let clock_div = system_clock_freq.div_ceil(MAX_PSRAM_FREQ);

        // compute period in ns
        let system_clock_period_ns = 1_000_000_000f32 / system_clock_freq as f32;

        // compute max_select expressed in 64-system-clock units
        let max_select_in_system_clock = MAX_CEM_NS / system_clock_period_ns;
        let max_select_in_64_system_clocks = max_select_in_system_clock / 64f32;
        let max_select_in_64_system_clocks = max_select_in_64_system_clocks as u8;

        // compute min deselect (CE high) in system clock cycles;
        let min_deselect_in_system_clock = MIN_CPH_NS / system_clock_period_ns;
        let min_deselect_in_system_clock = min_deselect_in_system_clock as u8 + 1;

        // configure timings (cooldown, clkdi, min/max CE#) to allow QMI transfer-chaining
        qmi.m1_timing().write(|w| unsafe {
            w.cooldown().bits(1);
            w.pagebreak()._1024();
            w.max_select().bits(max_select_in_64_system_clocks);
            w.min_deselect().bits(min_deselect_in_system_clock);
            w.rxdelay().bits(clock_div as u8);
            w.clkdiv().bits(clock_div as u8);
            w
        });

        // configure read format for FAST QUAD READ (0xEB)
        qmi.m1_rfmt().write(|w| {
            w.prefix_width().q();
            w.addr_width().q();
            w.suffix_width().q();
            w.dummy_width().q();
            w.data_width().q();
            w.prefix_len()._8();
            w.dummy_len()._24();
            w
        });
        qmi.m1_rcmd()
            .write(|w| unsafe { w.bits(FAST_QUAD_READ as u32) });

        // configure write format for QUAD_WRITE (0x38)
        qmi.m1_wfmt().write(|w| {
            w.prefix_width().q();
            w.addr_width().q();
            w.suffix_width().q();
            w.dummy_width().q();
            w.data_width().q();
            w.prefix_len()._8();
            w
        });
        qmi.m1_wcmd()
            .write(|w| unsafe { w.bits(QUAD_WRITE as u32) });

        qmi.direct_csr().write(|w| {
            w.en().clear_bit();
            w.auto_cs1n().set_bit();
            w
        });
        while qmi.direct_csr().read().busy().bit_is_set() {
            arch::nop();
        }
    });
}
