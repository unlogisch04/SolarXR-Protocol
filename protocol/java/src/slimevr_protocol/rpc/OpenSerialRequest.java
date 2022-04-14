// automatically generated by the FlatBuffers compiler, do not modify

package slimevr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class OpenSerialRequest extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_2_0_0(); }
  public static OpenSerialRequest getRootAsOpenSerialRequest(ByteBuffer _bb) { return getRootAsOpenSerialRequest(_bb, new OpenSerialRequest()); }
  public static OpenSerialRequest getRootAsOpenSerialRequest(ByteBuffer _bb, OpenSerialRequest obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public OpenSerialRequest __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }


  public static void startOpenSerialRequest(FlatBufferBuilder builder) { builder.startTable(0); }
  public static int endOpenSerialRequest(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public OpenSerialRequest get(int j) { return get(new OpenSerialRequest(), j); }
    public OpenSerialRequest get(OpenSerialRequest obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public OpenSerialRequestT unpack() {
    OpenSerialRequestT _o = new OpenSerialRequestT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(OpenSerialRequestT _o) {
  }
  public static int pack(FlatBufferBuilder builder, OpenSerialRequestT _o) {
    if (_o == null) return 0;
    startOpenSerialRequest(builder);
    return endOpenSerialRequest(builder);
  }
}

