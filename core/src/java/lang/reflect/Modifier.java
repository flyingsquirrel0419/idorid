package java.lang.reflect;

public class Modifier {
    public static final int PUBLIC = 1, PRIVATE = 2, PROTECTED = 4, STATIC = 8, FINAL = 16, SYNCHRONIZED = 32, VOLATILE = 64, TRANSIENT = 128,
        NATIVE = 256, INTERFACE = 512, ABSTRACT = 1024, STRICT = 2048;
    public static boolean isPublic(int m) { return (m & PUBLIC) != 0; }
    public static boolean isPrivate(int m) { return (m & PRIVATE) != 0; }
    public static boolean isProtected(int m) { return (m & PROTECTED) != 0; }
    public static boolean isStatic(int m) { return (m & STATIC) != 0; }
    public static boolean isFinal(int m) { return (m & FINAL) != 0; }
    public static boolean isSynchronized(int m) { return (m & SYNCHRONIZED) != 0; }
    public static boolean isVolatile(int m) { return (m & VOLATILE) != 0; }
    public static boolean isTransient(int m) { return (m & TRANSIENT) != 0; }
    public static boolean isNative(int m) { return (m & NATIVE) != 0; }
    public static boolean isInterface(int m) { return (m & INTERFACE) != 0; }
    public static boolean isAbstract(int m) { return (m & ABSTRACT) != 0; }
    public static boolean isStrict(int m) { return (m & STRICT) != 0; }
    public static int classModifiers() { return 0xc1f; }
    public static int methodModifiers() { return 0xd3f; }
    public static int fieldModifiers() { return 0xdf; }
    public static String toString(int m) {
        StringBuilder sb = new StringBuilder();
        if ((m & PUBLIC) != 0) sb.append("public ");
        if ((m & PROTECTED) != 0) sb.append("protected ");
        if ((m & PRIVATE) != 0) sb.append("private ");
        if ((m & ABSTRACT) != 0) sb.append("abstract ");
        if ((m & STATIC) != 0) sb.append("static ");
        if ((m & FINAL) != 0) sb.append("final ");
        if ((m & TRANSIENT) != 0) sb.append("transient ");
        if ((m & VOLATILE) != 0) sb.append("volatile ");
        if ((m & SYNCHRONIZED) != 0) sb.append("synchronized ");
        if ((m & NATIVE) != 0) sb.append("native ");
        if ((m & STRICT) != 0) sb.append("strictfp ");
        if ((m & INTERFACE) != 0) sb.append("interface ");
        int n = sb.length();
        return n > 0 ? sb.substring(0, n - 1) : "";
    }
}
