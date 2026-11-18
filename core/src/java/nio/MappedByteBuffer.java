package java.nio;
public abstract class MappedByteBuffer extends ByteBuffer { protected MappedByteBuffer(byte[] data,int offset,int capacity){super(data,offset,capacity);} public boolean isLoaded(){return true;} public MappedByteBuffer load(){return this;} public MappedByteBuffer force(){return this;} }
