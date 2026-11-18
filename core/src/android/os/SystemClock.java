package android.os;
public final class SystemClock {
    private SystemClock() {}
    public static long uptimeMillis() { return System.nanoTime() / 1000000L; }
    public static long elapsedRealtime() { return System.nanoTime() / 1000000L; }
    public static long elapsedRealtimeNanos() { return System.nanoTime(); }
    public static long currentThreadTimeMillis() { return uptimeMillis(); }
    public static void sleep(long ms) {}
}
