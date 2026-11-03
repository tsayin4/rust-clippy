#[derive(PartialEq, Hash)]
struct Foo;

impl PartialEq<u64> for Foo {
    fn eq(&self, _: &u64) -> bool {
        true
    }
}

#[derive(Hash)]
//~^ derived_hash_with_manual_eq

struct Bar;

impl PartialEq for Bar {
    fn eq(&self, _: &Bar) -> bool {
        true
    }
}

#[derive(Hash)]
//~^ derived_hash_with_manual_eq

struct Baz;

impl PartialEq<Baz> for Baz {
    fn eq(&self, _: &Baz) -> bool {
        true
    }
}

// Implementing `Hash` with a derived `PartialEq` is fine. See #2627

#[derive(PartialEq)]
struct Bah;

impl std::hash::Hash for Bah {
    fn hash<H: std::hash::Hasher>(&self, _: &mut H) {}
}

fn main() {}

mod issue15708 {
    // Check that the lint posts on the type definition node
    #[expect(clippy::derived_hash_with_manual_eq)]
    #[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
    pub struct Span {
        start: usize,
        end: usize,
    }

    impl PartialEq for Span {
        fn eq(&self, other: &Self) -> bool {
            self.start.cmp(&other.start).then(self.end.cmp(&other.end)).is_eq()
        }
    }
}

// Probe: rust-lang/rust-clippy#12092 -- does the lint fire on enums / generic types?
#[allow(clippy::needless_lifetimes)]
mod probe_12092 {
    // A. plain enum, no generics
    #[derive(Hash)]
    //~^ derived_hash_with_manual_eq
    enum A {
        X,
        Y,
    }

    impl PartialEq for A {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    // B. enum with a lifetime, manual impl reuses the SAME lifetime name as the derive
    #[derive(Hash)]
    //~^ derived_hash_with_manual_eq
    enum B<'a> {
        X(&'a str),
        Y(String),
    }

    impl<'a> PartialEq for B<'a> {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    // C. struct with a lifetime, manual impl ELIDES the lifetime ('_ instead of 'a)
    #[derive(Hash)]
    //~^ derived_hash_with_manual_eq
    struct C<'a>(&'a str);

    impl PartialEq for C<'_> {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    // D. struct with a type parameter
    #[derive(Hash)]
    //~^ derived_hash_with_manual_eq
    struct D<T>(T);

    impl<T> PartialEq for D<T> {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }
}
