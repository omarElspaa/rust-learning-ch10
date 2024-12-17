fn main() {
    println!("Hello, world!");
    // In Rust, &String is equal to &str, and &Vec<T> is equal to &[T] in function signatures because of deref coercion. Rust automatically converts references of types that implement the Deref trait to their dereferenced types when needed.
    // Search about deref coercion?
    // To parameterize the types in a new single function, we need to name the type parameter, just as we do for the value parameters to a function. You can use any identifier as a type parameter name. But we'll use T because, by convention, type parameter names in Rust are short, often just one letter, and Rust's type-naming convention is UpperCamelCase. Short for type, T is the default choice of most Rust programmers.
    // To define the generic largest function, we place type name declarations inside angle brackets, <>, between the name of the function and the parameter list.
    // fn largest<T>(list: &[T]) -> &T {
    // We read this definition as: the function largest is generic over some type T.



    // See Listing 10-5
    // The help text mentions std::cmp::PartialOrd, which is a trait, and we're going to talk about traits in the next section. For now, know that this error states that the body of largest wo't work for all possible types that T could be. Because we want to compare values of type T in the body, we can only use types whose values can be ordered. To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types ( see Appendix C for more on this trait )
    // Search for f16 and f128?
    // Note that we have to declare T just after impl so we can use T to specify that we're implementing methods on the Point<T>. By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
    struct Point<T> {
        x: T,
        y: T,
    }

    impl Point<i64> { // Wow wow wow
        fn x(&self) -> &i64 {
            &self.x
        }
    }
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // Rust accomplishes Zero Cost Abstractions by performing monomorphization of the code using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

    // Zero Cost Abstractions means adding higher-level programming concepts, like generics, collections and so on do not come with a run-time cost, only compile time cost (the code will be slower to compile). Any operation on zero-cost abstractions is as fast as you would write out matching functionality by hand using lower-level programming concepts like for loops, counters, ifs and using raw pointers.


    // A trait defines the functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

    // Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // Impl Summary for Type { functions }
    // One restriction to note is that we can implement a trait on a type only if either the trait or the type, or both, are local to our crate.
    // This restriction is part of a property called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other people's code can't break your code and vice versa. Without the rule two crates could implement the same trait for the same type, and Rust wouldn't know which implementation to use.
    // Coherence: the quality of being logical and consistent.

    // Sometimes it:s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type. Then, as we implement the trait on a particular type, we can keep or override each method:s default behavior.
    // To use a default implementation to summarize instances of NewsArticle, we specify an empty impl block with impl Summary for NewsArticle {}.

    // We can explore using traits to define functions that accept many different types.
    // pub fn notify(item: &impl Summary) {}, is a syntax sugar for a longer form known as a trait bound; it looks like this:
    // pub fn notify<T: Summary>(item: &T) {}
    // pub fn notify(item: &(impl Summary + Display)) {}
    // pub fn notify<T: Summary + Display>(item: &T) {}
    // Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. So, instead of:
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
    // we can use a where clause:
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // {}



    // We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait.
    // However, you can only use impl Trait if you're returning a single type.
    // When you return a type using impl Trait, it essentially tells the compiler that the function will return a single, concrete type that implements the specified trait. However, the key point here is "single type". All possible return values of the function must be the same type even though they are presented as a trait implementation.
    // Search for why? ( you can use the text in the top of page 200, to help you )

    // We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket  implementations and are used extensively in the Rust standard library.
    // Blanket implementations appear in the documentation for the trait in the "Implementors" section.
    // See the last paragraph in page 201, before "Validating References with Lifetimes"
}
