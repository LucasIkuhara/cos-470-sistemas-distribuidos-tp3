#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Request,
    Grant,
    Release,
}

impl From<u8> for Operation {
    fn from(num: u8) -> Self {
        match num {
            1 => Operation::Request,
            2 => Operation::Grant,
            3 => Operation::Release,
            _ => panic!("Invalid operation"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Request {
    pub operation: Operation,
    pub id: u32,
}

impl From<[u8; 5]> for Request {
    fn from(bytes: [u8; 5]) -> Self {
        Request {
            operation: Operation::from(bytes[0]),
            id: u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]),
        }
    }
}

impl Request {
    pub fn as_bytes(&self) -> [u8; 5] {
        let mut bytes = [0; 5];
        bytes[0] = match self.operation {
            Operation::Request => 1,
            Operation::Grant => 2,
            Operation::Release => 3,
        };
        bytes[1..5].copy_from_slice(&self.id.to_be_bytes());
        bytes
    }
}
