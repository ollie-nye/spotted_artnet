use super::header::art_header::ArtHeader;
use super::header::opcode;

pub struct ArtSync {
    header: ArtHeader,
    aux1: u8,
    aux2: u8,
}

impl ArtSync {
    pub fn new(aux1: u8, aux2: u8) -> ArtSync {
        let header = ArtHeader::new(opcode::OP_SYNC);

        ArtSync { header, aux1, aux2 }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.header.serialize().iter().cloned());
        out.push(self.aux1);
        out.push(self.aux2);

        out
    }

    pub fn deserialize(data: &[u8]) -> ArtSync {
        let aux1 = data[0];
        let aux2 = data[1];

        ArtSync::new(aux1, aux2)
    }
}
