package java.nio.channels;
import java.io.Closeable;
import java.io.IOException;
public abstract class FileLock implements Closeable { private final FileChannel channel; private final long position,size; private final boolean shared; protected FileLock(FileChannel c,long p,long s,boolean shared){channel=c;position=p;size=s;this.shared=shared;} public FileChannel channel(){return channel;} public long position(){return position;} public long size(){return size;} public boolean isShared(){return shared;} public abstract boolean isValid(); public abstract void release()throws IOException; public void close()throws IOException{release();} }
