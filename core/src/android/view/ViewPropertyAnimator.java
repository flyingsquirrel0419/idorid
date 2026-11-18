package android.view;
/** Animations complete instantly. */
public class ViewPropertyAnimator {
    private final View v;
    private Runnable end;
    ViewPropertyAnimator(View v) { this.v = v; }
    public ViewPropertyAnimator alpha(float a) { v.setAlpha(a); return this; }
    public ViewPropertyAnimator translationX(float x) { return this; }
    public ViewPropertyAnimator translationY(float y) { return this; }
    public ViewPropertyAnimator scaleX(float s) { return this; }
    public ViewPropertyAnimator scaleY(float s) { return this; }
    public ViewPropertyAnimator rotation(float r) { return this; }
    public ViewPropertyAnimator setDuration(long d) { return this; }
    public ViewPropertyAnimator setStartDelay(long d) { return this; }
    public ViewPropertyAnimator setInterpolator(Object i) { return this; }
    public ViewPropertyAnimator withEndAction(Runnable r) { end = r; return this; }
    public ViewPropertyAnimator withStartAction(Runnable r) { r.run(); return this; }
    public void start() { if (end != null) { Runnable r = end; end = null; v.post(r); } }
    public void cancel() {}
}
