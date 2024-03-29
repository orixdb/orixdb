#[allow(dead_code)]
pub fn white_out(s: String) {
	print!("\x1b[1m");
	print!("\x1b[37m");
	println!("{}", s);
	print!("\x1b[0m");
}

#[allow(dead_code)]
pub fn black_out(s: String) {
	print!("\x1b[1m");
	print!("\x1b[30m");
	println!("{}", s);
	print!("\x1b[0m");
}

#[allow(dead_code)]
pub fn blue_out(s: String) {
	print!("\x1b[1m");
	print!("\x1b[34m");
	println!("{}", s);
	print!("\x1b[0m");
}
#[allow(dead_code)]
pub fn red_out(s: String) {
	print!("\x1b[1m");
	print!("\x1b[31m");
	println!("{}", s);
	print!("\x1b[0m");
}

#[allow(dead_code)]
pub fn green_out(s: String) {
	print!("\x1b[1m");
	print!("\x1b[32m");
	println!("{}", s);
	print!("\x1b[0m");
}

#[allow(dead_code)]
pub fn yellow_out(s: String) {
	print!("\x1b[1m");
	print!("\x1b[33m");
	println!("{}", s);
	print!("\x1b[0m");
}

#[allow(dead_code)]
pub fn cyan_out(s: String) {
	print!("\x1b[1m");
	print!("\x1b[36m");
	println!("{}", s);
	print!("\x1b[0m");
}

#[allow(dead_code)]
pub fn magenta_out(s: String) {
	print!("\x1b[1m");
	print!("\x1b[35m");
	println!("{}", s);
	print!("\x1b[0m");
}

#[allow(dead_code)]
pub fn white_err(s: String) {
	eprint!("\x1b[1m");
	eprint!("\x1b[37m");
	eprintln!("{}", s);
	eprint!("\x1b[0m");
}

#[allow(dead_code)]
pub fn black_err(s: String) {
	eprint!("\x1b[1m");
	eprint!("\x1b[30m");
	eprintln!("{}", s);
	eprint!("\x1b[0m");
}

#[allow(dead_code)]
pub fn blue_err(s: String) {
	eprint!("\x1b[1m");
	eprint!("\x1b[34m");
	eprintln!("{}", s);
	eprint!("\x1b[0m");
}

#[allow(dead_code)]
pub fn red_err(s: String) {
	eprint!("\x1b[1m");
	eprint!("\x1b[31m");
	eprintln!("{}", s);
	eprint!("\x1b[0m");
}

#[allow(dead_code)]
pub fn green_err(s: String) {
	eprint!("\x1b[1m");
	eprint!("\x1b[32m");
	eprintln!("{}", s);
	eprint!("\x1b[0m");
}

#[allow(dead_code)]
pub fn yellow_err(s: String) {
	eprint!("\x1b[1m");
	eprint!("\x1b[33m");
	eprintln!("{}", s);
	eprint!("\x1b[0m");
}

#[allow(dead_code)]
pub fn cyan_err(s: String) {
	eprint!("\x1b[1m");
	eprint!("\x1b[36m");
	eprintln!("{}", s);
	eprint!("\x1b[0m");
}

#[allow(dead_code)]
pub fn magenta_err(s: String) {
	eprint!("\x1b[1m");
	eprint!("\x1b[35m");
	eprintln!("{}", s);
	eprint!("\x1b[0m");
}