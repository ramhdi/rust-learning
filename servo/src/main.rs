pub struct Servo {
    pin: u8,
    attached: bool,
}

impl Servo {
    fn new(pin: u8) -> Servo {
        return Servo {
            pin: pin,
            attached: false,
        }
    }

    fn attach(&mut self) {
        self.attached = true;
        println!("Servo attached to pin {}", self.pin);
    }

    fn write(&self, angle: f64) {
        if !self.attached {
            println!("Servo on pin {} not yet attached!", self.pin);
        } else {
            println!("Servo on pin {} moved to {} degree", self.pin, angle);
        }
        
    }
}

fn main() {
    let mut servo1 = Servo::new(8);
    servo1.attach();
    servo1.write(128.5);
    servo1.pin = 5;
    servo1.write(128.5);

    let mut servo2 = Servo::new(3);
    //servo2.attach();
    servo2.write(99.0);
    servo2.attached = true;
    servo2.write(99.0);
}
