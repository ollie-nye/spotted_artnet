pub struct ArtHeader {
    id: [u8; 8],
    opcode: u16,
    protocol_version: u16,
}

impl ArtHeader {
    pub fn new(opcode: u16) -> ArtHeader {
        let id: [u8; 8] = id();

        let protocol_version: u16 = 0x000e;

        ArtHeader {
            id,
            opcode,
            protocol_version,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.id.iter().cloned());
        out.extend(self.opcode.to_be_bytes().iter().cloned());
        out.extend(self.protocol_version.to_be_bytes().iter().cloned());

        out
    }

    pub fn opcode(&self) -> u16 {
        self.opcode
    }
}

pub struct ArtHeaderPartial {
    id: [u8; 8],
    opcode: u16,
}

impl ArtHeaderPartial {
    pub fn new(opcode: u16) -> ArtHeaderPartial {
        let id: [u8; 8] = id();

        ArtHeaderPartial { id, opcode }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.id.iter().cloned());
        out.extend(self.opcode.to_be_bytes().iter().cloned());

        out
    }
}

pub fn id() -> [u8; 8] {
    let mut id: [u8; 8] = [0x00; 8];

    id.copy_from_slice("Art-Net\0".as_bytes());

    id
}

#[cfg(test)]
#[test]
fn test_new_opcode() {
    let header = ArtHeader::new(0x1234);
    assert_eq!(header.opcode, 0x1234);
}

#[test]
fn test_new_protocol_version() {
    let header = ArtHeader::new(0x1234);
    assert_eq!(header.protocol_version, 0x000e);
}

#[test]
fn test_serialize_id() {
    let header = ArtHeader::new(0x1234);
    let bytes = header.serialize();
    assert_eq!(bytes[0..8], [65, 114, 116, 45, 78, 101, 116, 0]);
}

#[test]
fn test_serialize_opcode() {
    let header = ArtHeader::new(0x1234);
    let bytes = header.serialize();
    assert_eq!(bytes[8..10], [0x12, 0x34]);
}

#[test]
fn test_serialize_protocol_version() {
    let header = ArtHeader::new(0x1234);
    let bytes = header.serialize();
    assert_eq!(bytes[10..12], [0x00, 0x0e]);
}

#[test]
fn test_opcode() {
    let header = ArtHeader::new(0x1234);
    assert_eq!(header.opcode(), 0x1234);
}

#[test]
fn test_id() {
    assert_eq!(id(), [65, 114, 116, 45, 78, 101, 116, 0]);
}
