package java.lang;
public final class Float extends Number implements Comparable<Float> {
    public static final float POSITIVE_INFINITY = 1.0f / 0.0f;
    public static final float NEGATIVE_INFINITY = -1.0f / 0.0f;
    public static final float NaN = 0.0f / 0.0f;
    public static final float MAX_VALUE = 0x1.fffffeP+127f;
    public static final float MIN_VALUE = 0x0.000002P-126f;
    public static final float MIN_NORMAL = 0x1.0p-126f;
    public static final int SIZE = 32;
    public static final int BYTES = 4;
    @SuppressWarnings("unchecked")
    public static final Class<Float> TYPE = (Class<Float>) VMPrimitive.get('F');
    private final float value;
    public Float(float v) { value = v; }
    public Float(double v) { value = (float) v; }
    public static Float valueOf(float f) { return new Float(f); }
    public static Float valueOf(String s) { return new Float(parseFloat(s)); }
    public static float parseFloat(String s) { return (float) Double.parseDouble(s); }
    public static native String toString(float f);
    public static boolean isNaN(float f) { return f != f; }
    public static boolean isInfinite(float f) { return f == POSITIVE_INFINITY || f == NEGATIVE_INFINITY; }
    public static boolean isFinite(float f) { return Math.abs(f) <= MAX_VALUE; }
    public static native int floatToRawIntBits(float f);
    public static int floatToIntBits(float f) { return isNaN(f) ? 0x7fc00000 : floatToRawIntBits(f); }
    public static native float intBitsToFloat(int i);
    public static int compare(float a, float b) {
        if (a < b) return -1;
        if (a > b) return 1;
        int x = floatToIntBits(a), y = floatToIntBits(b);
        return x == y ? 0 : (x < y ? -1 : 1);
    }
    public static int hashCode(float f) { return floatToIntBits(f); }
    public static float sum(float a, float b) { return a + b; }
    public static float max(float a, float b) { return Math.max(a, b); }
    public static float min(float a, float b) { return Math.min(a, b); }
    public boolean isNaN() { return isNaN(value); }
    public boolean isInfinite() { return isInfinite(value); }
    public int intValue() { return (int) value; }
    public long longValue() { return (long) value; }
    public float floatValue() { return value; }
    public double doubleValue() { return value; }
    public int hashCode() { return floatToIntBits(value); }
    public boolean equals(Object o) { return o instanceof Float && floatToIntBits(((Float) o).value) == floatToIntBits(value); }
    public int compareTo(Float o) { return compare(value, o.value); }
    public String toString() { return toString(value); }
}
