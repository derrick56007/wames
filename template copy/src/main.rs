use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    let device_state = DeviceState::new();
    let mut last_pressed_key: Option<Keycode> = None;
    print!("{}[2J", 27 as char);

    print!("\x1B[2J\x1B[1;1H");
    println!("Hello, world!");
    

    'outer: 
    loop {

        let keys: Vec<Keycode> = device_state.get_keys();
        let mut pressed_key: Option<Keycode> = None;
        for key in keys.iter() {
            if Some(key) == last_pressed_key.as_ref() {
                // ignore if same key is pressed twice
                continue 'outer;
            }

            pressed_key = Some(*key);
            last_pressed_key = pressed_key;

            match key {
                _ => {} 
            }
        }
        if pressed_key.is_none() {
            last_pressed_key = None;
            continue 'outer;
        }
        print!("{}[2J", 27 as char);

        print!("\x1B[2J\x1B[1;1H");
        println!("Hello, world!");
    }
}
