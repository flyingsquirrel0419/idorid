package java.lang;
/** idorid internal: primitive class mirrors. */
final class VMPrimitive {
    private VMPrimitive() {}
    static native Class<?> get(char descriptor);
}
