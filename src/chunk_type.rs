use std::fmt::Display;
use std::convert::TryFrom;
use std::str::FromStr;
//use crate::{Error,Result};
/*
0关键 大写 1 辅助 小写
0公开 大写 1 私有 小写
必须大写
0复制不安全  大写 1  复制安全 小写
*/
#[derive(Debug,Clone,PartialEq,Eq)]
struct  ChunkType{
 Ancillary :u8,
 Private :u8,
 Reserved :u8,
 Safetocopy:u8,
}

//isbitzero
fn isbitzero(n:u8,p:u8)->bool{
    let rm =  n >>p;
    (rm&1)==0 
}

impl ChunkType{

    fn bytes(&self)->[u8;4]{
          let vbytes = [self.Ancillary,self.Private,self.Reserved,self.Safetocopy];
          vbytes
    }
//检查是否为大小写字母且第三位是否为大写
    fn is_valid(&self)->bool{
       for bia in self.bytes(){
        if !bia.is_ascii(){
            return false;
        }
        if !bia.is_ascii_uppercase()&&!bia.is_ascii_lowercase() {
            return false;
        }
    }
        let bia2 = self.Reserved;
        if bia2.is_ascii_lowercase(){
            return false;
        }
           true
    }

    fn is_valid_to_convert(&self)->bool{
        for bia in self.bytes(){
         if !bia.is_ascii(){
             return false;
         }
         if !bia.is_ascii_uppercase()&&!bia.is_ascii_lowercase() {
             return false;
         }
     }
            true
     }

//是否关键
    fn is_critical(&self)->bool{
       isbitzero(self.Ancillary, 5)
    }

    fn is_public(&self)->bool{
        isbitzero(self.Private, 5)
    }

    fn is_reserved_bit_valid(&self)->bool{
        isbitzero(self.Reserved, 5)
    }

    fn is_safe_to_copy(&self)->bool{
        !isbitzero(self.Safetocopy, 5)
    }
}



impl TryFrom<[u8;4]> for ChunkType{
    type Error = &'static str;
   fn try_from(bytes: [u8;4]) -> Result<Self, Self::Error> {
 let ct = ChunkType{
    Ancillary:bytes[0],
     Private:bytes[1],
     Reserved:bytes[2],
     Safetocopy:bytes[3],
     };

       if Self::is_valid_to_convert(&ct){
        Ok(ct)
       }else{
        Err("invalid letters")
       }
      
   }
}

impl FromStr for ChunkType{
    type Err =&'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len()!=4{
          return  Err("invalid length");
        }
       let sbytes = s.bytes();
       let mut bytes:[u8;4] = [0;4];
       for (i,b) in sbytes.enumerate(){
        bytes[i] = b;
       }
       let ct = ChunkType{
        Ancillary:bytes[0],
         Private:bytes[1],
         Reserved:bytes[2],
         Safetocopy:bytes[3],
         };
         if Self::is_valid_to_convert(&ct){
            Ok(ct)
           }else{
            Err("invalid letters")
           }

    }
}

impl Display for ChunkType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",String::from_utf8_lossy(&self.bytes()))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}




