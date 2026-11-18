package android.animation;
import java.util.ArrayList;
import java.util.Collection;
public class AnimatorSet extends Animator {
    private final ArrayList<Animator> children = new ArrayList<Animator>();
    private int remaining;
    private final AnimatorListenerAdapter childListener = new AnimatorListenerAdapter() { public void onAnimationEnd(Animator a) { a.removeListener(this); if (started && --remaining == 0) { started=running=false; notifyEnd(); } } };
    public void playTogether(Animator... items) { children.clear(); for (Animator a : items) children.add(a); }
    public void playTogether(Collection<Animator> items) { children.clear(); children.addAll(items); }
    public ArrayList<Animator> getChildAnimations() { return new ArrayList<Animator>(children); }
    public AnimatorSet setDuration(long d) { super.setDuration(d); for (Animator a : children) a.setDuration(d); return this; }
    public void start() { if (started) cancel(); started=running=true; remaining=children.size(); notifyStart(); if (!started) return; if (remaining==0) { started=running=false; notifyEnd(); return; } for (Animator a : children) { a.addListener(childListener); a.start(); } }
    public void end() { if (!started) start(); for (Animator a : children) if (a.isStarted()) a.end(); }
    public void cancel() { if (!started) return; started=running=false; notifyCancel(); for (Animator a : children) { a.removeListener(childListener); a.cancel(); } notifyEnd(); }
}
