package java.lang;
public final class Short extends Number implements Comparable<Short> {
    public static final short MIN_VALUE = -32768;
    public static final short MAX_VALUE = 32767;
    public static final int SIZE = 16;
    public static final int BYTES = 2;
    @SuppressWarnings("unchecked")
    public static final Class<Short> TYPE = (Class<Short>) VMPrimitive.get('S');
    private final short value;
    public Short(short v) { value = v; }
    public static Short valueOf(short s) { return new Short(s); }
    public static Short valueOf(String s) { return new Short(parseShort(s)); }
    public static short parseShort(String s) { return parseShort(s, 10); }
    public static short parseShort(String s, int r) {
        int i = Integer.parseInt(s, r);
        if (i < MIN_VALUE || i > MAX_VALUE) throw new NumberFormatException("Value out of range. Value:\"" + s + "\" Radix:" + r);
        return (short) i;
    }
    public static String toString(short s) { return Integer.toString(s); }
    public static int compare(short a, short b) { return a - b; }
    public static int hashCode(short s) { return s; }
    public static int toUnsignedInt(short s) { return s & 0xffff; }
    public static short reverseBytes(short i) { return (short) (((i & 0xff00) >> 8) | (i << 8)); }
    public int intValue() { return value; }
    public long longValue() { return value; }
    public float floatValue() { return value; }
    public double doubleValue() { return value; }
    public short shortValue() { return value; }
    public int hashCode() { return value; }
    public boolean equals(Object o) { return o instanceof Short && ((Short) o).value == value; }
    public int compareTo(Short o) { return value - o.value; }
    public String toString() { return Integer.toString(value); }
}
