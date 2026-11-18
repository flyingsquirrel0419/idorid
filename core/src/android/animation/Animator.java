package android.animation;
import java.util.ArrayList;
public abstract class Animator implements Cloneable {
    public interface AnimatorListener { void onAnimationStart(Animator a); void onAnimationEnd(Animator a); void onAnimationCancel(Animator a); void onAnimationRepeat(Animator a); }
    public interface AnimatorPauseListener { void onAnimationPause(Animator a); void onAnimationResume(Animator a); }
    protected ArrayList<AnimatorListener> listeners = new ArrayList<AnimatorListener>();
    protected long duration = 300, startDelay;
    protected TimeInterpolator interpolator;
    protected boolean running, started;
    public abstract void start();
    public abstract void end();
    public abstract void cancel();
    public Animator setDuration(long d) { if (d < 0) throw new IllegalArgumentException(); duration = d; return this; }
    public long getDuration() { return duration; }
    public void setStartDelay(long d) { startDelay = Math.max(0, d); }
    public long getStartDelay() { return startDelay; }
    public void setInterpolator(TimeInterpolator i) { interpolator = i; }
    public TimeInterpolator getInterpolator() { return interpolator; }
    public boolean isRunning() { return running; }
    public boolean isStarted() { return started; }
    public void addListener(AnimatorListener l) { if (l != null) listeners.add(l); }
    public void removeListener(AnimatorListener l) { listeners.remove(l); }
    public void removeAllListeners() { listeners.clear(); }
    public ArrayList<AnimatorListener> getListeners() { return listeners; }
    protected void notifyStart() { for (AnimatorListener l : new ArrayList<AnimatorListener>(listeners)) l.onAnimationStart(this); }
    protected void notifyEnd() { for (AnimatorListener l : new ArrayList<AnimatorListener>(listeners)) l.onAnimationEnd(this); }
    protected void notifyCancel() { for (AnimatorListener l : new ArrayList<AnimatorListener>(listeners)) l.onAnimationCancel(this); }
}
