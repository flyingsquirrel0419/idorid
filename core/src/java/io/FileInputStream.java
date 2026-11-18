package java.io;
public class FileInputStream extends InputStream {
    public FileInputStream(String path) throws FileNotFoundException { throw new FileNotFoundException(path); }
    public FileInputStream(File file) throws FileNotFoundException { throw new FileNotFoundException(file.toString()); }
    public FileInputStream(FileDescriptor fd) {}
    public java.nio.channels.FileChannel getChannel() { return null; }
    public int read() throws IOException { return -1; }
    public void close() throws IOException {}
}
