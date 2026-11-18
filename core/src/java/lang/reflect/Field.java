package java.lang.reflect;

public final class Field extends AccessibleObject implements Member {
    Class<?> declaringClass;
    String name;
    Class<?> type;
    int modifiers;
    int slot;
    private Field() {}
    public Class<?> getDeclaringClass() { return declaringClass; }
    public String getName() { return name; }
    public Class<?> getType() { return type; }
    public Type getGenericType() { return type; }
    public int getModifiers() { return modifiers; }
    public boolean isSynthetic() { return (modifiers & 0x1000) != 0; }
    public boolean isEnumConstant() { return (modifiers & 0x4000) != 0; }
    public <T extends java.lang.annotation.Annotation> T getAnnotation(Class<T> a) { return null; }
    public java.lang.annotation.Annotation[] getDeclaredAnnotations() { return new java.lang.annotation.Annotation[0]; }
    public boolean isAnnotationPresent(Class<? extends java.lang.annotation.Annotation> a) { return false; }
    private Object check(Object target) {
        if ((modifiers & Modifier.STATIC) != 0) return null;
        if (target == null) throw new NullPointerException("null receiver");
        if (!declaringClass.isInstance(target)) throw new IllegalArgumentException("Expected receiver of type " + declaringClass.getName());
        return target;
    }
    public Object get(Object target) throws IllegalAccessException { return get0(declaringClass, slot, (modifiers & Modifier.STATIC) != 0, check(target), type); }
    public void set(Object target, Object value) throws IllegalAccessException {
        if (type.isPrimitive() && value == null) throw new IllegalArgumentException("null for primitive field");
        set0(declaringClass, slot, (modifiers & Modifier.STATIC) != 0, check(target), type, value);
    }
    private static native Object get0(Class<?> c, int slot, boolean isStatic, Object target, Class<?> type);
    private static native void set0(Class<?> c, int slot, boolean isStatic, Object target, Class<?> type, Object value);
    public boolean getBoolean(Object t) throws IllegalAccessException { return ((Boolean) get(t)).booleanValue(); }
    public byte getByte(Object t) throws IllegalAccessException { return ((Number) get(t)).byteValue(); }
    public char getChar(Object t) throws IllegalAccessException { return ((Character) get(t)).charValue(); }
    public short getShort(Object t) throws IllegalAccessException { return ((Number) get(t)).shortValue(); }
    public int getInt(Object t) throws IllegalAccessException { Object v = get(t); return v instanceof Character ? ((Character) v).charValue() : ((Number) v).intValue(); }
    public long getLong(Object t) throws IllegalAccessException { return ((Number) get(t)).longValue(); }
    public float getFloat(Object t) throws IllegalAccessException { return ((Number) get(t)).floatValue(); }
    public double getDouble(Object t) throws IllegalAccessException { return ((Number) get(t)).doubleValue(); }
    public void setBoolean(Object t, boolean v) throws IllegalAccessException { set(t, Boolean.valueOf(v)); }
    public void setByte(Object t, byte v) throws IllegalAccessException { set(t, Byte.valueOf(v)); }
    public void setChar(Object t, char v) throws IllegalAccessException { set(t, Character.valueOf(v)); }
    public void setShort(Object t, short v) throws IllegalAccessException { set(t, Short.valueOf(v)); }
    public void setInt(Object t, int v) throws IllegalAccessException { set(t, Integer.valueOf(v)); }
    public void setLong(Object t, long v) throws IllegalAccessException { set(t, Long.valueOf(v)); }
    public void setFloat(Object t, float v) throws IllegalAccessException { set(t, Float.valueOf(v)); }
    public void setDouble(Object t, double v) throws IllegalAccessException { set(t, Double.valueOf(v)); }
    public boolean equals(Object o) { return o instanceof Field && ((Field) o).declaringClass == declaringClass && ((Field) o).name.equals(name); }
    public int hashCode() { return declaringClass.getName().hashCode() ^ name.hashCode(); }
    public String toString() { return Modifier.toString(modifiers) + (modifiers == 0 ? "" : " ") + type.getTypeName() + " " + declaringClass.getName() + "." + name; }
}
