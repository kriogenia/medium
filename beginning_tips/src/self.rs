struct Foo {
	bar: i32
}

impl Foo {
	fn instead_of_this() -> Foo {
		Foo { bar: 0 }
	}

	fn you_can_use_this() -> Self {
		Self { bar: 0 }
	}
}

fn main() {
	let a = Foo::instead_of_this();
	let b = Foo::you_can_use_this();
	assert_eq!(a.bar, b.bar);
}