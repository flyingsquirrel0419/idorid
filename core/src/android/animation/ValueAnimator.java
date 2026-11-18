package android.animation;
import android.os.Handler;
import android.os.SystemClock;
import java.util.ArrayList;
public class ValueAnimator extends Animator {
    public interface AnimatorUpdateListener { void onAnimationUpdate(ValueAnimator animation); }
    public static final int INFINITE = -1, RESTART = 1, REVERSE = 2;
    private final Handler handler = new Handler();
    private final ArrayList<AnimatorUpdateListener> updates = new ArrayList<AnimatorUpdateListener>();
    private float[] floats = {0, 1}; private int[] ints; private Object value; private float fraction;
    private long pauseTime;
    private long startTime; private int repeatCount, repeatMode = RESTART, iteration;
    private final Runnable frame = new Runnable() { public void run() { doFrame(); } };
    public static ValueAnimator ofFloat(float... values) { ValueAnimator a = new ValueAnimator(); a.setFloatValues(values); return a; }
    public static ValueAnimator ofInt(int... values) { ValueAnimator a = new ValueAnimator(); a.setIntValues(values); return a; }
    public void setFloatValues(float... values) { floats = values; ints = null; }
    public void setIntValues(int... values) { ints = values; }
    public void addUpdateListener(AnimatorUpdateListener l) { if (l != null) updates.add(l); }
    public void removeUpdateListener(AnimatorUpdateListener l) { updates.remove(l); }
    public void removeAllUpdateListeners() { updates.clear(); }
    public Object getAnimatedValue() { return value; }
    public Object getAnimatedValue(String name) { return value; }
    public float getAnimatedFraction() { return fraction; }
    public void setRepeatCount(int n) { repeatCount = n; } public int getRepeatCount() { return repeatCount; }
    public void setRepeatMode(int mode) { repeatMode = mode; } public int getRepeatMode() { return repeatMode; }
    public ValueAnimator setDuration(long d) { super.setDuration(d); return this; }
    public void start() { if (started) cancel(); paused=false; started = true; running = true; iteration = 0; startTime = SystemClock.uptimeMillis() + startDelay; notifyStart(); if (!started) return; if (startDelay == 0) applyFraction(0); handler.postDelayed(frame, startDelay == 0 ? 16 : startDelay); }
    public void pause() { if (started && !paused) { pauseTime=SystemClock.uptimeMillis(); handler.removeCallbacks(frame); super.pause(); } }
    public void resume() { if(paused) { startTime+=SystemClock.uptimeMillis()-pauseTime; super.resume(); if(started) handler.postDelayed(frame,16); } }
    public PropertyValuesHolder[] getValues() { return new PropertyValuesHolder[] { ints==null ? PropertyValuesHolder.ofFloat("",floats) : PropertyValuesHolder.ofInt("",ints) }; }
    public void reverse() { if (started) { long elapsed=SystemClock.uptimeMillis()-startTime; startTime=SystemClock.uptimeMillis()-Math.max(0,duration-elapsed); repeatMode=REVERSE; iteration++; } else { repeatMode=REVERSE; start(); iteration=1; applyFraction(1); } }
    private void doFrame() {
        if (!started) return;
        long now = SystemClock.uptimeMillis(); if (now < startTime) { handler.postDelayed(frame, startTime-now); return; }
        float f = duration == 0 ? 1 : (float)(now-startTime)/duration;
        if (f >= 1) {
            applyFraction(repeatMode == REVERSE && (iteration & 1) != 0 ? 0 : 1);
            if (!started) return;
            if (repeatCount == INFINITE || iteration < repeatCount) { iteration++; startTime = now; for (AnimatorListener l : new ArrayList<AnimatorListener>(listeners)) l.onAnimationRepeat(this); handler.postDelayed(frame, 16); }
            else finish();
        } else { applyFraction(repeatMode == REVERSE && (iteration & 1) != 0 ? 1-f : f); if (started) handler.postDelayed(frame, 16); }
    }
    protected void applyFraction(float f) {
        fraction = interpolator == null ? f : interpolator.getInterpolation(f);
        int n = ints != null ? ints.length : floats.length;
        if (n > 0) { float pos = fraction * (n-1); int i = (int)pos; if (i < 0) i = 0; if (i >= n-1) i = n-1; int j = Math.min(i+1,n-1); float t = pos-i;
            value = ints != null ? (Object)Integer.valueOf(ints[i] + (int)((ints[j]-ints[i])*t)) : (Object)Float.valueOf(floats[i] + (floats[j]-floats[i])*t); }
        animateValue(value);
        for (AnimatorUpdateListener l : new ArrayList<AnimatorUpdateListener>(updates)) l.onAnimationUpdate(this);
    }
    protected void animateValue(Object v) {}
    private void finish() { handler.removeCallbacks(frame); started = running = paused = false; notifyEnd(); }
    public void end() { if (!started) { started = running = true; notifyStart(); } applyFraction(repeatMode == REVERSE && repeatCount != INFINITE && (repeatCount & 1) != 0 ? 0 : 1); if (started) finish(); }
    public void cancel() { if (!started) return; handler.removeCallbacks(frame); started = running = paused = false; notifyCancel(); notifyEnd(); }
}
