// automatically generated by the FlatBuffers compiler, do not modify

package slimevr_protocol.rpc;

import com.google.flatbuffers.FlatBufferBuilder;

public class RpcMessageUnion {
  private byte type;
  private Object value;

  public byte getType() { return type; }

  public void setType(byte type) { this.type = type; }

  public Object getValue() { return value; }

  public void setValue(Object value) { this.value = value; }

  public RpcMessageUnion() {
    this.type = RpcMessage.NONE;
    this.value = null;
  }

  public slimevr_protocol.rpc.HeartbeatRequestT asHeartbeatRequest() { return (slimevr_protocol.rpc.HeartbeatRequestT) value; }
  public slimevr_protocol.rpc.HeartbeatResponseT asHeartbeatResponse() { return (slimevr_protocol.rpc.HeartbeatResponseT) value; }
  public slimevr_protocol.rpc.ResetRequestT asResetRequest() { return (slimevr_protocol.rpc.ResetRequestT) value; }
  public slimevr_protocol.rpc.AssignTrackerRequestT asAssignTrackerRequest() { return (slimevr_protocol.rpc.AssignTrackerRequestT) value; }
  public slimevr_protocol.rpc.SettingsRequestT asSettingsRequest() { return (slimevr_protocol.rpc.SettingsRequestT) value; }
  public slimevr_protocol.rpc.SettingsResponseT asSettingsResponse() { return (slimevr_protocol.rpc.SettingsResponseT) value; }
  public slimevr_protocol.rpc.ChangeSettingsRequestT asChangeSettingsRequest() { return (slimevr_protocol.rpc.ChangeSettingsRequestT) value; }
  public slimevr_protocol.rpc.RecordBVHRequestT asRecordBVHRequest() { return (slimevr_protocol.rpc.RecordBVHRequestT) value; }
  public slimevr_protocol.rpc.RecordBVHStatusT asRecordBVHStatus() { return (slimevr_protocol.rpc.RecordBVHStatusT) value; }
  public slimevr_protocol.rpc.SkeletonConfigRequestT asSkeletonConfigRequest() { return (slimevr_protocol.rpc.SkeletonConfigRequestT) value; }
  public slimevr_protocol.rpc.ChangeSkeletonConfigRequestT asChangeSkeletonConfigRequest() { return (slimevr_protocol.rpc.ChangeSkeletonConfigRequestT) value; }
  public slimevr_protocol.rpc.SkeletonResetAllRequestT asSkeletonResetAllRequest() { return (slimevr_protocol.rpc.SkeletonResetAllRequestT) value; }
  public slimevr_protocol.rpc.SkeletonConfigResponseT asSkeletonConfigResponse() { return (slimevr_protocol.rpc.SkeletonConfigResponseT) value; }
  public slimevr_protocol.rpc.OpenSerialRequestT asOpenSerialRequest() { return (slimevr_protocol.rpc.OpenSerialRequestT) value; }
  public slimevr_protocol.rpc.CloseSerialRequestT asCloseSerialRequest() { return (slimevr_protocol.rpc.CloseSerialRequestT) value; }
  public slimevr_protocol.rpc.SetWifiRequestT asSetWifiRequest() { return (slimevr_protocol.rpc.SetWifiRequestT) value; }
  public slimevr_protocol.rpc.SerialUpdateResponseT asSerialUpdateResponse() { return (slimevr_protocol.rpc.SerialUpdateResponseT) value; }

  public static int pack(FlatBufferBuilder builder, RpcMessageUnion _o) {
    switch (_o.type) {
      case RpcMessage.HeartbeatRequest: return slimevr_protocol.rpc.HeartbeatRequest.pack(builder, _o.asHeartbeatRequest());
      case RpcMessage.HeartbeatResponse: return slimevr_protocol.rpc.HeartbeatResponse.pack(builder, _o.asHeartbeatResponse());
      case RpcMessage.ResetRequest: return slimevr_protocol.rpc.ResetRequest.pack(builder, _o.asResetRequest());
      case RpcMessage.AssignTrackerRequest: return slimevr_protocol.rpc.AssignTrackerRequest.pack(builder, _o.asAssignTrackerRequest());
      case RpcMessage.SettingsRequest: return slimevr_protocol.rpc.SettingsRequest.pack(builder, _o.asSettingsRequest());
      case RpcMessage.SettingsResponse: return slimevr_protocol.rpc.SettingsResponse.pack(builder, _o.asSettingsResponse());
      case RpcMessage.ChangeSettingsRequest: return slimevr_protocol.rpc.ChangeSettingsRequest.pack(builder, _o.asChangeSettingsRequest());
      case RpcMessage.RecordBVHRequest: return slimevr_protocol.rpc.RecordBVHRequest.pack(builder, _o.asRecordBVHRequest());
      case RpcMessage.RecordBVHStatus: return slimevr_protocol.rpc.RecordBVHStatus.pack(builder, _o.asRecordBVHStatus());
      case RpcMessage.SkeletonConfigRequest: return slimevr_protocol.rpc.SkeletonConfigRequest.pack(builder, _o.asSkeletonConfigRequest());
      case RpcMessage.ChangeSkeletonConfigRequest: return slimevr_protocol.rpc.ChangeSkeletonConfigRequest.pack(builder, _o.asChangeSkeletonConfigRequest());
      case RpcMessage.SkeletonResetAllRequest: return slimevr_protocol.rpc.SkeletonResetAllRequest.pack(builder, _o.asSkeletonResetAllRequest());
      case RpcMessage.SkeletonConfigResponse: return slimevr_protocol.rpc.SkeletonConfigResponse.pack(builder, _o.asSkeletonConfigResponse());
      case RpcMessage.OpenSerialRequest: return slimevr_protocol.rpc.OpenSerialRequest.pack(builder, _o.asOpenSerialRequest());
      case RpcMessage.CloseSerialRequest: return slimevr_protocol.rpc.CloseSerialRequest.pack(builder, _o.asCloseSerialRequest());
      case RpcMessage.SetWifiRequest: return slimevr_protocol.rpc.SetWifiRequest.pack(builder, _o.asSetWifiRequest());
      case RpcMessage.SerialUpdateResponse: return slimevr_protocol.rpc.SerialUpdateResponse.pack(builder, _o.asSerialUpdateResponse());
      default: return 0;
    }
  }
}

