package android.content.res;

import java.io.IOException;
import java.io.InputStream;

public final class AssetManager implements AutoCloseable {
    public static final int ACCESS_UNKNOWN = 0, ACCESS_RANDOM = 1, ACCESS_STREAMING = 2, ACCESS_BUFFER = 3;
    public AssetManager() {}
    public InputStream open(String name) throws IOException { throw new java.io.FileNotFoundException(name); }
    public InputStream open(String name, int mode) throws IOException { return open(name); }
    public String[] list(String path) throws IOException { return new String[0]; }
    public AssetFileDescriptor openFd(String name) throws IOException { throw new java.io.FileNotFoundException(name); }
    public XmlResourceParser openXmlResourceParser(String name) throws IOException { throw new java.io.FileNotFoundException(name); }
    public String[] getLocales() { return new String[] { "en-US" }; }
    public void close() {}
}
