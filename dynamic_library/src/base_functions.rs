use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn double_of(v: i32) -> i32 {
    v * 2
}

#[no_mangle]
pub extern "C" fn to_upper(letters: *mut libc::c_char) {
    let mut offset = 0;
    loop {
        let letter = unsafe { *letters.offset(offset) } as u8 as char;
        if letter == '\0' {
            break;
        }
        let upper = letter.to_ascii_uppercase();
        unsafe {
            *letters.offset(offset) = upper as u8 as i8;
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
    letters: *mut libc::c_char,
) -> FindMostCommonLetterResult {
    let mut occurrences = HashMap::new();
    let mut offset = 0;
    loop {
        let letter = unsafe { *letters.offset(offset) } as u8 as char;
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
            letter: letter as u8 as i8,
            occurrences: occurr,
        })
        .unwrap_or(FindMostCommonLetterResult {
            letter: 'a' as u8 as i8,
            occurrences: 0,
        })
}
