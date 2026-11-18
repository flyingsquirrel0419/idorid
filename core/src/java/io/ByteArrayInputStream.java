package java.io;
public class ByteArrayInputStream extends InputStream {
    protected byte[] buf;
    protected int pos, count;
    public ByteArrayInputStream(byte[] b) { buf = b; count = b.length; }
    public ByteArrayInputStream(byte[] b, int off, int len) { buf = b; pos = off; count = Math.min(off + len, b.length); }
    public int read() { return pos < count ? buf[pos++] & 0xff : -1; }
    public int available() { return count - pos; }
}
