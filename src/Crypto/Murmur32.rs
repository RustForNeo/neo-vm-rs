use std::num::Wrapping;
use std::convert::TryInto;

const C1: u32 = 0xcc9e2d51;
const C2: u32 = 0x1b873593;
const R1: u32 = 15;
const R2: u32 = 13;
const M: u32 = 5;
const N: u32 = 0xe6546b64;

#[derive(Default)]
pub struct Murmur32 {
    seed: u32,
    hash: u32,
    length: u32,
}

impl Murmur32 {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }

    fn hash_core(&mut self, data: &[u8]) {
        let len = data.len() as u32;
        self.length += len;

        let remainder = len & 3;
        let aligned_length = len - remainder;
        for chunk in data.chunks(4).take(aligned_length as usize / 4) {
            let mut k = Wrapping(u32::from_le_bytes(chunk.try_into().unwrap()));
            k *= Wrapping(C1);
            k = Wrapping((k.0 << R1) | (k.0 >> (32 - R1)));
            k *= Wrapping(C2);
            self.hash ^= k.0;
            self.hash = (self.hash << Wrapping(R2).0) | (self.hash >> Wrapping(32 - R2).0);
            self.hash = self.hash.wrapping_mul(M).wrapping_add(N);
        }

        if remainder > 0 {
            let mut remaining_bytes = 0;
            for i in 0..remainder {
                remaining_bytes ^= (data[aligned_length as usize + i as usize] as u32) << (i * 8);
            }
            let mut k = Wrapping(remaining_bytes);
            k *= Wrapping(C1);
            k = Wrapping((k.0 << R1) | (k.0 >> (32 - R1)));
            k *= Wrapping(C2);
            self.hash ^= k.0;
        }
    }
}

impl std::hash::Hasher for Murmur32 {
    fn finish(&self) -> u64 {
        let mut hash = self.hash;
        let len = self.length;

        hash ^= len;
        hash ^= hash >> 16;
        hash = hash.wrapping_mul(0x85ebca6b);
        hash ^= hash >> 13;
        hash = hash.wrapping_mul(0xc2b2ae35);
        hash ^= hash >> 16;

        hash as u64
    }

    fn write(&mut self, data: &[u8]) {
        self.hash_core(data);
    }
}


#[cfg(test)]
mod tests {
    use std::hash::Hasher;
    use super::*;

    #[test]
    fn test_murmur32() {
        let mut hasher = Murmur32::new(123456);
        let data = "Hello, world!".as_bytes();
        hasher.write(data);
        let expected: u64 = 3284962968;
        assert_eq!(hasher.finish(), expected as u64);

        let mut hasher = Murmur32::new(654321);
        let data = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer bibendum augue sit amet tortor luctus vestibulum. Sed hendrerit augue vel neque feugiat ultricies. Fusce feugiat augue a elit dignissim, eu efficitur libero ullamcorper. Aenean eget velit tortor. Nulla facilisi. Integer sit amet lobortis lectus. Integer semper, augue et facilisis hendrerit, neque urna vulputate orci, a bibendum metus diam ac nunc. Vestibulum feugiat interdum metus, sit amet fermentum velit malesuada ut. Nulla facilisi. Praesent sodales nibh in lorem viverra, a lacinia enim cursus. Pellentesque nec tincidunt mauris. Vivamus viverra, nulla eu ullamcorper congue, massa mi malesuada sapien, vitae fermentum sapien felis ut nisl. Quisque nec libero ut tellus imperdiet maximus eget a mauris.".as_bytes();
        hasher.write(data);
        let expected: u64 = 3940501148;
        assert_eq!(hasher.finish(), expected as u64);
    }
}
