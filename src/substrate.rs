use crate::Fixed;
use parity_scale_codec::{Decode, Encode, EncodeLike, Error, Input, MaxEncodedLen, Output};
use scale_info::TypeInfo;

impl TypeInfo for Fixed {
    type Identity = <i128 as TypeInfo>::Identity;
    fn type_info() -> scale_info::Type {
        <i128 as TypeInfo>::type_info()
    }
}

impl Decode for Fixed {
    fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
        let bits = <i128 as Decode>::decode(input)?;
        Ok(Self::from_bits(bits))
    }
    fn skip<I: Input>(input: &mut I) -> Result<(), Error> {
        <i128 as Decode>::skip(input)
    }
    fn encoded_fixed_size() -> Option<usize> {
        <i128 as Decode>::encoded_fixed_size()
    }
}

impl Encode for Fixed {
    fn size_hint(&self) -> usize {
        self.0.to_bits().size_hint()
    }
    fn encode_to<T: Output + ?Sized>(&self, dest: &mut T) {
        self.0.to_bits().encode_to(dest);
    }
    fn encode(&self) -> alloc::vec::Vec<u8> {
        self.0.to_bits().encode()
    }
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        self.0.to_bits().using_encoded(f)
    }
    fn encoded_size(&self) -> usize {
        self.0.to_bits().encoded_size()
    }
}

impl EncodeLike<Fixed> for Fixed {}

impl MaxEncodedLen for Fixed {
    fn max_encoded_len() -> usize {
        <i128 as MaxEncodedLen>::max_encoded_len()
    }
}
