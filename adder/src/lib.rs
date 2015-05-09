#[test]
#[should_panic(expected = "assertion failed")]  //tells it to only give a bye/bi/by? if matched error
fn it_works() {
    assert_eq!("Hello", "World");
}

//assert(argument/evaluation) - sets panic! if passed false
//assert_eq(arg1, arg2) - compares two arguments and sets panic if they do not match

