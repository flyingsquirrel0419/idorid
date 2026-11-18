package java.nio.charset;
public abstract class Charset implements Comparable<Charset> {
    private final String name;
    protected Charset(String name) { this.name = name; }
    public static Charset forName(String n) { return StandardCharsets.UTF_8; }
    public static Charset defaultCharset() { return StandardCharsets.UTF_8; }
    public final String name() { return name; }
    public String displayName() { return name; }
    public final int compareTo(Charset o) { return name.compareTo(o.name); }
    public String toString() { return name; }
}
