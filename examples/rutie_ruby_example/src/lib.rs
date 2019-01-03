#[macro_use]
extern crate rutie;

use rutie::{Class, Object, RString, Thread, VM, NilClass};
use std::fs::File;

class!(RutieExample);

methods! {
    RutieExample,
    _itself,
    fn heap_allocated_returning_input() -> NilClass {
        let input = "Object".to_string();
        let handler = move || {
            // Trying to make sure the compiler doesn't optimize the whole function away
            let metadata = File::open("/Users/dominik/code/rust/rutie/examples/rutie_ruby_example/fail.rb").unwrap().metadata().unwrap();
            metadata.len();
            println!("inside closure: '{}'", input);
            input
        };
        let ret = Thread::call_without_gvl(handler, Some(|| {}));
        println!("returned value: '{}'", ret);
        return NilClass::new();
    }
    // Output:
    // inside closure: 'Object'
    // returned value: '���c�'
    // ruby(23643,0x115ba85c0) malloc: *** error for object 0x7ff63ca123d0: pointer being freed was not allocated
    // ruby(23643,0x115ba85c0) malloc: *** set a breakpoint in malloc_error_break to debug
    //
    // Somehow the pointer to the string gets messed up when returned from the closure, after `ret` gets
    // out of scope rust tried to deallocate some object that does not exist (at least not at the
    // address that is freed)

    fn stack_allocated_returning_input() -> NilClass {
        let input = 42;
        let handler = move || {
            // Trying to make sure the compiler doesn't optimize the whole function away
            let metadata = File::open("/Users/dominik/code/rust/rutie/examples/rutie_ruby_example/fail.rb").unwrap().metadata().unwrap();
            metadata.len();
            println!("inside closure: '{}'", input);
            input
        };
        let ret = Thread::call_without_gvl(handler, Some(|| {}));
        println!("returned value: '{}'", ret);
        return NilClass::new();
    }
    // Output:
    // inside closure: '-759049584'
    // returned value: '-759049584'
    //
    // Similar behavior as in `heap_allocated_returning_input` but since nothing needs to be freed
    // we just get a random value from somewhere on the stack(?)

    fn heap_allocated_returning_from_closure() -> NilClass {
        let input = "Object".to_string();
        let handler = move || {
            // Trying to make sure the compiler doesn't optimize the whole function away
            let metadata = File::open("/Users/dominik/code/rust/rutie/examples/rutie_ruby_example/fail.rb").unwrap().metadata().unwrap();
            println!("inside closure: '{}'", input);
            metadata.len()
        };
        let ret = Thread::call_without_gvl(handler, Some(|| {}));
        println!("returned value: '{}'", ret);
        return NilClass::new();
    }
    // Output:
    // inside closure: 'Object'
    // ruby(25217,0x1016035c0) malloc: *** error for object 0x7fe46e51d6c0: pointer being freed was not allocated
    // ruby(25217,0x1016035c0) malloc: *** set a breakpoint in malloc_error_break to debug
    //
    // The free is called somewhere in `util::ptr_to_data`, I assume it is trying to free the
    // `input` before returning the result from the closure (which makes sense). However I don't
    // understand why the free is called for the wrong pointer as the value of `input` inside the
    // closure looks correct.

    fn stack_allocated_returning_from_closure() -> NilClass {
        let input = 42;
        let handler = move || {
            // Trying to make sure the compiler doesn't optimize the whole function away
            let metadata = File::open("/Users/dominik/code/rust/rutie/examples/rutie_ruby_example/fail.rb").unwrap().metadata().unwrap();
            println!("inside closure: '{}'", input);
            metadata.len()
        };
        let ret = Thread::call_without_gvl(handler, Some(|| {}));
        println!("returned value: '{}'", ret);
        return NilClass::new();
    }
    // Output:
    // inside closure: '623171904'
    // returned value: '79'
    //
    // Again the input is not moved correctly to the closure but returning from the closure works as
    // expected.
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_rutie_ruby_example() {
    Class::new("RutieExample", None).define(|itself| {
        itself.def_self("stack_allocated_returning_input", stack_allocated_returning_input);
        itself.def_self("stack_allocated_returning_from_closure", stack_allocated_returning_from_closure);
        itself.def_self("heap_allocated_returning_input", heap_allocated_returning_input);
        itself.def_self("heap_allocated_returning_from_closure", heap_allocated_returning_from_closure);
    });
}
