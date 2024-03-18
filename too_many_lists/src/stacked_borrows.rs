
// KEYPOINT: 堆叠引用场景有时候是合法的 需要进一步使用miri进行检查

#[cfg(test)]
mod test{
    // #[test]
    // fn stacked_borrows(){
    //     let mut data = 10;
    //     let ref1 = &mut data;
    //     let ref2 = &mut *ref1;
    //
    //     // ORDER SWAPPED!
    //     *ref1 += 1;
    //     *ref2 += 2;
    //
    //     println!("{}", data);
    // }

    // cargo test ok
    // but cargo +nightly miri test error
    #[test]
    fn stacked_borrows2(){
        unsafe {
            let mut data = 10;
            let ref1 = &mut data;
            let ptr2 = ref1 as *mut _;

            // ORDER SWAPPED!
            *ref1 += 1;
            *ptr2 += 2;

            println!("{}", data);
        }
    }

    #[test]
    fn stacked_borrows3(){
        unsafe {
            let mut data = 10;
            let ref1 = &mut data;
            let ptr2 = ref1 as *mut _;
            let ref3 = &mut *ptr2;
            let ptr4 = ref3 as *mut _;

            // Access the first raw pointer first
            *ptr2 += 2;

            // Then access things in "borrow stack" order
            *ptr4 += 4;
            *ref3 += 3;
            *ptr2 += 2;
            *ref1 += 1;

            println!("{}", data);
        }
    }

    #[test]
    fn stacked_borrows4(){
        unsafe {
            let mut data = [0; 10];
            let ref1_at_0 = &mut data[0];           // Reference to 0th element
            let ptr2_at_0 = ref1_at_0 as *mut i32;  // Ptr to 0th element
            let ptr3_at_1 = ptr2_at_0.add(1);       // Ptr to 1st element

            *ptr3_at_1 += 3;
            *ptr2_at_0 += 2;
            *ref1_at_0 += 1;

            // Should be [3, 3, 0, ...]
            println!("{:?}", &data[..]);
        }

    }
}