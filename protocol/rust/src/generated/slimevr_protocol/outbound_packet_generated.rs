// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum OutboundPacketOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct OutboundPacket<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for OutboundPacket<'a> {
  type Inner = OutboundPacket<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> OutboundPacket<'a> {
  pub const VT_PACKET_COUNT: flatbuffers::VOffsetT = 4;
  pub const VT_ACKNOWLEDGE_ME: flatbuffers::VOffsetT = 6;
  pub const VT_PACKET_TYPE: flatbuffers::VOffsetT = 8;
  pub const VT_PACKET: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    OutboundPacket { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args OutboundPacketArgs
  ) -> flatbuffers::WIPOffset<OutboundPacket<'bldr>> {
    let mut builder = OutboundPacketBuilder::new(_fbb);
    if let Some(x) = args.packet { builder.add_packet(x); }
    builder.add_packet_count(args.packet_count);
    builder.add_packet_type(args.packet_type);
    builder.add_acknowledge_me(args.acknowledge_me);
    builder.finish()
  }


  #[inline]
  pub fn packet_count(&self) -> u32 {
    self._tab.get::<u32>(OutboundPacket::VT_PACKET_COUNT, Some(0)).unwrap()
  }
  #[inline]
  pub fn acknowledge_me(&self) -> bool {
    self._tab.get::<bool>(OutboundPacket::VT_ACKNOWLEDGE_ME, Some(false)).unwrap()
  }
  #[inline]
  pub fn packet_type(&self) -> OutboundUnion {
    self._tab.get::<OutboundUnion>(OutboundPacket::VT_PACKET_TYPE, Some(OutboundUnion::NONE)).unwrap()
  }
  #[inline]
  pub fn packet(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(OutboundPacket::VT_PACKET, None)
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn packet_as_slimevr_protocol_rpc_heartbeat_request(&self) -> Option<rpc::HeartbeatRequest<'a>> {
    if self.packet_type() == OutboundUnion::slimevr_protocol_rpc_HeartbeatRequest {
      self.packet().map(rpc::HeartbeatRequest::init_from_table)
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn packet_as_slimevr_protocol_rpc_settings_response(&self) -> Option<rpc::SettingsResponse<'a>> {
    if self.packet_type() == OutboundUnion::slimevr_protocol_rpc_SettingsResponse {
      self.packet().map(rpc::SettingsResponse::init_from_table)
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn packet_as_slimevr_protocol_data_feed_poll_data_feed(&self) -> Option<data_feed::PollDataFeed<'a>> {
    if self.packet_type() == OutboundUnion::slimevr_protocol_data_feed_PollDataFeed {
      self.packet().map(data_feed::PollDataFeed::init_from_table)
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn packet_as_slimevr_protocol_data_feed_data_feed_request(&self) -> Option<data_feed::DataFeedRequest<'a>> {
    if self.packet_type() == OutboundUnion::slimevr_protocol_data_feed_DataFeedRequest {
      self.packet().map(data_feed::DataFeedRequest::init_from_table)
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn packet_as_slimevr_protocol_data_feed_data_feed_update(&self) -> Option<data_feed::DataFeedUpdate<'a>> {
    if self.packet_type() == OutboundUnion::slimevr_protocol_data_feed_DataFeedUpdate {
      self.packet().map(data_feed::DataFeedUpdate::init_from_table)
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for OutboundPacket<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u32>("packet_count", Self::VT_PACKET_COUNT, false)?
     .visit_field::<bool>("acknowledge_me", Self::VT_ACKNOWLEDGE_ME, false)?
     .visit_union::<OutboundUnion, _>("packet_type", Self::VT_PACKET_TYPE, "packet", Self::VT_PACKET, false, |key, v, pos| {
        match key {
          OutboundUnion::slimevr_protocol_rpc_HeartbeatRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<rpc::HeartbeatRequest>>("OutboundUnion::slimevr_protocol_rpc_HeartbeatRequest", pos),
          OutboundUnion::slimevr_protocol_rpc_SettingsResponse => v.verify_union_variant::<flatbuffers::ForwardsUOffset<rpc::SettingsResponse>>("OutboundUnion::slimevr_protocol_rpc_SettingsResponse", pos),
          OutboundUnion::slimevr_protocol_data_feed_PollDataFeed => v.verify_union_variant::<flatbuffers::ForwardsUOffset<data_feed::PollDataFeed>>("OutboundUnion::slimevr_protocol_data_feed_PollDataFeed", pos),
          OutboundUnion::slimevr_protocol_data_feed_DataFeedRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<data_feed::DataFeedRequest>>("OutboundUnion::slimevr_protocol_data_feed_DataFeedRequest", pos),
          OutboundUnion::slimevr_protocol_data_feed_DataFeedUpdate => v.verify_union_variant::<flatbuffers::ForwardsUOffset<data_feed::DataFeedUpdate>>("OutboundUnion::slimevr_protocol_data_feed_DataFeedUpdate", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct OutboundPacketArgs {
    pub packet_count: u32,
    pub acknowledge_me: bool,
    pub packet_type: OutboundUnion,
    pub packet: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for OutboundPacketArgs {
  #[inline]
  fn default() -> Self {
    OutboundPacketArgs {
      packet_count: 0,
      acknowledge_me: false,
      packet_type: OutboundUnion::NONE,
      packet: None,
    }
  }
}

pub struct OutboundPacketBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> OutboundPacketBuilder<'a, 'b> {
  #[inline]
  pub fn add_packet_count(&mut self, packet_count: u32) {
    self.fbb_.push_slot::<u32>(OutboundPacket::VT_PACKET_COUNT, packet_count, 0);
  }
  #[inline]
  pub fn add_acknowledge_me(&mut self, acknowledge_me: bool) {
    self.fbb_.push_slot::<bool>(OutboundPacket::VT_ACKNOWLEDGE_ME, acknowledge_me, false);
  }
  #[inline]
  pub fn add_packet_type(&mut self, packet_type: OutboundUnion) {
    self.fbb_.push_slot::<OutboundUnion>(OutboundPacket::VT_PACKET_TYPE, packet_type, OutboundUnion::NONE);
  }
  #[inline]
  pub fn add_packet(&mut self, packet: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(OutboundPacket::VT_PACKET, packet);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> OutboundPacketBuilder<'a, 'b> {
    let start = _fbb.start_table();
    OutboundPacketBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<OutboundPacket<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for OutboundPacket<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("OutboundPacket");
      ds.field("packet_count", &self.packet_count());
      ds.field("acknowledge_me", &self.acknowledge_me());
      ds.field("packet_type", &self.packet_type());
      match self.packet_type() {
        OutboundUnion::slimevr_protocol_rpc_HeartbeatRequest => {
          if let Some(x) = self.packet_as_slimevr_protocol_rpc_heartbeat_request() {
            ds.field("packet", &x)
          } else {
            ds.field("packet", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        OutboundUnion::slimevr_protocol_rpc_SettingsResponse => {
          if let Some(x) = self.packet_as_slimevr_protocol_rpc_settings_response() {
            ds.field("packet", &x)
          } else {
            ds.field("packet", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        OutboundUnion::slimevr_protocol_data_feed_PollDataFeed => {
          if let Some(x) = self.packet_as_slimevr_protocol_data_feed_poll_data_feed() {
            ds.field("packet", &x)
          } else {
            ds.field("packet", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        OutboundUnion::slimevr_protocol_data_feed_DataFeedRequest => {
          if let Some(x) = self.packet_as_slimevr_protocol_data_feed_data_feed_request() {
            ds.field("packet", &x)
          } else {
            ds.field("packet", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        OutboundUnion::slimevr_protocol_data_feed_DataFeedUpdate => {
          if let Some(x) = self.packet_as_slimevr_protocol_data_feed_data_feed_update() {
            ds.field("packet", &x)
          } else {
            ds.field("packet", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("packet", &x)
        },
      };
      ds.finish()
  }
}
