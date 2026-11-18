package java.lang.reflect;

/** Shared state of Method and Constructor; {@code slot} is the VM method index. */
public abstract class Executable extends AccessibleObject implements Member {
    Class<?> declaringClass;
    String name;
    Class<?>[] parameterTypes;
    int modifiers;
    int slot;
    Executable() {}
    public Class<?> getDeclaringClass() { return declaringClass; }
    public String getName() { return name; }
    public int getModifiers() { return modifiers; }
    public Class<?>[] getParameterTypes() { return parameterTypes.clone(); }
    public int getParameterCount() { return parameterTypes.length; }
    public Type[] getGenericParameterTypes() { return getParameterTypes(); }
    public Class<?>[] getExceptionTypes() { return new Class<?>[0]; }
    public boolean isVarArgs() { return (modifiers & 0x80) != 0; }
    public boolean isSynthetic() { return (modifiers & 0x1000) != 0; }
    public java.lang.annotation.Annotation[][] getParameterAnnotations() { return new java.lang.annotation.Annotation[parameterTypes.length][0]; }
    public <T extends java.lang.annotation.Annotation> T getAnnotation(Class<T> type) { return null; }
    public java.lang.annotation.Annotation[] getDeclaredAnnotations() { return new java.lang.annotation.Annotation[0]; }
    public java.lang.annotation.Annotation[] getAnnotations() { return getDeclaredAnnotations(); }
    public boolean isAnnotationPresent(Class<? extends java.lang.annotation.Annotation> a) { return false; }
    static boolean sameParams(Class<?>[] a, Class<?>[] b) {
        if (b == null) b = new Class<?>[0];
        if (a.length != b.length) return false;
        for (int i = 0; i < a.length; i++) if (a[i] != b[i]) return false;
        return true;
    }
    String params() {
        StringBuilder sb = new StringBuilder("(");
        for (int i = 0; i < parameterTypes.length; i++) { if (i > 0) sb.append(','); sb.append(parameterTypes[i].getTypeName()); }
        return sb.append(')').toString();
    }
    /** Runs the method; target exceptions propagate unwrapped (callers wrap them). */
    static native Object invoke0(int slot, boolean virtual, Object receiver, Object[] args, Class<?>[] params, Class<?> ret);
}
