use std::time::Duration;

pub struct Spi;

pub struct Gpio;

fn sleep_ms(duration: Duration) {
    todo!()
}

impl Spi {
    fn write(&self, data: &[u8]) {
        todo!()
    }
}

impl Gpio {
    fn set(&self, val: bool) {
        todo!()
    }

    fn get(&self) -> bool {
        todo!()
    }
}

pub struct LCDDriver<const W: u8, const H: u8> {
    spi: Spi,
    dc: Gpio,
    rst: Gpio,
}

impl<const W: u8, const H: u8> LCDDriver<W, H> {
    pub fn reset(&self) {
        self.rst.set(false);
        sleep_ms(Duration::from_millis(100));
        self.rst.set(true);
        sleep_ms(Duration::from_millis(100));
    }

    pub fn new(spi: Spi, dc: Gpio, rst: Gpio) -> Self {
        let lcd = LCDDriver { spi, dc, rst };

        lcd.reset();

        lcd.send_empty_cmd(0x01);
        sleep_ms(Duration::from_millis(100));

        lcd.send_empty_cmd(0x11);
        sleep_ms(Duration::from_millis(100));

        lcd.send_cmd(0x26, &[0x04]);
        lcd.send_cmd(0x3A, &[0x55]);
        sleep_ms(Duration::from_millis(20));

        lcd.send_cmd(0x36, &[0x80 | 0x20 | 0x08]);
        lcd.send_cmd(0x2A, &[W - 1, 0, 0, 0]);
        lcd.send_cmd(0x2B, &[H - 1, 0, 0, 0]);
        lcd.send_empty_cmd(0x13);
        sleep_ms(Duration::from_millis(2));

        lcd.send_empty_cmd(0x29);
        sleep_ms(Duration::from_millis(100));

        lcd
    }

    fn send_empty_cmd(&self, cmd: u8) {
        self.dc.set(false);
        self.spi.write(&[cmd]);
    }

    fn send_cmd(&self, cmd: u8, data: &[u8]) {
        self.dc.set(false);
        self.spi.write(&[cmd]);

        self.dc.set(true);
        self.spi.write(data);
    }

    fn refresh_all(&self, vram: &[u8]) {
        self.send_cmd(0x2A, &[0, 0, 0, W]);
        self.send_cmd(0x2B, &[0, 0, 0, H]);
        self.send_cmd(0x2C, vram);

        sleep_ms(Duration::from_millis(100));
    }
}
