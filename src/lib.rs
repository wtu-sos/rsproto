mod base;

pub enum SprotoType {
    Request = 0,
    Response = 1,
}

pub enum SprotoFiledType {
    TInteger = 0,
    TBoolean = 1,
    TString = 2,
    TStruct = 3,
}

pub enum SubTypeOfString {
    TStringString = 0,
    TStringBinary = 1,
}

#[allow(dead_code)]
const SPROTO_ARRAY: u8 = 0x80;
#[allow(dead_code)]
const SIZEOF_LENGTH: usize = 4;
#[allow(dead_code)]
const SIZEOF_HEADER: usize = 2;
#[allow(dead_code)]
const SIZEOF_FIELD: usize = 2;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
