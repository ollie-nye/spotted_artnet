use super::header;

pub struct ArtPoll {
  header: header::art_header::ArtHeader,
  talk_to_me: u8,
  priority: u8
}

impl ArtPoll {
  pub fn new(talk_to_me: u8, priority: u8) -> ArtPoll {
    let header = header::art_header::ArtHeader::new(header::opcode::OP_POLL);

    let art_poll = ArtPoll { header, talk_to_me, priority };

    art_poll
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend(self.header.serialize().iter().cloned());
    out.push(self.talk_to_me);
    out.push(self.priority);

    out
  }
}
