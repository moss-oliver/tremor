#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyboardKey {
    A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,
    Tab,Enter,
    Left,Up,Right,Down,
    Unknown
}

pub struct InputManager {
    pressed_keys: Vec<KeyboardKey>
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            pressed_keys: Vec::with_capacity(32)
        }
    }
    pub fn set_key_down(&mut self, key: KeyboardKey, value: bool) {
        let found_index = self.pressed_keys.iter().position(|&n| n == key );
        match found_index {
            Some(index) => {
                if !value {
                    //index exists, so remove if keyup.
                    self.pressed_keys.remove(index);
                }
            },
            None => {
                if value {
                    //index doesnt exist, so add if keydown
                    self.pressed_keys.push(key);
                }
            },
        }
    }
    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        let ref pressed = self.pressed_keys;
        for k in pressed {
            if(*k == key)
            {
                return true;
            }
        }
        return false;
    }
}