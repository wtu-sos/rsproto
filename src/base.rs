use crate::*;
use std::convert::TryInto;

#[allow(dead_code)]
pub fn to_word(bs: [u8; 2]) -> u16 {
    u16::from_le_bytes(bs)
}

#[allow(dead_code)]
pub fn to_dword(bs: [u8; 4]) -> u32 {
    u32::from_le_bytes(bs)
}

#[allow(dead_code)]
pub fn count_array(bs: &[u8]) -> i32 {
    let mut n = 0i32;
    let mut index = 0usize;
    if let Some(len_bytes) = bs.get(0 .. SIZEOF_LENGTH) {
        let mut len = to_dword(len_bytes.try_into().expect("slice with incorrect length"));
        index += SIZEOF_LENGTH;
        
        while len > 0 {
            if len < SIZEOF_LENGTH as u32 {
                return -1;
            }

            if let Some(ns_bytes) = bs.get(index .. index + SIZEOF_LENGTH) {
                let mut nsz = to_dword(ns_bytes.try_into().expect("slice with incorrect length"));
                nsz += SIZEOF_LENGTH as u32;
                if nsz > len {
                    return -1;
                }

                n += 1;
                len -= nsz;
                index += nsz as usize;
            } else {
                return -1;
            }
        }
    }
    return n; 
}

#[allow(dead_code)]
pub fn struct_filed(bs: &[u8]) -> i32 {
    let mut field_num = 0usize;
    let sz = bs.len();
    if sz < SIZEOF_LENGTH {
        return -1;
    }

    field_num = to_word(bs.get(0 .. 2).unwrap().try_into().expect("slice with incorrect length")) as usize;
    let field_part_len = SIZEOF_FIELD * field_num;
    let header = SIZEOF_HEADER + field_part_len;
    if sz < header {
        return -1;
    }

    let (field_part, data_part) = bs[SIZEOF_HEADER..].split_at(field_part_len);
    let mut data_index = 0usize;
    let mut data_size = data_part.len();
    for i in 0 .. field_num {
        let value = to_word(field_part[i*SIZEOF_FIELD .. (i+1)*SIZEOF_FIELD].try_into().expect(""));
        if value != 0 {
            continue;
        }
        if data_size < SIZEOF_LENGTH {
            return -1;
        }
        let dsz = to_dword(data_part[data_index..data_index+SIZEOF_LENGTH].try_into().expect(""));
        data_size -= SIZEOF_LENGTH;
        if data_size < dsz as usize {
            return -1;
        }
        data_size -= dsz as usize;
        data_index = data_index + SIZEOF_LENGTH + dsz as usize;
    }
    
    return field_num as i32;
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]    
    pub fn test_struct_field() {
        let raw_proto = [0x03, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x02, 0x00, 0x05, 0x00,0x00,0x00, 0x41, 0x6C, 0x69, 0x63, 0x65];
        let field_num = struct_filed(raw_proto.as_ref());
        assert_eq!(field_num, 3);
    }

    #[test]    
    pub fn test_struct_field_not_finished() {
        let raw_proto = [0x03, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x02, 0x00, 0x05, 0x00,0x00,0x00, 0x41, 0x6C, 0x69];
        let field_num = struct_filed(raw_proto.as_ref());
        assert_eq!(field_num, -1);
    }

    #[test]    
    pub fn test_struct_field_lack_field() {
        let raw_proto = [0x03, 0x00, 0x00];
        let field_num = struct_filed(raw_proto.as_ref());
        assert_eq!(field_num, -1);
    }
}
