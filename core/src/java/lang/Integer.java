package java.lang;

public final class Integer extends Number implements Comparable<Integer> {
    public static final int MIN_VALUE = 0x80000000;
    public static final int MAX_VALUE = 0x7fffffff;
    public static final int SIZE = 32;
    public static final int BYTES = 4;
    @SuppressWarnings("unchecked")
    public static final Class<Integer> TYPE = (Class<Integer>) VMPrimitive.get('I');
    private static final Integer[] CACHE = new Integer[256];
    static final char[] DIGITS = "0123456789abcdefghijklmnopqrstuvwxyz".toCharArray();

    private final int value;
    public Integer(int v) { value = v; }
    public Integer(String s) { value = parseInt(s); }

    public static Integer valueOf(int i) {
        if (i >= -128 && i <= 127) {
            Integer c = CACHE[i + 128];
            if (c == null) { c = new Integer(i); CACHE[i + 128] = c; }
            return c;
        }
        return new Integer(i);
    }
    public static Integer valueOf(String s) { return valueOf(parseInt(s)); }
    public static Integer valueOf(String s, int radix) { return valueOf(parseInt(s, radix)); }
    public static int parseInt(String s) { return parseInt(s, 10); }
    public static int parseInt(String s, int radix) {
        if (s == null) throw new NumberFormatException("null");
        long r = Long.parseLongImpl(s, radix, MIN_VALUE, MAX_VALUE);
        return (int) r;
    }
    public static int parseUnsignedInt(String s) { return (int) Long.parseLong(s); }
    public static int parseUnsignedInt(String s, int radix) { return (int) Long.parseLong(s, radix); }
    public static Integer decode(String s) { return valueOf((int) Long.decode(s).longValue()); }
    public static Integer getInteger(String n) { return null; }

    public static String toString(int i) {
        if (i == MIN_VALUE) return "-2147483648";
        char[] buf = new char[11];
        int p = 11;
        boolean neg = i < 0;
        if (neg) i = -i;
        do { buf[--p] = (char) ('0' + i % 10); i /= 10; } while (i != 0);
        if (neg) buf[--p] = '-';
        return new String(buf, p, 11 - p);
    }
    public static String toString(int i, int radix) { return Long.toString(i, radix); }
    static String toUnsigned(int i, int shift) {
        char[] buf = new char[32];
        int p = 32, mask = (1 << shift) - 1;
        do { buf[--p] = DIGITS[i & mask]; i >>>= shift; } while (i != 0);
        return new String(buf, p, 32 - p);
    }
    public static String toHexString(int i) { return toUnsigned(i, 4); }
    public static String toOctalString(int i) { return toUnsigned(i, 3); }
    public static String toBinaryString(int i) { return toUnsigned(i, 1); }
    public static String toUnsignedString(int i) { return Long.toString(i & 0xffffffffL); }
    public static long toUnsignedLong(int i) { return i & 0xffffffffL; }

    public static int compare(int a, int b) { return a < b ? -1 : (a == b ? 0 : 1); }
    public static int compareUnsigned(int a, int b) { return compare(a + MIN_VALUE, b + MIN_VALUE); }
    public static int signum(int i) { return (i >> 31) | (-i >>> 31); }
    public static int hashCode(int i) { return i; }
    public static int sum(int a, int b) { return a + b; }
    public static int max(int a, int b) { return Math.max(a, b); }
    public static int min(int a, int b) { return Math.min(a, b); }
    public static int bitCount(int i) {
        i = i - ((i >>> 1) & 0x55555555);
        i = (i & 0x33333333) + ((i >>> 2) & 0x33333333);
        i = (i + (i >>> 4)) & 0x0f0f0f0f;
        i = i + (i >>> 8);
        i = i + (i >>> 16);
        return i & 0x3f;
    }
    public static int highestOneBit(int i) { return i & (MIN_VALUE >>> numberOfLeadingZeros(i)); }
    public static int lowestOneBit(int i) { return i & -i; }
    public static int numberOfLeadingZeros(int i) {
        if (i == 0) return 32;
        int n = 1;
        if (i >>> 16 == 0) { n += 16; i <<= 16; }
        if (i >>> 24 == 0) { n += 8; i <<= 8; }
        if (i >>> 28 == 0) { n += 4; i <<= 4; }
        if (i >>> 30 == 0) { n += 2; i <<= 2; }
        n -= i >>> 31;
        return n;
    }
    public static int numberOfTrailingZeros(int i) {
        if (i == 0) return 32;
        int n = 0;
        while ((i & 1) == 0) { i >>>= 1; n++; }
        return n;
    }
    public static int rotateLeft(int i, int d) { return (i << d) | (i >>> -d); }
    public static int rotateRight(int i, int d) { return (i >>> d) | (i << -d); }
    public static int reverse(int i) {
        i = (i & 0x55555555) << 1 | (i >>> 1) & 0x55555555;
        i = (i & 0x33333333) << 2 | (i >>> 2) & 0x33333333;
        i = (i & 0x0f0f0f0f) << 4 | (i >>> 4) & 0x0f0f0f0f;
        return reverseBytes(i);
    }
    public static int reverseBytes(int i) { return (i << 24) | ((i & 0xff00) << 8) | ((i >>> 8) & 0xff00) | (i >>> 24); }

    public int intValue() { return value; }
    public long longValue() { return value; }
    public float floatValue() { return value; }
    public double doubleValue() { return value; }
    public int hashCode() { return value; }
    public boolean equals(Object o) { return o instanceof Integer && ((Integer) o).value == value; }
    public int compareTo(Integer o) { return compare(value, o.value); }
    public String toString() { return toString(value); }
}
