pub trait Pixel:Clone+Copy+Default{
    fn to_u8(&self)->Vec<u8>;
    fn to_u8_with(&self, vec:&mut Vec<u8>);
}