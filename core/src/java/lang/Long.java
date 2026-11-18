package java.lang;

public final class Long extends Number implements Comparable<Long> {
    public static final long MIN_VALUE = 0x8000000000000000L;
    public static final long MAX_VALUE = 0x7fffffffffffffffL;
    public static final int SIZE = 64;
    public static final int BYTES = 8;
    @SuppressWarnings("unchecked")
    public static final Class<Long> TYPE = (Class<Long>) VMPrimitive.get('J');
    private static final Long[] CACHE = new Long[256];

    private final long value;
    public Long(long v) { value = v; }
    public Long(String s) { value = parseLong(s); }

    public static Long valueOf(long l) {
        if (l >= -128 && l <= 127) {
            int i = (int) l + 128;
            Long c = CACHE[i];
            if (c == null) { c = new Long(l); CACHE[i] = c; }
            return c;
        }
        return new Long(l);
    }
    public static Long valueOf(String s) { return valueOf(parseLong(s)); }
    public static Long valueOf(String s, int radix) { return valueOf(parseLong(s, radix)); }
    public static long parseLong(String s) { return parseLong(s, 10); }
    public static long parseLong(String s, int radix) {
        if (s == null) throw new NumberFormatException("null");
        return parseLongImpl(s, radix, MIN_VALUE, MAX_VALUE);
    }
    static long parseLongImpl(String s, int radix, long min, long max) {
        int n = s.length();
        if (n == 0) throw new NumberFormatException("For input string: \"\"");
        int i = 0;
        boolean neg = false;
        char c0 = s.charAt(0);
        if (c0 == '-' || c0 == '+') { neg = c0 == '-'; i = 1; if (n == 1) throw nfe(s); }
        long limit = neg ? min : -max;
        long r = 0;
        long multmin = limit / radix;
        for (; i < n; i++) {
            int d = Character.digit(s.charAt(i), radix);
            if (d < 0 || r < multmin) throw nfe(s);
            r *= radix;
            if (r < limit + d) throw nfe(s);
            r -= d;
        }
        return neg ? r : -r;
    }
    private static NumberFormatException nfe(String s) { return new NumberFormatException("For input string: \"" + s + "\""); }
    public static Long decode(String s) {
        int radix = 10, i = 0;
        boolean neg = false;
        if (s.startsWith("-")) { neg = true; i = 1; } else if (s.startsWith("+")) i = 1;
        if (s.startsWith("0x", i) || s.startsWith("0X", i)) { radix = 16; i += 2; }
        else if (s.startsWith("#", i)) { radix = 16; i++; }
        else if (s.startsWith("0", i) && s.length() > i + 1) { radix = 8; i++; }
        long v = parseLong(s.substring(i), radix);
        return valueOf(neg ? -v : v);
    }

    public static String toString(long l) { return toString(l, 10); }
    public static String toString(long l, int radix) {
        if (radix < 2 || radix > 36) radix = 10;
        if (l == 0) return "0";
        char[] buf = new char[65];
        int p = 65;
        boolean neg = l < 0;
        if (!neg) l = -l;
        while (l != 0) { buf[--p] = Integer.DIGITS[(int) -(l % radix)]; l /= radix; }
        if (neg) buf[--p] = '-';
        return new String(buf, p, 65 - p);
    }
    static String toUnsigned(long i, int shift) {
        char[] buf = new char[64];
        int p = 64;
        long mask = (1L << shift) - 1;
        do { buf[--p] = Integer.DIGITS[(int) (i & mask)]; i >>>= shift; } while (i != 0);
        return new String(buf, p, 64 - p);
    }
    public static String toHexString(long i) { return toUnsigned(i, 4); }
    public static String toOctalString(long i) { return toUnsigned(i, 3); }
    public static String toBinaryString(long i) { return toUnsigned(i, 1); }

    public static int compare(long a, long b) { return a < b ? -1 : (a == b ? 0 : 1); }
    public static int signum(long i) { return (int) ((i >> 63) | (-i >>> 63)); }
    public static int hashCode(long l) { return (int) (l ^ (l >>> 32)); }
    public static long sum(long a, long b) { return a + b; }
    public static long max(long a, long b) { return Math.max(a, b); }
    public static long min(long a, long b) { return Math.min(a, b); }
    public static int bitCount(long i) { return Integer.bitCount((int) i) + Integer.bitCount((int) (i >>> 32)); }
    public static int numberOfLeadingZeros(long i) { int h = (int) (i >>> 32); return h == 0 ? 32 + Integer.numberOfLeadingZeros((int) i) : Integer.numberOfLeadingZeros(h); }
    public static int numberOfTrailingZeros(long i) { int l = (int) i; return l == 0 ? 32 + Integer.numberOfTrailingZeros((int) (i >>> 32)) : Integer.numberOfTrailingZeros(l); }
    public static long highestOneBit(long i) { return i & (MIN_VALUE >>> numberOfLeadingZeros(i)); }
    public static long rotateLeft(long i, int d) { return (i << d) | (i >>> -d); }
    public static long rotateRight(long i, int d) { return (i >>> d) | (i << -d); }
    public static long reverseBytes(long i) {
        return ((long) Integer.reverseBytes((int) i) << 32) | (Integer.reverseBytes((int) (i >>> 32)) & 0xffffffffL);
    }

    public int intValue() { return (int) value; }
    public long longValue() { return value; }
    public float floatValue() { return value; }
    public double doubleValue() { return value; }
    public int hashCode() { return hashCode(value); }
    public boolean equals(Object o) { return o instanceof Long && ((Long) o).value == value; }
    public int compareTo(Long o) { return compare(value, o.value); }
    public String toString() { return toString(value); }
}
