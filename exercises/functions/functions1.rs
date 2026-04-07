// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.

fn main() {
	let call_me: Box<dyn Fn()> = Box::new(|| {});
    call_me();
}
