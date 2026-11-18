package android.os;
public final class StrictMode {
    public static class ThreadPolicy { public static class Builder { public Builder() {} public Builder(ThreadPolicy p) {} public Builder permitAll() { return this; } public Builder permitDiskReads() { return this; } public Builder permitDiskWrites() { return this; } public ThreadPolicy build() { return new ThreadPolicy(); } } }
    private static ThreadPolicy policy=new ThreadPolicy();
    public static ThreadPolicy getThreadPolicy() { return policy; } public static void setThreadPolicy(ThreadPolicy p) { policy=p; } public static ThreadPolicy allowThreadDiskReads() { return policy; } public static ThreadPolicy allowThreadDiskWrites() { return policy; }
}
