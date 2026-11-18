package android.os;
import java.util.ArrayList;
public final class MessageQueue {
    public interface IdleHandler { boolean queueIdle(); }
    private final ArrayList<IdleHandler> idleHandlers = new ArrayList<IdleHandler>();
    public void addIdleHandler(IdleHandler handler) { if (handler == null) throw new NullPointerException(); idleHandlers.add(handler); }
    public void removeIdleHandler(IdleHandler handler) { idleHandlers.remove(handler); }
    void dispatchIdle() { for (IdleHandler h : new ArrayList<IdleHandler>(idleHandlers)) if (!h.queueIdle()) idleHandlers.remove(h); }
    public boolean isIdle() { return Looper.getMainLooper().head == null || Looper.getMainLooper().head.when > SystemClock.uptimeMillis(); }
}
