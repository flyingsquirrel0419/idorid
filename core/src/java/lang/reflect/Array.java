package java.lang.reflect;

public final class Array {
    private Array() {}
    public static native Object newInstance(Class<?> componentType, int length);
    public static Object newInstance(Class<?> componentType, int... dims) {
        if (dims.length == 0) throw new IllegalArgumentException("Empty dimensions array");
        return multi(componentType, dims, 0);
    }
    private static Object multi(Class<?> c, int[] dims, int i) {
        if (i == dims.length - 1) return newInstance(c, dims[i]);
        Class<?> sub = arrayClassOf(c, dims.length - i - 1);
        Object[] a = (Object[]) newInstance(sub, dims[i]);
        for (int k = 0; k < a.length; k++) a[k] = multi(c, dims, i + 1);
        return a;
    }
    /** Class for c[]...[] with `depth` dimensions. */
    private static native Class<?> arrayClassOf(Class<?> c, int depth);
    public static native int getLength(Object array);
    public static native Object get(Object array, int index);
    public static native void set(Object array, int index, Object value);
    public static int getInt(Object a, int i) { return ((Number) get(a, i)).intValue(); }
    public static long getLong(Object a, int i) { return ((Number) get(a, i)).longValue(); }
    public static double getDouble(Object a, int i) { return ((Number) get(a, i)).doubleValue(); }
    public static boolean getBoolean(Object a, int i) { return (Boolean) get(a, i); }
    public static void setInt(Object a, int i, int v) { set(a, i, Integer.valueOf(v)); }
}
