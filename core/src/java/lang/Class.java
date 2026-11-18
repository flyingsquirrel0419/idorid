package java.lang;

public final class Class<T> implements java.io.Serializable, java.lang.reflect.Type {
    private Class() {}
    public native String getName();
    public String getSimpleName() {
        String n = getName();
        if (isArray()) return getComponentType().getSimpleName() + "[]";
        int i = Math.max(n.lastIndexOf('.'), n.lastIndexOf('$'));
        return n.substring(i + 1);
    }
    public String getCanonicalName() { return getName().replace('$', '.'); }
    public String getTypeName() { return getName(); }
    public native boolean isInstance(Object o);
    public native boolean isAssignableFrom(Class<?> c);
    public native boolean isInterface();
    public native boolean isArray();
    public native boolean isPrimitive();
    public boolean isEnum() { return getSuperclass() == Enum.class; }
    public native Class<? super T> getSuperclass();
    public native Class<?> getComponentType();
    public native T newInstance() throws InstantiationException, IllegalAccessException;
    public native T[] getEnumConstants();
    public native int getModifiers();
    public static native Class<?> forName(String name) throws ClassNotFoundException;
    public static Class<?> forName(String name, boolean init, ClassLoader l) throws ClassNotFoundException { return forName(name); }
    public ClassLoader getClassLoader() { return ClassLoader.getSystemClassLoader(); }
    public Package getPackage() { return null; }
    public boolean desiredAssertionStatus() { return false; }
    @SuppressWarnings("unchecked")
    public T cast(Object o) {
        if (o != null && !isInstance(o)) throw new ClassCastException(o.getClass().getName() + " cannot be cast to " + getName());
        return (T) o;
    }
    public boolean isAnnotationPresent(Class<? extends java.lang.annotation.Annotation> a) { return false; }
    public <A extends java.lang.annotation.Annotation> A getAnnotation(Class<A> a) { return null; }
    public String toString() { return (isInterface() ? "interface " : isPrimitive() ? "" : "class ") + getName(); }
}
