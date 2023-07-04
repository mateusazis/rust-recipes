use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn double_of(v: i32) -> i32 {
    v * 2
}

#[no_mangle]
pub extern "C" fn to_upper(letters: *mut libc::c_char) {
    let mut offset = 0;
    loop {
        let letter = char::from(unsafe { *letters.offset(offset) } as u8);
        if letter == '\0' {
            break;
        }
        let upper = letter.to_ascii_uppercase();
        let mut arr = [0u8; 1];
        upper.encode_utf8(&mut arr);
        unsafe {
            *letters.offset(offset) = arr[0] as libc::c_char;
        }
        offset += 1;
    }
}

#[repr(C)]
pub struct FindMostCommonLetterResult {
    pub letter: libc::c_char,
    pub occurrences: i32,
}

#[no_mangle]
pub extern "C" fn find_most_common_letter(
    letters: *const libc::c_char,
) -> FindMostCommonLetterResult {
    let mut occurrences = HashMap::new();
    let mut offset = 0;
    loop {
        let letter = char::from(unsafe { *letters.offset(offset) } as u8);
        if letter == '\0' {
            break;
        }
        occurrences.insert(letter, occurrences.get(&letter).unwrap_or(&0) + 1);
        offset += 1;
    }

    occurrences
        .into_iter()
        .max_by(|(_, ocurrences1), (_, ocurrences2)| ocurrences1.cmp(ocurrences2))
        .map(|(letter, occurr)| FindMostCommonLetterResult {
            letter: letter as u8 as libc::c_char,
            occurrences: occurr,
        })
        .unwrap_or(FindMostCommonLetterResult {
            letter: 'a' as u8 as libc::c_char,
            occurrences: 0,
        })
}
