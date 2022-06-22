mod veteran;
mod newcomer;
mod foo;

fn main() {
    assert!(foo::bar());
	assert_eq!(2, veteran::mul(2, newcomer::mul::MUL_IDENTITY));
	println!("{}", newcomer::hello());
}
