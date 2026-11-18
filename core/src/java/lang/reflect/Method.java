package java.lang.reflect;

public final class Method extends Executable {
    Class<?> returnType;
    private Method() {}
    public Class<?> getReturnType() { return returnType; }
    public Type getGenericReturnType() { return returnType; }
    public boolean isDefault() { return declaringClass.isInterface() && (modifiers & Modifier.ABSTRACT) == 0 && (modifiers & Modifier.STATIC) == 0; }
    public boolean isBridge() { return (modifiers & 0x40) != 0; }
    public Object getDefaultValue() { return null; }
    public Object invoke(Object target, Object... args) throws IllegalAccessException, InvocationTargetException {
        boolean isStatic = (modifiers & Modifier.STATIC) != 0;
        if (!isStatic) {
            if (target == null) throw new NullPointerException("null receiver");
            if (!declaringClass.isInstance(target)) throw new IllegalArgumentException("Expected receiver of type " + declaringClass.getName() + ", but got " + target.getClass().getName());
        }
        if (args == null) args = new Object[0];
        if (args.length != parameterTypes.length) throw new IllegalArgumentException("Wrong number of arguments; expected " + parameterTypes.length + ", got " + args.length);
        try {
            return invoke0(slot, !isStatic && (modifiers & Modifier.PRIVATE) == 0, target, args, parameterTypes, returnType);
        } catch (IllegalArgumentException e) {
            throw e;
        } catch (Throwable t) {
            throw new InvocationTargetException(t);
        }
    }
    public boolean equals(Object o) { return o instanceof Method && ((Method) o).slot == slot; }
    public int hashCode() { return declaringClass.getName().hashCode() ^ name.hashCode(); }
    public String toString() { return Modifier.toString(modifiers) + (modifiers == 0 ? "" : " ") + returnType.getTypeName() + " " + declaringClass.getName() + "." + name + params(); }
}
