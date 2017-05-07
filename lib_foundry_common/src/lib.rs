pub enum PlatformType {
    Win32,
    MacOs,
    Unix
}

pub trait Platform {
    type T_WIN : Window;
    fn get_platform_type(&self) -> PlatformType;
    fn get_platform_name(&self) -> &'static str;
    fn create_window(&self, window_name:&str) -> Option<Self::T_WIN>;
}

pub enum PlatformEvent {
    //Window events
    WindowClose,
    //Keyboard events
    KeyboardKeydown { key:KeyboardKey},
    KeyboardKeyup { key:KeyboardKey},
}

#[derive(Debug)]
pub enum KeyboardKey {
    A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,
    Tab,Enter,
    Left,Up,Right,Down,
    Unknown
}

pub trait Window {
    
}

pub struct PlatformError<T> where T: std::fmt::Debug {
    pub error_code:T,
    pub text:&'static str
}

impl<T> PlatformError<T> where T: std::fmt::Debug {
    pub fn new(new_error_code:T, new_text:&'static str) -> PlatformError<T> where T: std::fmt::Debug {
        return PlatformError {
            error_code:new_error_code,
            text:new_text
        }
    }
}

impl<T> std::fmt::Debug for PlatformError<T> where T: std::fmt::Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Platform error {:?}: {}", self.error_code, self.text)
    }
}