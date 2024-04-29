#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Important because this sets the bit in the DDR register!
    pins.d9.into_output();

    // - TC1 runs off a 250kHz clock, with 5000 counts per overflow => 50 Hz signal.
    // - Each count increases the duty-cycle by 4us.
    // - Use OC1A which is connected to D9 of the Arduino Uno.
    let tc1 = dp.TC1;
    tc1.icr1.write(|w| w.bits(4999));
    tc1.tccr1a
        .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
    tc1.tccr1b
        .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());

    loop {
        // Move servo to front position (400 counts => ~1.6ms)
        tc1.ocr1a.write(|w| w.bits(400));
        arduino_hal::delay_ms(1000);

        // Move servo to right position (700 counts => ~2.8ms)
        tc1.ocr1a.write(|w| w.bits(700));
        arduino_hal::delay_ms(1000);

        // Move servo to left position (100 counts => ~0.4ms)
        tc1.ocr1a.write(|w| w.bits(100));
        arduino_hal::delay_ms(1000);
    }
}

