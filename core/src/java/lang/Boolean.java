package java.lang;
public final class Boolean implements java.io.Serializable, Comparable<Boolean> {
    public static final Boolean TRUE = new Boolean(true);
    public static final Boolean FALSE = new Boolean(false);
    @SuppressWarnings("unchecked")
    public static final Class<Boolean> TYPE = (Class<Boolean>) VMPrimitive.get('Z');
    private final boolean value;
    public Boolean(boolean v) { value = v; }
    public Boolean(String s) { value = parseBoolean(s); }
    public static Boolean valueOf(boolean b) { return b ? TRUE : FALSE; }
    public static Boolean valueOf(String s) { return parseBoolean(s) ? TRUE : FALSE; }
    public static boolean parseBoolean(String s) { return s != null && s.equalsIgnoreCase("true"); }
    public static String toString(boolean b) { return b ? "true" : "false"; }
    public static int compare(boolean a, boolean b) { return a == b ? 0 : (a ? 1 : -1); }
    public static int hashCode(boolean b) { return b ? 1231 : 1237; }
    public static boolean logicalAnd(boolean a, boolean b) { return a && b; }
    public static boolean logicalOr(boolean a, boolean b) { return a || b; }
    public static boolean logicalXor(boolean a, boolean b) { return a ^ b; }
    public boolean booleanValue() { return value; }
    public int hashCode() { return hashCode(value); }
    public boolean equals(Object o) { return o instanceof Boolean && ((Boolean) o).value == value; }
    public int compareTo(Boolean o) { return compare(value, o.value); }
    public String toString() { return toString(value); }
}
