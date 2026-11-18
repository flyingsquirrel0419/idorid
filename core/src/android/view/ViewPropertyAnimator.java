package android.view;
import android.animation.Animator;
import android.animation.AnimatorListenerAdapter;
import android.animation.TimeInterpolator;
import android.animation.ValueAnimator;
import android.os.Handler;
import java.util.ArrayList;
public class ViewPropertyAnimator {
    private final View view;
    private final ArrayList<String> names = new ArrayList<String>();
    private final ArrayList<Float> targets = new ArrayList<Float>();
    private final Handler handler = new Handler();
    private long duration=300, delay;
    private TimeInterpolator interpolator;
    private Animator.AnimatorListener listener;
    private ValueAnimator.AnimatorUpdateListener updateListener;
    private Runnable endAction, startAction;
    private ValueAnimator animator;
    private final Runnable starter = new Runnable() { public void run() { start(); } };
    ViewPropertyAnimator(View v) { view=v; }
    private ViewPropertyAnimator property(String name,float value) { int i=names.indexOf(name); if (i<0) { names.add(name); targets.add(Float.valueOf(value)); } else targets.set(i,Float.valueOf(value)); handler.removeCallbacks(starter); handler.post(starter); return this; }
    public ViewPropertyAnimator alpha(float a) { return property("alpha",a); }
    public ViewPropertyAnimator translationX(float x) { return property("translationX",x); }
    public ViewPropertyAnimator translationY(float y) { return property("translationY",y); }
    public ViewPropertyAnimator scaleX(float s) { return property("scaleX",s); }
    public ViewPropertyAnimator scaleY(float s) { return property("scaleY",s); }
    public ViewPropertyAnimator rotation(float r) { return property("rotation",r); }
    public ViewPropertyAnimator x(float x) { return property("x",x); }
    public ViewPropertyAnimator y(float y) { return property("y",y); }
    public ViewPropertyAnimator setDuration(long d) { if (d<0) throw new IllegalArgumentException(); duration=d; return this; }
    public long getDuration() { return duration; }
    public ViewPropertyAnimator setStartDelay(long d) { delay=Math.max(0,d); return this; }
    public long getStartDelay() { return delay; }
    public ViewPropertyAnimator setInterpolator(TimeInterpolator i) { interpolator=i; return this; }
    public ViewPropertyAnimator setListener(Animator.AnimatorListener l) { listener=l; return this; }
    public ViewPropertyAnimator setUpdateListener(ValueAnimator.AnimatorUpdateListener l) { updateListener=l; return this; }
    public ViewPropertyAnimator withEndAction(Runnable r) { endAction=r; return this; }
    public ViewPropertyAnimator withStartAction(Runnable r) { startAction=r; return this; }
    private float get(String n) { if ("alpha".equals(n)) return view.getAlpha(); if ("translationX".equals(n)) return view.getTranslationX(); if ("translationY".equals(n)) return view.getTranslationY(); if ("scaleX".equals(n)) return view.getScaleX(); if ("scaleY".equals(n)) return view.getScaleY(); if ("rotation".equals(n)) return view.getRotation(); if ("x".equals(n)) return view.getX(); return view.getY(); }
    private void set(String n,float v) { if ("alpha".equals(n)) view.setAlpha(v); else if ("translationX".equals(n)) view.setTranslationX(v); else if ("translationY".equals(n)) view.setTranslationY(v); else if ("scaleX".equals(n)) view.setScaleX(v); else if ("scaleY".equals(n)) view.setScaleY(v); else if ("rotation".equals(n)) view.setRotation(v); else if ("x".equals(n)) view.setX(v); else view.setY(v); }
    public void start() {
        handler.removeCallbacks(starter); if (names.isEmpty()) return; if (animator!=null) animator.cancel();
        final ArrayList<String> ns=new ArrayList<String>(names); final ArrayList<Float> ends=new ArrayList<Float>(targets); final ArrayList<Float> starts=new ArrayList<Float>();
        names.clear(); targets.clear(); for (String n : ns) starts.add(Float.valueOf(get(n)));
        final Runnable end=endAction, begin=startAction; endAction=startAction=null;
        final Animator.AnimatorListener external=listener; final ValueAnimator.AnimatorUpdateListener update=updateListener;
        animator=ValueAnimator.ofFloat(0,1); animator.setDuration(duration); animator.setStartDelay(delay); animator.setInterpolator(interpolator);
        animator.addUpdateListener(new ValueAnimator.AnimatorUpdateListener() { public void onAnimationUpdate(ValueAnimator a) { float f=((Float)a.getAnimatedValue()).floatValue(); for (int i=0;i<ns.size();i++) { float s=starts.get(i).floatValue(); set(ns.get(i),s+(ends.get(i).floatValue()-s)*f); } if (update!=null) update.onAnimationUpdate(a); } });
        animator.addListener(new AnimatorListenerAdapter() { boolean cancelled; public void onAnimationStart(Animator a) { if (begin!=null) begin.run(); if (external!=null) external.onAnimationStart(a); } public void onAnimationCancel(Animator a) { cancelled=true; if (external!=null) external.onAnimationCancel(a); } public void onAnimationEnd(Animator a) { if (!cancelled && end!=null) end.run(); if (external!=null) external.onAnimationEnd(a); } });
        animator.start();
    }
    public void cancel() { handler.removeCallbacks(starter); names.clear(); targets.clear(); if (animator!=null) animator.cancel(); }
}
