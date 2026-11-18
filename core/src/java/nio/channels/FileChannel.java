package java.nio.channels;
import java.io.Closeable;
import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.MappedByteBuffer;
public abstract class FileChannel implements Closeable { public static class MapMode { public static final MapMode READ_ONLY=new MapMode(),READ_WRITE=new MapMode(),PRIVATE=new MapMode(); private MapMode(){} } public abstract int read(ByteBuffer dst)throws IOException; public abstract int write(ByteBuffer src)throws IOException; public abstract long position()throws IOException; public abstract FileChannel position(long p)throws IOException; public abstract long size()throws IOException; public abstract MappedByteBuffer map(MapMode mode,long position,long size)throws IOException; public FileLock tryLock()throws IOException{return null;} public abstract FileLock lock()throws IOException; }
