package java.io;
/** Path-only File (no filesystem access yet). */
public class File implements Serializable, Comparable<File> {
    public static final String separator = "/";
    public static final char separatorChar = '/';
    public static final String pathSeparator = ":";
    private final String path;
    public File(String p) { path = p; }
    public File(String parent, String child) { path = parent + "/" + child; }
    public File(File parent, String child) { this(parent.getPath(), child); }
    public String getPath() { return path; }
    public String getAbsolutePath() { return path; }
    public File getAbsoluteFile() { return this; }
    public String getName() { int i = path.lastIndexOf('/'); return path.substring(i + 1); }
    public String getParent() { int i = path.lastIndexOf('/'); return i <= 0 ? null : path.substring(0, i); }
    public File getParentFile() { String p = getParent(); return p == null ? null : new File(p); }
    public boolean canRead() { return false; } public boolean canWrite() { return false; } public boolean createNewFile() throws IOException { throw new IOException("File access unavailable"); }
    public boolean exists() { return false; }
    public boolean isDirectory() { return false; }
    public boolean isFile() { return false; }
    public boolean mkdirs() { return true; }
    public boolean mkdir() { return true; }
    public boolean delete() { return false; }
    public long length() { return 0; }
    public String[] list() { return new String[0]; }
    public File[] listFiles() { return new File[0]; }
    public int compareTo(File o) { return path.compareTo(o.path); }
    public boolean equals(Object o) { return o instanceof File && ((File) o).path.equals(path); }
    public int hashCode() { return path.hashCode(); }
    public String toString() { return path; }
}
