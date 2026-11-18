package android.view;
import android.os.Handler;
import java.util.HashMap;
public final class Choreographer {
    public interface FrameCallback { void doFrame(long frameTimeNanos); }
    private static final Choreographer INSTANCE=new Choreographer();
    private final Handler handler=new Handler();
    private final HashMap<FrameCallback,Runnable> callbacks=new HashMap<FrameCallback,Runnable>();
    public static Choreographer getInstance() { return INSTANCE; }
    public void postFrameCallback(final FrameCallback callback) { postFrameCallbackDelayed(callback,0); }
    public void postFrameCallbackDelayed(final FrameCallback callback,long delay) { removeFrameCallback(callback); Runnable r=new Runnable() { public void run() { callbacks.remove(callback); callback.doFrame(System.nanoTime()); } }; callbacks.put(callback,r); handler.postDelayed(r,Math.max(0,delay)+16); }
    public void removeFrameCallback(FrameCallback callback) { Runnable r=callbacks.remove(callback); if (r!=null) handler.removeCallbacks(r); }
}
