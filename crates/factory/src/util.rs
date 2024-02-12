// Copyright 2024 Natalie Baker // AGPLv3 //

pub fn insert_into<T: Copy>(arr: &mut [T], idx: usize, v: T) -> bool {
    if idx >= arr.len() {
        return false;
    }

    // Safety: We're moving the array backwards by 1 index.
    unsafe {
        core::ptr::copy(
            arr.as_mut_ptr().add(idx  ), 
            arr.as_mut_ptr().add(idx+1), 
            arr.len() - (idx + 1)
        );
    }

    arr[idx] = v;
    true
}

pub fn remove_from<T: Copy>(arr: &mut [T], idx: usize) -> Option<T> {
    if idx >= arr.len() {
        return None;
    }
    let result = arr[idx];
    // Safety: We're moving the array forwards by 1 index. 
    unsafe {
        core::ptr::copy(
            arr.as_mut_ptr().add(idx+1), 
            arr.as_mut_ptr().add(idx  ), 
            arr.len() - (idx + 1)
        );
    }
    Some(result)
}
