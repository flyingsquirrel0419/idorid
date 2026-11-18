package android.os;

/**
 * Single main looper. The host drives it: after every event the VM calls
 * drain(); delayed messages report their due time so the host can schedule a
 * timer and call back.
 */
public final class Looper {
    private static final Looper MAIN = new Looper();
    Message head;
    private Looper() {}
    public static Looper getMainLooper() { return MAIN; }
    public static Looper myLooper() { return MAIN; }
    public static void prepare() {}
    public static void prepareMainLooper() {}
    public static void loop() { drain(); }
    public void quit() {}
    public void quitSafely() {}
    public Thread getThread() { return Thread.currentThread(); }
    public boolean isCurrentThread() { return true; }

    void enqueue(Message m) {
        if (head == null || m.when < head.when) { m.next = head; head = m; return; }
        Message p = head;
        while (p.next != null && p.next.when <= m.when) p = p.next;
        m.next = p.next;
        p.next = m;
    }
    void remove(Handler h, Runnable r, int what, Object token, boolean byWhat) {
        Message prev = null, p = head;
        while (p != null) {
            boolean match = p.target == h && (r != null ? p.callback == r : byWhat ? (p.callback == null && p.what == what) : true)
                && (token == null || p.obj == token);
            if (match) { if (prev == null) head = p.next; else prev.next = p.next; }
            else prev = p;
            p = p.next;
        }
    }
    boolean has(Handler h, int what) {
        for (Message p = head; p != null; p = p.next) if (p.target == h && p.callback == null && p.what == what) return true;
        return false;
    }

    /** Runs all due messages (and started threads). Returns ms until the next one, or -1. */
    public static long drain() {
        int guard = 0;
        while (true) {
            java.lang.VMThread.runPending();
            Message m = MAIN.head;
            if (m == null) return -1;
            long now = SystemClock.uptimeMillis();
            if (m.when > now) return m.when - now;
            MAIN.head = m.next;
            m.next = null;
            m.target.dispatchMessage(m);
            if (++guard > 10000) return 0;
        }
    }
}
