package java.io;
public final class FileDescriptor { public static final FileDescriptor in=new FileDescriptor(),out=new FileDescriptor(),err=new FileDescriptor(); public boolean valid(){return false;} public void sync() throws IOException {throw new IOException("File access unavailable");} }
