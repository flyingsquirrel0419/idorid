package java.lang;
public class Runtime {
    private static final Runtime INSTANCE = new Runtime();
    public static Runtime getRuntime() { return INSTANCE; }
    public int availableProcessors() { return 1; }
    public long maxMemory() { return 256L * 1024 * 1024; }
    public long totalMemory() { return 64L * 1024 * 1024; }
    public long freeMemory() { return 32L * 1024 * 1024; }
    public void gc() {}
    public void exit(int c) { System.exit(c); }
    public void loadLibrary(String l) { System.loadLibrary(l); }
}
