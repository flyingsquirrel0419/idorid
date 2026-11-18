package java.lang.reflect;

public final class Constructor<T> extends Executable {
    private Constructor() {}
    @SuppressWarnings("unchecked")
    public Class<T> getDeclaringClass() { return (Class<T>) declaringClass; }
    public String getName() { return declaringClass.getName(); }
    public T newInstance(Object... args) throws InstantiationException, IllegalAccessException, InvocationTargetException {
        if ((declaringClass.getModifiers() & Modifier.ABSTRACT) != 0) throw new InstantiationException("Can't instantiate abstract class " + declaringClass.getName());
        if (args == null) args = new Object[0];
        if (args.length != parameterTypes.length) throw new IllegalArgumentException("Wrong number of arguments; expected " + parameterTypes.length + ", got " + args.length);
        Object o = allocate(declaringClass);
        try {
            invoke0(slot, false, o, args, parameterTypes, void.class);
        } catch (IllegalArgumentException e) {
            throw e;
        } catch (Throwable t) {
            throw new InvocationTargetException(t);
        }
        @SuppressWarnings("unchecked") T r = (T) o;
        return r;
    }
    private static native Object allocate(Class<?> c);
    public boolean equals(Object o) { return o instanceof Constructor && ((Constructor<?>) o).slot == slot; }
    public int hashCode() { return declaringClass.getName().hashCode(); }
    public String toString() { return Modifier.toString(modifiers) + (modifiers == 0 ? "" : " ") + declaringClass.getName() + params(); }
}
