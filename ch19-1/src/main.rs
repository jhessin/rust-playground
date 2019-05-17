// Unsafe Rust
//
//All the code we’ve discussed so far has had Rust’s memory safety
// guarantees enforced at compile time. However, Rust has a second
// language hidden inside it that doesn’t enforce these memory safety
// guarantees: it’s called unsafe Rust and works just like regular Rust,
// but gives us extra superpowers.
//
//Unsafe Rust exists because, by nature, static analysis is conservative.
// When the compiler tries to determine whether or not code upholds the
// guarantees, it’s better for it to reject some valid programs rather
// than accept some invalid programs. Although the code might be okay,
// as far as Rust is able to tell, it’s not! In these cases, you can use
// unsafe code to tell the compiler, “Trust me, I know what I’m doing.”
// The downside is that you use it at your own risk: if you use unsafe
// code incorrectly, problems due to memory unsafety, such as null
// pointer dereferencing, can occur.
//
//Another reason Rust has an unsafe alter ego is that the underlying
// computer hardware is inherently unsafe. If Rust didn’t let you do
// unsafe operations, you couldn’t do certain tasks. Rust needs to allow
// you to do low-level systems programming, such as directly interacting
// with the operating system or even writing your own operating system.
// Working with low-level systems programming is one of the goals of the
// language. Let’s explore what we can do with unsafe Rust and how to do
// it. Unsafe Superpowers
//
//To switch to unsafe Rust, use the unsafe keyword and then start a new
// block that holds the unsafe code. You can take four actions in unsafe
// Rust, called unsafe superpowers, that you can’t in safe Rust. Those
// superpowers include the ability to:
//
//    Dereference a raw pointer
//    Call an unsafe function or method
//    Access or modify a mutable static variable
//    Implement an unsafe trait
//
//It’s important to understand that unsafe doesn’t turn off the borrow
// checker or disable any other of Rust’s safety checks: if you use a
// reference in unsafe code, it will still be checked. The unsafe
// keyword only gives you access to these four features that are then
// not checked by the compiler for memory safety. You’ll still get some
// degree of safety inside of an unsafe block.
//
//In addition, unsafe does not mean the code inside the block is
// necessarily dangerous or that it will definitely have memory safety
// problems: the intent is that as the programmer, you’ll ensure the
// code inside an unsafe block will access memory in a valid way.
//
//People are fallible, and mistakes will happen, but by requiring these
// four unsafe operations to be inside blocks annotated with unsafe
// you’ll know that any errors related to memory safety must be within
// an unsafe block. Keep unsafe blocks small; you’ll be thankful later
// when you investigate memory bugs.
//
//To isolate unsafe code as much as possible, it’s best to enclose unsafe
// code within a safe abstraction and provide a safe API, which we’ll
// discuss later in the chapter when we examine unsafe functions and
// methods. Parts of the standard library are implemented as safe
// abstractions over unsafe code that has been audited. Wrapping unsafe
// code in a safe abstraction prevents uses of unsafe from leaking out
// into all the places that you or your users might want to use the
// functionality implemented with unsafe code, because using a safe
// abstraction is safe.
#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    //Let’s look at each of the four unsafe superpowers in turn. We’ll also
    // look at some abstractions that provide a safe interface to unsafe
    // code. Dereferencing a Raw Pointer
    //
    //In Chapter 4, in the “Dangling References” section, we mentioned that the
    // compiler ensures references are always valid. Unsafe Rust has two new
    // types called raw pointers that are similar to references. As with
    // references, raw pointers can be immutable or mutable and are written
    // as *const T and *mut T, respectively. The asterisk isn’t the
    // dereference operator; it’s part of the type name. In the context of
    // raw pointers, immutable means that the pointer can’t be directly
    // assigned to after being dereferenced.
    //
    //Different from references and smart pointers, raw pointers:
    //
    //    Are allowed to ignore the borrowing rules by having both immutable and
    // mutable pointers or multiple mutable pointers to the same location
    //    Aren’t guaranteed to point to valid memory
    //    Are allowed to be null
    //    Don’t implement any automatic cleanup
    //
    //By opting out of having Rust enforce these guarantees, you can give up
    // guaranteed safety in exchange for greater performance or the ability to
    // interface with another language or hardware where Rust’s guarantees don’t
    // apply.
    //
    //Listing 19-1 shows how to create an immutable and a mutable raw pointer
    // from references.
    //
    //
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    //
    //Listing 19-1: Creating raw pointers from references
    //
    //Notice that we don’t include the unsafe keyword in this code. We can
    // create raw pointers in safe code; we just can’t dereference raw
    // pointers outside an unsafe block, as you’ll see in a bit.
    //
    //We’ve created raw pointers by using as to cast an immutable and a mutable
    // reference into their corresponding raw pointer types. Because we created
    // them directly from references guaranteed to be valid, we know these
    // particular raw pointers are valid, but we can’t make that assumption
    // about just any raw pointer.
    //
    //Next, we’ll create a raw pointer whose validity we can’t be so certain
    // of. Listing 19-2 shows how to create a raw pointer to an arbitrary
    // location in memory. Trying to use arbitrary memory is undefined:
    // there might be data at that address or there might not, the compiler
    // might optimize the code so there is no memory access, or the program
    // might error with a segmentation fault. Usually, there is no good
    // reason to write code like this, but it is possible.
    //
    //
    let address = 0x012345usize;
    let r = address as *const i32;
    //
    //Listing 19-2: Creating a raw pointer to an arbitrary memory address
    //
    //Recall that we can create raw pointers in safe code, but we can’t
    // dereference raw pointers and read the data being pointed to. In
    // Listing 19-3, we use the dereference operator * on a raw pointer that
    // requires an unsafe block.
    //
    // [This code block contains unsafe code.]

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    //
    //Listing 19-3: Dereferencing raw pointers within an unsafe block
    //
    //Creating a pointer does no harm; it’s only when we try to access the
    // value that it points at that we might end up dealing with an invalid
    // value.
    //
    //Note also that in Listing 19-1 and 19-3, we created *const i32 and *mut
    // i32 raw pointers that both pointed to the same memory location, where
    // num is stored. If we instead tried to create an immutable and a
    // mutable reference to num, the code would not have compiled because
    // Rust’s ownership rules don’t allow a mutable reference at the same
    // time as any immutable references. With raw pointers, we can create a
    // mutable pointer and an immutable pointer to the same location and
    // change data through the mutable pointer, potentially creating a data
    // race. Be careful!
    //
    //With all of these dangers, why would you ever use raw pointers? One major
    // use case is when interfacing with C code, as you’ll see in the next
    // section, “Calling an Unsafe Function or Method.” Another case is when
    // building up safe abstractions that the borrow checker doesn’t
    // understand. We’ll introduce unsafe functions and then look at an
    // example of a safe abstraction that uses unsafe code. Calling an
    // Unsafe Function or Method
    //
    //The second type of operation that requires an unsafe block is calls to
    // unsafe functions. Unsafe functions and methods look exactly like
    // regular functions and methods, but they have an extra unsafe before
    // the rest of the definition. The unsafe keyword in this context
    // indicates the function has requirements we need to uphold when we
    // call this function, because Rust can’t guarantee we’ve
    // met these requirements. By calling an unsafe function within an unsafe
    // block, we’re saying that we’ve read this function’s documentation and
    // take responsibility for upholding the function’s contracts.
    //
    //Here is an unsafe function named dangerous that doesn’t do anything in
    // its body:
    //
    // [This code block contains unsafe code.]
    //
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
    //
    //We must call the dangerous function within a separate unsafe block. If we
    // try to call dangerous without the unsafe block, we’ll get an error:
    //
    //error[E0133]: call to unsafe function requires unsafe function or block
    // -->
    //  |
    //4 |     dangerous();
    //  |     ^^^^^^^^^^^ call to unsafe function
    //
    //By inserting the unsafe block around our call to dangerous, we’re
    // asserting to Rust that we’ve read the function’s documentation, we
    // understand how to use it properly, and we’ve verified that we’re
    // fulfilling the contract of the function.
    //
    //Bodies of unsafe functions are effectively unsafe blocks, so to perform
    // other unsafe operations within an unsafe function, we don’t need to
    // add another unsafe block. Creating a Safe Abstraction over Unsafe
    // Code
    //
    //Just because a function contains unsafe code doesn’t mean we need to mark
    // the entire function as unsafe. In fact, wrapping unsafe code in a
    // safe function is a common abstraction. As an example, let’s study a
    // function from the standard library, split_at_mut, that requires some
    // unsafe code and explore how we might implement it. This safe method
    // is defined on mutable slices: it takes one slice and makes it two by
    // splitting the slice at the index given as an argument. Listing 19-4
    // shows how to use split_at_mut.
    //
    //
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    //
    //Listing 19-4: Using the safe split_at_mut function
    //
    //We can’t implement this function using only safe Rust. An attempt might
    // look something like Listing 19-5, which won’t compile. For
    // simplicity, we’ll implement split_at_mut as a function rather than a
    // method and only for slices of i32 values rather than for a generic
    // type T.
    //
    // [This code does not compile!]
    //fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut
    // [i32]) {    let len = slice.len();
    //
    //    assert!(mid <= len);
    //
    //    (&mut slice[..mid],
    //     &mut slice[mid..])
    //}
    //
    //Listing 19-5: An attempted implementation of split_at_mut using only safe
    // Rust
    //
    //This function first gets the total length of the slice. Then it asserts
    // that the index given as a parameter is within the slice by checking
    // whether it’s less than or equal to the length. The assertion means
    // that if we pass an index that is greater than the index to split the
    // slice at, the function will panic before it attempts to use that
    // index.
    //
    //Then we return two mutable slices in a tuple: one from the start of the
    // original slice to the mid index and another from mid to the end of the
    // slice.
    //
    //When we try to compile the code in Listing 19-5, we’ll get an error.
    //
    //error[E0499]: cannot borrow `*slice` as mutable more than once at a time
    // -->
    //  |
    //6 |     (&mut slice[..mid],
    //  |           ----- first mutable borrow occurs here
    //7 |      &mut slice[mid..])
    //  |           ^^^^^ second mutable borrow occurs here
    //8 | }
    //  | - first borrow ends here
    //
    //Rust’s borrow checker can’t understand that we’re borrowing different
    // parts of the slice; it only knows that we’re borrowing from the same
    // slice twice. Borrowing different parts of a slice is fundamentally
    // okay because the two slices aren’t overlapping, but Rust isn’t smart
    // enough to know this. When we know code is okay, but Rust doesn’t,
    // it’s time to reach for unsafe code.
    //
    //Listing 19-6 shows how to use an unsafe block, a raw pointer, and some
    // calls to unsafe functions to make the implementation of split_at_mut
    // work.
    //
    // [This code block contains unsafe code.]
    //
    use std::slice;

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
            )
        }
    }
    //
    //Listing 19-6: Using unsafe code in the implementation of the split_at_mut
    // function
    //
    //Recall from “The Slice Type” section in Chapter 4 that slices are a
    // pointer to some data and the length of the slice. We use the len
    // method to get the length of a slice and the as_mut_ptr method to
    // access the raw pointer of a slice. In this case, because we have a
    // mutable slice to i32 values, as_mut_ptr returns a raw pointer with
    // the type *mut i32, which we’ve stored in the variable ptr.
    //
    //We keep the assertion that the mid index is within the slice. Then we get
    // to the unsafe code: the slice::from_raw_parts_mut function takes a
    // raw pointer and a length, and it creates a slice. We use this
    // function to create a slice that starts from ptr and is mid items
    // long. Then we call the offset method on ptr with mid as an argument
    // to get a raw pointer that starts at mid, and we create a slice using
    // that pointer and the remaining number of items after mid
    // as the length.
    //
    //The function slice::from_raw_parts_mut is unsafe because it takes a raw
    // pointer and must trust that this pointer is valid. The offset method on
    // raw pointers is also unsafe, because it must trust that the offset
    // location is also a valid pointer. Therefore, we had to put an unsafe
    // block around our calls to slice::from_raw_parts_mut and offset so we
    // could call them. By looking at the code and by adding the assertion
    // that mid must be less than or equal to len, we can tell that all the
    // raw pointers used within the unsafe block will be valid pointers to
    // data within the slice. This is an acceptable and appropriate use of
    // unsafe.
    //
    //Note that we don’t need to mark the resulting split_at_mut function as
    // unsafe, and we can call this function from safe Rust. We’ve created a
    // safe abstraction to the unsafe code with an implementation of the
    // function that uses unsafe code in a safe way, because it creates only
    // valid pointers from the data this function has access to.
    //
    //In contrast, the use of slice::from_raw_parts_mut in Listing 19-7 would
    // likely crash when the slice is used. This code takes an arbitrary memory
    // location and creates a slice 10,000 items long.
    //
    // [This code block contains unsafe code.]

    let address = 0x01234usize;
    let r = address as *mut i32;

    // -- This will probable cause a memory fault --
    //    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    //
    //Listing 19-7: Creating a slice from an arbitrary memory location
    //
    //We don’t own the memory at this arbitrary location, and there is no
    // guarantee that the slice this code creates contains valid i32 values.
    // Attempting to use slice as though it’s a valid slice results in
    // undefined behavior.
    //
    // Using extern Functions to Call External Code
    //
    //Sometimes, your Rust code might need to interact with code written in
    // another language. For this, Rust has a keyword, extern, that
    // facilitates the creation and use of a Foreign Function Interface
    // (FFI). An FFI is a way for a programming language to define functions
    // and enable a different (foreign) programming language to call those
    // functions.
    //
    //Listing 19-8 demonstrates how to set up an integration with the abs
    // function from the C standard library. Functions declared within
    // extern blocks are always unsafe to call from Rust code. The reason is
    // that other languages don’t enforce Rust’s rules and guarantees, and
    // Rust can’t check them, so responsibility falls on the programmer to
    // ensure safety.
    //
    //Filename: src/lib.rs
    //
    // [This code block contains unsafe code.]
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    //
    //Listing 19-8: Declaring and calling an extern function defined in another
    // language
    //
    //Within the extern "C" block, we list the names and signatures of external
    // functions from another language we want to call. The "C" part defines
    // which application binary interface (ABI) the external function uses:
    // the ABI defines how to call the function at the assembly level. The
    // "C" ABI is the most common and follows the C programming language’s
    // ABI.
    //
    //    Calling Rust Functions from Other Languages
    //
    //    We can also use extern to create an interface that allows other
    // languages to call Rust functions. Instead of an extern block, we add
    // the extern keyword and specify the ABI to use just before the fn
    // keyword. We also need to add a #[no_mangle] annotation to tell the
    // Rust compiler not to mangle the name of this function. Mangling is
    // when a compiler changes the name we’ve given a function to a
    // different name that contains more information for other parts
    // of the compilation process to consume but is less human readable. Every
    // programming language compiler mangles names slightly differently, so for
    // a Rust function to be nameable by other languages, we must disable
    // the Rust compiler’s name mangling.
    //
    //    In the following example, we make the call_from_c function accessible
    // from C code, after it’s compiled to a shared library and linked from
    // C:
    //
    //
    //    #[no_mangle]
    //    pub extern "C" fn call_from_c() {
    //        println!("Just called a Rust function from C!");
    //    }
    //
    //    This usage of extern does not require unsafe.
    //
    //Accessing or Modifying a Mutable Static Variable
    //
    //Until now, we’ve not talked about global variables, which Rust does
    // support but can be problematic with Rust’s ownership rules. If two
    // threads are accessing the same mutable global variable, it can cause
    // a data race.
    //
    //In Rust, global variables are called static variables. Listing 19-9 shows
    // an example declaration and use of a static variable with a string
    // slice as a value.
    //
    //Filename: src/lib.rs
    //
    static HELLO_WORLD: &str = "Hello, world!";
    //
    println!("name is: {}", HELLO_WORLD);
    //
    //Listing 19-9: Defining and using an immutable static variable
    //
    //Static variables are similar to constants, which we discussed in the
    // “Differences Between Variables and Constants” section in Chapter 3. The
    // names of static variables are in SCREAMING_SNAKE_CASE by convention,
    // and we must annotate the variable’s type, which is &'static str in
    // this example. Static variables can only store references with the
    // 'static lifetime, which means the Rust compiler can figure out the
    // lifetime; we don’t need to annotate it explicitly. Accessing an
    // immutable static variable is safe.
    //
    //Constants and immutable static variables might seem similar, but a subtle
    // difference is that values in a static variable have a fixed address in
    // memory. Using the value will always access the same data. Constants, on
    // the other hand, are allowed to duplicate their data whenever they’re
    // used.
    //
    //Another difference between constants and static variables is that static
    // variables can be mutable. Accessing and modifying mutable static
    // variables is unsafe. Listing 19-10 shows how to declare, access, and
    // modify a mutable static variable named COUNTER.
    //
    //Filename: src/lib.rs
    //
    // [This code block contains unsafe code.]
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
    //
    //Listing 19-10: Reading from or writing to a mutable static variable is
    // unsafe
    //
    //As with regular variables, we specify mutability using the mut keyword.
    // Any code that reads or writes from COUNTER must be within an unsafe
    // block. This code compiles and prints COUNTER: 3 as we would expect
    // because it’s single threaded. Having multiple threads access COUNTER
    // would likely result in data races.
    //
    //With mutable data that is globally accessible, it’s difficult to ensure
    // there are no data races, which is why Rust considers mutable static
    // variables to be unsafe. Where possible, it’s preferable to use the
    // concurrency techniques and thread-safe smart pointers we discussed in
    // Chapter 16 so the compiler checks that data accessed from different
    // threads is done safely. Implementing an Unsafe Trait
    //
    //The final action that works only with unsafe is implementing an unsafe
    // trait. A trait is unsafe when at least one of its methods has some
    // invariant that the compiler can’t verify. We can declare that a trait
    // is unsafe by adding the unsafe keyword before trait and marking the
    // implementation of the trait as unsafe too, as shown in Listing 19-11.
    //
    // [This code block contains unsafe code.]
    //
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
    //
    //Listing 19-11: Defining and implementing an unsafe trait
    //
    //By using unsafe impl, we’re promising that we’ll uphold the invariants
    // that the compiler can’t verify.
    //
    //As an example, recall the Sync and Send marker traits we discussed in the
    // “Extensible Concurrency with the Sync and Send Traits” section in Chapter
    // 16: the compiler implements these traits automatically if our types
    // are composed entirely of Send and Sync types. If we implement a type
    // that contains a type that is not Send or Sync, such as raw pointers,
    // and we want to mark that type as Send or Sync, we must use unsafe.
    // Rust can’t verify that our type upholds the guarantees that it can be
    // safely sent across threads or accessed from multiple threads;
    // therefore, we need to do those checks manually and indicate
    // as such with unsafe. When to Use Unsafe Code
    //
    //Using unsafe to take one of the four actions (superpowers) just discussed
    // isn’t wrong or even frowned upon. But it is trickier to get unsafe code
    // correct because the compiler can’t help uphold memory safety. When you
    // have a reason to use unsafe code, you can do so, and having the
    // explicit unsafe annotation makes it easier to track down the source
    // of problems if they occur.
}
