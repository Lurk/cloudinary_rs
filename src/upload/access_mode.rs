use core::fmt;

#[derive(Debug)]
pub enum AccessModes {
    Public,
    Authenticated,
}

impl fmt::Display for AccessModes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccessModes::Public => write!(f, "public"),
            AccessModes::Authenticated => write!(f, "authenticated"),
        }
    }
}