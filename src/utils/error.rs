use std::{
	error::Error,
	fmt::{Debug, Display},
};

pub type EvoidResult<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Clone)]
pub enum EtherVoidError {
	AnimNotFound(String),
}

impl EtherVoidError {
	fn do_the_formatting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::AnimNotFound(e) => write!(f, "Anim Not Found: \"{e}\" is not a known animation"),
		}
	}
}

impl Display for EtherVoidError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.do_the_formatting(f)
	}
}
impl Debug for EtherVoidError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.do_the_formatting(f)
	}
}

// I feel like this should be a derive macro
impl Error for EtherVoidError {}
