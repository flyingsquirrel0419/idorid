package android.widget;
import android.content.Context;
public class Toast {
    public static final int LENGTH_SHORT = 0, LENGTH_LONG = 1;
    private CharSequence text;
    private int duration;
    public Toast(Context c) {}
    public static Toast makeText(Context c, CharSequence t, int d) { Toast x = new Toast(c); x.text = t; x.duration = d; return x; }
    public static Toast makeText(Context c, int resId, int d) { return makeText(c, c.getResources().getText(resId), d); }
    public void show() { android.app.ActivityThread.toast(String.valueOf(text)); }
    public void cancel() {}
    public void setText(CharSequence t) { text = t; }
    public void setText(int r) {}
    public void setDuration(int d) { duration = d; }
    public int getDuration() { return duration; }
    public void setGravity(int g, int x, int y) {}
    public void setView(android.view.View v) {}
}
