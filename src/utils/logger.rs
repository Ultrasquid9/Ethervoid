use std::{
	fs::File,
	io::{Result, Stdout, Write},
};

use tracing::info;

use super::error::EvoidResult;
pub struct Logger {
	stdout: Stdout,
	file: File,
}

impl Logger {
	pub fn new() -> Self {
		let file = std::fs::OpenOptions::new()
			.append(true)
			.create(true)
			.open("./output.log")
			.expect("Could not create log file");

		let stdout = std::io::stdout();

		Self { stdout, file }
	}
}

impl Write for Logger {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		let ok_stdout = self.stdout.write(buf)?;
		let ok_file = self.file.write(buf)?;
		Ok(std::cmp::min(ok_stdout, ok_file))
	}

	fn flush(&mut self) -> Result<()> {
		self.stdout.flush()?;
		self.file.flush()?;
		Ok(())
	}
}

/// Initiates the logger. Should do nothing if already called.
pub async fn init_log() -> EvoidResult<()> {
	// Renaming old log
	_ = std::fs::rename("./output.log", "./output.log.old");

	let subscriber = tracing_subscriber::FmtSubscriber::builder()
		.with_writer(Logger::new)
		.finish();

	tracing::subscriber::set_global_default(subscriber)?;
	info!("Logger created successfully");

	Ok(())
}
