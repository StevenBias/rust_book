# Cheatsheet for rust book


## Common Cargo commands
### Build a project
```
cargo build
```

Check code to be sure it complies, does not create an executable
```
cargo check
```


### Build and run a project
```
cargo run
```

#### Build and run a project with an environment variable
For instance with variable named "ENV_VAR" equal to 1:
```
ENV_VAR=1 cargo run
```

### Automated tests
#### Run all tests
```
cargo test
```
By default, tests run in parrallel with several threads.\
To run tests consecutively, use only one thread:
```
cargo test -- --test-threads=1
```
#### Showing function output
By default, if the test is ok, no output is printed.\
To show print:
```
cargo test --nocapture
```

#### Run only ignored tests
```
cargo test -- --ignored
```

### Build for release
The default directory of the build result is in "target/debug" and the the compulation is not optimized.

To build a release with optimization, the command is:
```
cargo build --release
```
The result will be in "target/release"

### Update crate
The update of the cargo will update the "Cargo.lock" file.
A cargo command allow you to check for minor new version (0.3.11 -> 0.3.15 for instance).
This command is:
```
cargo update
```

But if you want to update newer version, you have to update the "Cargo.toml" file.


### Module System
**Packages** A Cargo feature that lets you build, test, and share crates

**Crates** A tree of modules that produces a library or executable

**Modules** and use Let you control the organization, scope, and privacy
of paths

**Paths** A way of naming an item, such as a struct, function, or module


### Use panic backtrace
To get a backtrace of an error:
```
RUST_BACKTRACE=1 cargo run
```


## Smart pointers
The three most common smart pointers are:
- **Box\<T\>** for allocating values on the heap
- **Rc\<T\>** , a reference counting type that enables multiple ownership
- **Ref\<T\>** and **RefMut\<T\>** , accessed through **RefCell\<T\>** , a type that enforces the borrowing rules at runtime instead of compile time

### Box\<T\>
Boxes are most often used in these situations:
- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

### [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)
The inherent methods of *Rc* are all associated functions, 
which means that you have to call them as e.g., **Rc::get_mut(&mut value)** instead of 
*value.get_mut()*. This avoids conflicts with methods of the inner type *T*.

#### [Weak](https://doc.rust-lang.org/std/rc/struct.Weak.html)
Weak is a version of **Rc** that holds a non-owning reference to the managed allocation. The allocation is accessed by calling **upgrade** on the Weak pointer, which returns an **Option<Rc<T>>**.

A *Weak* pointer is useful for keeping a temporary reference to the allocation managed by **Rc** without preventing its inner value from being dropped. It is also used to prevent circular references between **Rc** pointers, since mutual owning references would never allow either **Rc** to be dropped. For example, a tree could have strong **Rc** pointers from parent nodes to children, and *Weak* pointers from children back to their parents.

The typical way to obtain a *Weak* pointer is to call **Rc::downgrade**.

Constructs a new *Weak<T>*, without allocating any memory. Calling **upgrade** on the return value always gives **None**.\
Example:
```
use std::rc::Weak;

let empty: Weak<i64> = Weak::new();
assert!(empty.upgrade().is_none());
```
### RefCell\<T\>
Similar to *Rc<T>*, *RefCell<T>* is only for **use in single-threaded scenarios** and will give you a compile-time error if you try using it in a multithreaded context.\
With *RefCell*, the borrowing rules are enforced at running time.\
So if the rules are broken, it will compile but the program will panic and exit!\
Therefore, particularities of *RefCell* are:
- At any given time, you can have either but not both of the following: one mutable reference or any number of immutable references.
- References must always be valid.

### Treating a Type Like a Reference by Implementing the Deref Trait
Implement the *Deref* function for the custom smart pointer.
```
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    u type Target = T;
    fn deref(&self) -> &T {
        v &self.0
    }
}
```
Rust does deref coercion when it finds types and trait implementations
in three cases:
- From *&T* to *&U* when **T: Deref<Target=U>**
- From &mut *T* to *&mut U* when **T: DerefMut<Target=U>**
- From &mut *T* to *&U* when **T: Deref<Target=U>**

### Recap of smart pointers
Here is a recap of the reasons to choose **Box<T>**, **Rc<T>**, or **RefCell<T>**:
- Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
- Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
- Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is
immutable.

