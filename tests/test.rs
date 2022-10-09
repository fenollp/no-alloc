use assert_no_alloc::assert_no_alloc;
use no_alloc::no_alloc;

fn do_alloc() {
    let _tmp: Box<u32> = Box::new(42);
}

// use trybuild

#[test]
fn fnfnfn() {
    #[no_alloc]
    {
        assert_no_alloc(|| {
            do_alloc();
        })
    }
}

#[test]
fn fnfnfnfn() {
    #[no_alloc]
    let _tmp = assert_no_alloc(|| -> Box<u32> {
        // ...
        // ...    Imagine actual work here
        // ...
        Box::new(42)
    });
}

#[test]
fn forbidden_simple_attribute() {
    #[no_alloc]
    fn func() {
        assert_no_alloc(|| {
            do_alloc();
        });
    }

    // assert_eq!(check_and_reset(), false);
    func();
    // assert_eq!(check_and_reset(), true);
}
