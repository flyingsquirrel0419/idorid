package android.os;

public class Handler {
    public interface Callback { boolean handleMessage(Message msg); }
    private final Callback callback;
    public Handler() { this((Callback) null); }
    public Handler(Callback cb) { callback = cb; }
    public Handler(Looper l) { this((Callback) null); }
    public Handler(Looper l, Callback cb) { callback = cb; }
    public static Handler createAsync(Looper l) { return new Handler(l); }
    public void handleMessage(Message msg) {}
    public void dispatchMessage(Message msg) {
        if (msg.callback != null) { msg.callback.run(); return; }
        if (callback != null && callback.handleMessage(msg)) return;
        handleMessage(msg);
    }
    public final Looper getLooper() { return Looper.getMainLooper(); }
    public final boolean post(Runnable r) { return postDelayed(r, 0); }
    public final boolean postAtFrontOfQueue(Runnable r) { return postDelayed(r, 0); }
    public final boolean postDelayed(Runnable r, long delay) { return sendMessageDelayed(Message.obtain(this, r), delay); }
    public final boolean postDelayed(Runnable r, Object token, long delay) { Message m = Message.obtain(this, r); m.obj = token; return sendMessageDelayed(m, delay); }
    public final boolean postAtTime(Runnable r, Object token, long uptime) { Message m = Message.obtain(this, r); m.obj = token; return sendMessageAtTime(m, uptime); }
    public final boolean postAtTime(Runnable r, long uptime) { return sendMessageAtTime(Message.obtain(this, r), uptime); }
    public final void removeCallbacks(Runnable r) { Looper.getMainLooper().remove(this, r, 0, null, false); }
    public final void removeCallbacks(Runnable r, Object token) { Looper.getMainLooper().remove(this, r, 0, token, false); }
    public final void removeMessages(int what) { Looper.getMainLooper().remove(this, null, what, null, true); }
    public final void removeMessages(int what, Object obj) { Looper.getMainLooper().remove(this, null, what, obj, true); }
    public final void removeCallbacksAndMessages(Object token) { Looper.getMainLooper().remove(this, null, 0, token, false); }
    public final boolean hasMessages(int what) { return Looper.getMainLooper().has(this, what); }
    public final Message obtainMessage() { return Message.obtain(this); }
    public final Message obtainMessage(int what) { return Message.obtain(this, what); }
    public final Message obtainMessage(int what, Object obj) { return Message.obtain(this, what, obj); }
    public final Message obtainMessage(int what, int a1, int a2) { return Message.obtain(this, what, a1, a2); }
    public final Message obtainMessage(int what, int a1, int a2, Object obj) { return Message.obtain(this, what, a1, a2, obj); }
    public final boolean sendMessage(Message m) { return sendMessageDelayed(m, 0); }
    public final boolean sendEmptyMessage(int what) { return sendMessage(obtainMessage(what)); }
    public final boolean sendEmptyMessageDelayed(int what, long d) { return sendMessageDelayed(obtainMessage(what), d); }
    public final boolean sendMessageDelayed(Message m, long d) { return sendMessageAtTime(m, SystemClock.uptimeMillis() + Math.max(0, d)); }
    public boolean sendMessageAtTime(Message m, long uptime) {
        m.target = this;
        m.when = uptime;
        Looper.getMainLooper().enqueue(m);
        return true;
    }
}
