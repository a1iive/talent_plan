use std::cmp::Ordering;
use std::string::ToString;
// key max size 256 Bytes
// while value max size 4KB
pub type KeyType = [u8;256];
pub type ValueType = [u8;4096];

#[derive(Clone,Copy)]
pub struct Key{
    pub data:KeyType,
}
#[derive(Clone,Copy)]
pub struct Value{
    // pub tag:u8,
    pub data:ValueType,
}

impl Key{
    pub fn from_str(s: &str) -> Self{
        let mut key = [0; 256];
        let chars = s.as_bytes();
        key[..s.len()].clone_from_slice(&chars[..s.len()]);
        Key { data: key }
    }
}
impl Value{
    pub fn from_str(s: &str) -> Self{
        let mut value = [0; 4096];
        let chars = s.as_bytes();
        value[..s.len()].clone_from_slice(&chars[..s.len()]);
        // Value { tag:1 , data: value }
        Value { data: value }
    }
}

impl ToString for Key{
    fn to_string(&self) -> String {
        self.data
            .iter()
            .cloned()
            .map(|x| x as char)
            .collect::<String>()
            .trim_matches(char::from(0))
            .to_owned()
    }
}

impl ToString for Value{
    fn to_string(&self) -> String {
        self.data
            .iter()
            .cloned()
            .map(|x| x as char)
            .collect::<String>()
            .trim_matches(char::from(0))
            .to_owned()
    }
}
impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Key) -> Option<Ordering> {
        Some(self.data.cmp(&other.data))
    }
}
impl Ord for Key {
    fn cmp(&self,other:&Key)->Ordering {
        self.data.cmp(&other.data)
    }
}
impl PartialEq for Key {
    fn eq(&self, other: &Key) -> bool {
        if self.data.len()==other.data.len(){
            let mut i = 0;
            while i < self.data.len(){
                if self.data[i]==other.data[i]{
                    i+=1;
                }
                else {
                    return false;
                }
            }
            true
        }
        else {
            false
        }
    }
}
impl Eq for Key{}