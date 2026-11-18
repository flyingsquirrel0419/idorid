package android.view;
import java.util.ArrayList;
public final class ViewTreeObserver {
    public interface OnGlobalLayoutListener { void onGlobalLayout(); }
    public interface OnPreDrawListener { boolean onPreDraw(); }
    public interface OnDrawListener { void onDraw(); }
    private final ArrayList<OnGlobalLayoutListener> layout = new ArrayList<OnGlobalLayoutListener>();
    private final ArrayList<OnPreDrawListener> preDraw = new ArrayList<OnPreDrawListener>();
    private final ArrayList<OnDrawListener> draw = new ArrayList<OnDrawListener>();
    public void addOnGlobalLayoutListener(OnGlobalLayoutListener l) { layout.add(l); }
    public void removeOnGlobalLayoutListener(OnGlobalLayoutListener l) { layout.remove(l); }
    public void removeGlobalOnLayoutListener(OnGlobalLayoutListener l) { layout.remove(l); }
    public void addOnPreDrawListener(OnPreDrawListener l) { preDraw.add(l); }
    public void removeOnPreDrawListener(OnPreDrawListener l) { preDraw.remove(l); }
    public void addOnDrawListener(OnDrawListener l) { draw.add(l); }
    public void removeOnDrawListener(OnDrawListener l) { draw.remove(l); }
    public void dispatchOnGlobalLayout() { for (OnGlobalLayoutListener l : new ArrayList<OnGlobalLayoutListener>(layout)) l.onGlobalLayout(); }
    public boolean dispatchOnPreDraw() { boolean proceed=true; for (OnPreDrawListener l : new ArrayList<OnPreDrawListener>(preDraw)) if (!l.onPreDraw()) proceed=false; return proceed; }
    public void dispatchOnDraw() { for (OnDrawListener l : new ArrayList<OnDrawListener>(draw)) l.onDraw(); }
    public boolean isAlive() { return true; }
}
