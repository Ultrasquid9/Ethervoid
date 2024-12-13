use std::{error::Error, fmt::{Debug, Display}};

#[derive(Clone)]
pub enum EtherVoidError {
	AnimNotFound(String)
}

impl Display for EtherVoidError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::AnimNotFound(e) => write!(f, "Anim Not Found: \"{e}\" is not a known animation")
		}
	}
}

impl Debug for EtherVoidError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::AnimNotFound(e) => write!(f, "Anim Not Found: \"{e}\" is not a known animation")
		}		
	}
}

// I feel like this should be a derive macro
impl Error for EtherVoidError {}
