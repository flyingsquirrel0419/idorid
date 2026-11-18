package java.io;
public class FileInputStream extends InputStream {
    public FileInputStream(String path) throws FileNotFoundException { throw new FileNotFoundException(path); }
    public FileInputStream(File file) throws FileNotFoundException { throw new FileNotFoundException(file.toString()); }
    public int read() throws IOException { return -1; }
    public void close() throws IOException {}
}
