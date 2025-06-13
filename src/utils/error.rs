use std::{
	error::Error,
	fmt::{Debug, Display, Formatter, Result},
};

pub type EvoidResult<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Clone, Debug)]
pub enum EtherVoidError {
	AnimNotFound(String),
}

impl Display for EtherVoidError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Self::AnimNotFound(e) => write!(f, "Anim Not Found: \"{e}\" is not a known animation"),
		}
	}
}

// I feel like this should be a derive macro
impl Error for EtherVoidError {}
