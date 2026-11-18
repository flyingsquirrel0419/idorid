package java.lang;
public abstract class ClassLoader {
    private static ClassLoader system;
    protected ClassLoader() {}
    public static ClassLoader getSystemClassLoader() {
        if (system == null) system = new ClassLoader() {};
        return system;
    }
    public Class<?> loadClass(String name) throws ClassNotFoundException { return Class.forName(name); }
}
