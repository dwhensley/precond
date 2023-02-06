use precond::precond;

#[test]
fn test_arity_one() {
    #[precond(|a| a * 10)]
    fn f(a: u64) -> u64 {
        a
    }

    assert_eq!(10, f(1));
}

#[test]
fn test_arity_two() {
    #[precond(|a, b| (b, a))]
    fn f(a: i32, b: i32) -> i32 {
        a - b
    }

    assert_eq!(-7, f(10, 3))
}

#[test]
fn test_arity_five() {
    #[precond(|a, b, c, d, e| (e, d, c, b, a))]
    fn f(a: u8, b: u8, c: u8, d: u8, e: u8) -> u8 {
        a
    }

    assert_eq!(5, f(1, 2, 3, 4, 5));
}

#[test]
fn test_generics() {
    #[precond(|a, b| (b, a))]
    fn f<T>(a: T, b: T) -> T {
        a
    }

    assert_eq!(2u64, f(1u64, 2u64));
    assert_eq!((2u8, 2u8), f((1u8, 1u8), (2u8, 2u8)));
}

#[test]
fn test_generics_with_constraints() {
    #[precond(|a, b| (a, b))]
    fn f<A: Clone, B: std::fmt::Debug>(a: A, b: B) -> A {
        let _ = a.clone();
        let _ = format!("{:?}", b);
        a
    }

    assert_eq!(2usize, f(2usize, "string"));
}

#[test]
fn test_generics_with_where_clause() {
    #[precond(|a, b| (a.clone() + a, b))]
    fn f<A, B>(a: A, b: B) -> A
    where
        A: std::ops::Add<Output = A> + Clone,
        B: std::string::ToString,
    {
        let _ = b.to_string();
        let a_cloned = a.clone();
        a + a_cloned
    }

    assert_eq!(8usize, f(2usize, "string"));
}
