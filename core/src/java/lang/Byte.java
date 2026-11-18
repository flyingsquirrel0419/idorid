package java.lang;
public final class Byte extends Number implements Comparable<Byte> {
    public static final byte MIN_VALUE = -128;
    public static final byte MAX_VALUE = 127;
    public static final int SIZE = 8;
    public static final int BYTES = 1;
    @SuppressWarnings("unchecked")
    public static final Class<Byte> TYPE = (Class<Byte>) VMPrimitive.get('B');
    private static final Byte[] CACHE = new Byte[256];
    private final byte value;
    public Byte(byte v) { value = v; }
    public static Byte valueOf(byte b) {
        Byte c = CACHE[b + 128];
        if (c == null) { c = new Byte(b); CACHE[b + 128] = c; }
        return c;
    }
    public static Byte valueOf(String s) { return valueOf(parseByte(s)); }
    public static byte parseByte(String s) { return parseByte(s, 10); }
    public static byte parseByte(String s, int r) {
        int i = Integer.parseInt(s, r);
        if (i < MIN_VALUE || i > MAX_VALUE) throw new NumberFormatException("Value out of range. Value:\"" + s + "\" Radix:" + r);
        return (byte) i;
    }
    public static String toString(byte b) { return Integer.toString(b); }
    public static int compare(byte a, byte b) { return a - b; }
    public static int hashCode(byte b) { return b; }
    public static int toUnsignedInt(byte b) { return b & 0xff; }
    public int intValue() { return value; }
    public long longValue() { return value; }
    public float floatValue() { return value; }
    public double doubleValue() { return value; }
    public byte byteValue() { return value; }
    public int hashCode() { return value; }
    public boolean equals(Object o) { return o instanceof Byte && ((Byte) o).value == value; }
    public int compareTo(Byte o) { return value - o.value; }
    public String toString() { return Integer.toString(value); }
}
