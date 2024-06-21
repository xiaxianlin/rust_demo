fn opaque_read(val: &i32) {
    println!("{}", val);
}

fn main() {
    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;

        *ref1 += 1;
        *ptr2 += 2;

        println!("{}", data);
    }

    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;
        let ref3 = &mut *ptr2;
        let ptr4 = ref3 as *mut _;

        *ptr4 += 4;
        *ref3 += 3;
        *ptr2 += 2;
        *ref1 += 1;

        println!("{}", data);
    }

    unsafe {
        let mut data = [0; 10];
        let ref1_at_0 = &mut data[0]; // 获取第 1 个元素的引用
        let ptr2_at_0 = ref1_at_0 as *mut i32; // 裸指针 ptr 指向第 1 个元素
        let ptr3_at_1 = ptr2_at_0.add(1); // 对裸指针进行运算，指向第 2 个元素

        *ptr3_at_1 += 3;
        *ptr2_at_0 += 2;
        *ref1_at_0 += 1;

        // Should be [3, 3, 0, ...]
        println!("{:?}", &data[..]);
    }

    unsafe {
        let mut data = [0; 10];

        let slice1_all = &mut data[..]; // Slice for the entire array
        let ptr2_all = slice1_all.as_mut_ptr(); // Pointer for the entire array

        let ptr3_at_0 = ptr2_all; // Pointer to 0th elem (the same)
        let ptr4_at_1 = ptr2_all.add(1); // Pointer to 1th elem
        let ref5_at_0 = &mut *ptr3_at_0; // Reference to 0th elem
        let ref6_at_1 = &mut *ptr4_at_1; // Reference to 1th elem

        *ref6_at_1 += 6;
        *ref5_at_0 += 5;
        *ptr4_at_1 += 4;
        *ptr3_at_0 += 3;

        // 在循环中修改所有元素( 仅仅为了有趣 )
        // (可以使用任何裸指针，它们共享同一个借用!)
        for idx in 0..10 {
            *ptr2_all.add(idx) += idx;
        }

        // 同样为了有趣，再实现下安全版本的循环
        for (idx, elem_ref) in slice1_all.iter_mut().enumerate() {
            *elem_ref += idx;
        }

        // Should be [8, 12, 4, 6, 8, 10, 12, 14, 16, 18]
        println!("{:?}", &data[..]);
    }

    unsafe {
        let mut data = 10;
        let mref1 = &mut data;
        let ptr2 = mref1 as *mut i32;
        let sref3 = &*mref1;
        let ptr4 = sref3 as *const i32 as *mut i32;

        opaque_read(&*ptr4);
        opaque_read(sref3);
        *ptr2 += 2;
        *mref1 += 1;

        opaque_read(&data);

        opaque_read(&data);
    }
}
