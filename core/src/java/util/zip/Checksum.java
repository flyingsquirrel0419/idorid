package java.util.zip;
public interface Checksum { void update(int b); void update(byte[] b,int offset,int length); long getValue(); void reset(); }
