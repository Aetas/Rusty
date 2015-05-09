pub fn add_two(a:i32) -> i32 {
    a + 2
}

#[test]
//#[should_panic(expected = "assertion failed")]  //tells it to only give a bye/bi/by? if matched error
fn it_works() {
    assert_eq!(4, add_two(2));
}

//assert(argument/evaluation) - sets panic! if passed false
//assert_eq(arg1, arg2) - compares two arguments and sets panic if they do not match

