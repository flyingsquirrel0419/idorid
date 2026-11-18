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
    public <U> Class<? extends U> asSubclass(Class<U> type) { if(!type.isAssignableFrom(this))throw new ClassCastException();return (Class<? extends U>)this; }
    // ---------------------------------------------------------------- reflection (VM-backed)
    private native java.lang.reflect.Constructor<?>[] constructors0(boolean publicOnly);
    private native java.lang.reflect.Method[] methods0(boolean publicOnly);
    private native java.lang.reflect.Field[] fields0(boolean publicOnly);
    public native Class<?>[] getInterfaces();
    @SuppressWarnings("unchecked")
    public java.lang.reflect.Constructor<T>[] getDeclaredConstructors() { return (java.lang.reflect.Constructor<T>[]) constructors0(false); }
    @SuppressWarnings("unchecked")
    public java.lang.reflect.Constructor<T>[] getConstructors() { return (java.lang.reflect.Constructor<T>[]) constructors0(true); }
    public java.lang.reflect.Constructor<T> getDeclaredConstructor(Class<?>... types) throws NoSuchMethodException { return findCtor(getDeclaredConstructors(), types); }
    public java.lang.reflect.Constructor<T> getConstructor(Class<?>... types) throws NoSuchMethodException { return findCtor(getConstructors(), types); }
    private java.lang.reflect.Constructor<T> findCtor(java.lang.reflect.Constructor<T>[] cs, Class<?>[] types) throws NoSuchMethodException {
        for (java.lang.reflect.Constructor<T> c : cs) if (sameParams(c.getParameterTypes(), types)) return c;
        throw new NoSuchMethodException(getName() + ".<init>" + describe(types));
    }
    public java.lang.reflect.Method[] getDeclaredMethods() { return methods0(false); }
    public java.lang.reflect.Method[] getMethods() {
        java.util.ArrayList<java.lang.reflect.Method> out = new java.util.ArrayList<java.lang.reflect.Method>();
        java.util.HashSet<String> seen = new java.util.HashSet<String>();
        collectPublic(this, out, seen);
        return out.toArray(new java.lang.reflect.Method[0]);
    }
    private static void collectPublic(Class<?> c, java.util.ArrayList<java.lang.reflect.Method> out, java.util.HashSet<String> seen) {
        for (Class<?> k = c; k != null; k = k.getSuperclass()) {
            for (java.lang.reflect.Method m : k.methods0(true)) if (seen.add(m.getName() + describe(m.getParameterTypes()))) out.add(m);
            for (Class<?> i : k.getInterfaces()) collectPublic(i, out, seen);
        }
    }
    public java.lang.reflect.Method getDeclaredMethod(String name, Class<?>... types) throws NoSuchMethodException {
        java.lang.reflect.Method best = null;
        for (java.lang.reflect.Method m : getDeclaredMethods())
            if (m.getName().equals(name) && sameParams(m.getParameterTypes(), types) && (best == null || !m.isBridge())) best = m;
        if (best == null) throw new NoSuchMethodException(getName() + "." + name + describe(types));
        return best;
    }
    public java.lang.reflect.Method getMethod(String name, Class<?>... types) throws NoSuchMethodException {
        for (java.lang.reflect.Method m : getMethods()) if (m.getName().equals(name) && sameParams(m.getParameterTypes(), types)) return m;
        throw new NoSuchMethodException(getName() + "." + name + describe(types));
    }
    public java.lang.reflect.Field[] getDeclaredFields() { return fields0(false); }
    public java.lang.reflect.Field[] getFields() {
        java.util.ArrayList<java.lang.reflect.Field> out = new java.util.ArrayList<java.lang.reflect.Field>();
        for (Class<?> k = this; k != null; k = k.getSuperclass()) { for (java.lang.reflect.Field f : k.fields0(true)) out.add(f); for (Class<?> i : k.getInterfaces()) for (java.lang.reflect.Field f : i.fields0(true)) out.add(f); }
        return out.toArray(new java.lang.reflect.Field[0]);
    }
    public java.lang.reflect.Field getDeclaredField(String name) throws NoSuchFieldException {
        for (java.lang.reflect.Field f : getDeclaredFields()) if (f.getName().equals(name)) return f;
        throw new NoSuchFieldException(name);
    }
    public java.lang.reflect.Field getField(String name) throws NoSuchFieldException {
        for (java.lang.reflect.Field f : getFields()) if (f.getName().equals(name)) return f;
        throw new NoSuchFieldException(name);
    }
    private static boolean sameParams(Class<?>[] a, Class<?>[] b) {
        if (b == null) b = new Class<?>[0];
        if (a.length != b.length) return false;
        for (int i = 0; i < a.length; i++) if (a[i] != b[i]) return false;
        return true;
    }
    private static String describe(Class<?>[] t) {
        StringBuilder sb = new StringBuilder("(");
        if (t != null) for (int i = 0; i < t.length; i++) { if (i > 0) sb.append(", "); sb.append(t[i] == null ? "null" : t[i].getName()); }
        return sb.append(')').toString();
    }
    public java.lang.reflect.Type[] getGenericInterfaces() { return getInterfaces(); }
    public java.lang.reflect.Type getGenericSuperclass() { return getSuperclass(); }
    public Class<?>[] getDeclaredClasses() { return new Class<?>[0]; }
    public Class<?>[] getClasses() { return new Class<?>[0]; }
    public Class<?> getDeclaringClass() { return getEnclosingClass(); }
    public Class<?> getEnclosingClass() {
        String n = getName(); int i = n.lastIndexOf('$');
        if (i < 0) return null;
        try { return forName(n.substring(0, i)); } catch (ClassNotFoundException e) { return null; }
    }
    public boolean isAnonymousClass() { String n = getName(); int i = n.lastIndexOf('$'); return i >= 0 && i + 1 < n.length() && Character.isDigit(n.charAt(i + 1)); }
    public boolean isLocalClass() { return false; }
    public boolean isMemberClass() { return getName().indexOf('$') >= 0 && !isAnonymousClass(); }
    public boolean isSynthetic() { return (getModifiers() & 0x1000) != 0; }
    public boolean isAnnotation() { return (getModifiers() & 0x2000) != 0; }
    public java.lang.annotation.Annotation[] getAnnotations() { return new java.lang.annotation.Annotation[0]; }
    public java.lang.annotation.Annotation[] getDeclaredAnnotations() { return new java.lang.annotation.Annotation[0]; }
    public java.lang.reflect.TypeVariable<Class<T>>[] getTypeParameters() { @SuppressWarnings("unchecked") java.lang.reflect.TypeVariable<Class<T>>[] r = new java.lang.reflect.TypeVariable[0]; return r; }
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
    public <A extends java.lang.annotation.Annotation> A getDeclaredAnnotation(Class<A> a) { return null; }
    public String toString() { return (isInterface() ? "interface " : isPrimitive() ? "" : "class ") + getName(); }
}
