package android.util;

public final class Log {
    public static final int VERBOSE = 2, DEBUG = 3, INFO = 4, WARN = 5, ERROR = 6, ASSERT = 7;
    private Log() {}
    private static native void nativeWrite(int prio, String tag, String msg);
    public static int println(int prio, String tag, String msg) { nativeWrite(prio, tag, msg); return msg == null ? 0 : msg.length(); }
    private static String withTr(String msg, Throwable tr) { return tr == null ? msg : msg + "\n" + getStackTraceString(tr); }
    public static int v(String t, String m) { return println(VERBOSE, t, m); }
    public static int v(String t, String m, Throwable tr) { return println(VERBOSE, t, withTr(m, tr)); }
    public static int d(String t, String m) { return println(DEBUG, t, m); }
    public static int d(String t, String m, Throwable tr) { return println(DEBUG, t, withTr(m, tr)); }
    public static int i(String t, String m) { return println(INFO, t, m); }
    public static int i(String t, String m, Throwable tr) { return println(INFO, t, withTr(m, tr)); }
    public static int w(String t, String m) { return println(WARN, t, m); }
    public static int w(String t, String m, Throwable tr) { return println(WARN, t, withTr(m, tr)); }
    public static int w(String t, Throwable tr) { return println(WARN, t, withTr("", tr)); }
    public static int e(String t, String m) { return println(ERROR, t, m); }
    public static int e(String t, String m, Throwable tr) { return println(ERROR, t, withTr(m, tr)); }
    public static int wtf(String t, Throwable tr) { return println(ASSERT,t,withTr("",tr)); }
    public static int wtf(String t, String m) { return println(ASSERT, t, m); }
    public static int wtf(String t, String m, Throwable tr) { return println(ASSERT, t, withTr(m, tr)); }
    public static boolean isLoggable(String t, int level) { return true; }
    public static String getStackTraceString(Throwable tr) {
        if (tr == null) return "";
        java.io.StringWriter sw = new java.io.StringWriter();
        tr.printStackTrace(new java.io.PrintWriter(sw));
        return sw.toString();
    }
}
