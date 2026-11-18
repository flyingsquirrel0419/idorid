package java.nio.charset;
public final class StandardCharsets {
    private StandardCharsets() {}
    private static final class Impl extends Charset { Impl(String n) { super(n); } }
    public static final Charset UTF_8 = new Impl("UTF-8");
    public static final Charset US_ASCII = new Impl("US-ASCII");
    public static final Charset ISO_8859_1 = new Impl("ISO-8859-1");
    public static final Charset UTF_16 = new Impl("UTF-16");
}
