package android.view.animation;
import android.os.Handler;
public class Animation {
    public interface AnimationListener { void onAnimationStart(Animation animation); void onAnimationEnd(Animation animation); void onAnimationRepeat(Animation animation); }
    public static final int ABSOLUTE=0, RELATIVE_TO_SELF=1, RELATIVE_TO_PARENT=2, INFINITE=-1, RESTART=1, REVERSE=2;
    private AnimationListener listener; private long duration,startOffset; private Interpolator interpolator; private boolean started,ended;
    public void setAnimationListener(AnimationListener l) { listener=l; }
    public void setDuration(long d) { duration=d; } public long getDuration() { return duration; }
    public void setStartOffset(long d) { startOffset=d; } public long getStartOffset() { return startOffset; }
    public void setInterpolator(Interpolator i) { interpolator=i; } public Interpolator getInterpolator() { return interpolator; }
    public void setFillAfter(boolean fill) {} public void setFillBefore(boolean fill) {} public void setRepeatCount(int count) {} public void setRepeatMode(int mode) {}
    public boolean hasStarted() { return started; } public boolean hasEnded() { return ended; }
    public void start() { started=true; ended=false; if (listener!=null) listener.onAnimationStart(this); new Handler().post(new Runnable() { public void run() { finish(); } }); }
    public void startNow() { start(); } public void cancel() { finish(); } public void reset() { started=ended=false; }
    private void finish() { if (!ended) { ended=true; if (listener!=null) listener.onAnimationEnd(this); } }
    public boolean getTransformation(long currentTime,Transformation out,float scale) { return getTransformation(currentTime,out); }
    public boolean getTransformation(long currentTime,Transformation out) { if (!started) start(); return !ended; }
}
