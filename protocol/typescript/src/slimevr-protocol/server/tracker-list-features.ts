// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { DeviceStatusFeatures, DeviceStatusFeaturesT } from '../../slimevr-protocol/server/device-status-features';


export class TrackerListFeatures {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
__init(i:number, bb:flatbuffers.ByteBuffer):TrackerListFeatures {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsTrackerListFeatures(bb:flatbuffers.ByteBuffer, obj?:TrackerListFeatures):TrackerListFeatures {
  return (obj || new TrackerListFeatures()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsTrackerListFeatures(bb:flatbuffers.ByteBuffer, obj?:TrackerListFeatures):TrackerListFeatures {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new TrackerListFeatures()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

interval():number {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint16(this.bb_pos + offset) : 0;
}

complete():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

deviceFeatures(obj?:DeviceStatusFeatures):DeviceStatusFeatures|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? (obj || new DeviceStatusFeatures()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

static startTrackerListFeatures(builder:flatbuffers.Builder) {
  builder.startObject(3);
}

static addInterval(builder:flatbuffers.Builder, interval:number) {
  builder.addFieldInt16(0, interval, 0);
}

static addComplete(builder:flatbuffers.Builder, complete:boolean) {
  builder.addFieldInt8(1, +complete, +false);
}

static addDeviceFeatures(builder:flatbuffers.Builder, deviceFeaturesOffset:flatbuffers.Offset) {
  builder.addFieldOffset(2, deviceFeaturesOffset, 0);
}

static endTrackerListFeatures(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}


unpack(): TrackerListFeaturesT {
  return new TrackerListFeaturesT(
    this.interval(),
    this.complete(),
    (this.deviceFeatures() !== null ? this.deviceFeatures()!.unpack() : null)
  );
}


unpackTo(_o: TrackerListFeaturesT): void {
  _o.interval = this.interval();
  _o.complete = this.complete();
  _o.deviceFeatures = (this.deviceFeatures() !== null ? this.deviceFeatures()!.unpack() : null);
}
}

export class TrackerListFeaturesT {
constructor(
  public interval: number = 0,
  public complete: boolean = false,
  public deviceFeatures: DeviceStatusFeaturesT|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const deviceFeatures = (this.deviceFeatures !== null ? this.deviceFeatures!.pack(builder) : 0);

  TrackerListFeatures.startTrackerListFeatures(builder);
  TrackerListFeatures.addInterval(builder, this.interval);
  TrackerListFeatures.addComplete(builder, this.complete);
  TrackerListFeatures.addDeviceFeatures(builder, deviceFeatures);

  return TrackerListFeatures.endTrackerListFeatures(builder);
}
}
