package android.widget;
import android.content.Context;
import android.util.AttributeSet;
public class SeekBar extends ProgressBar {
    public interface OnSeekBarChangeListener { void onProgressChanged(SeekBar s, int p, boolean fromUser); void onStartTrackingTouch(SeekBar s); void onStopTrackingTouch(SeekBar s); }
    private OnSeekBarChangeListener listener;
    public SeekBar(Context c) { super(c); setIndeterminate(false); }
    public SeekBar(Context c, AttributeSet a) { super(c, a); setIndeterminate(false); }
    public void setOnSeekBarChangeListener(OnSeekBarChangeListener l) { listener = l; }
    public void setProgress(int p) { super.setProgress(p); if (listener != null) listener.onProgressChanged(this, getProgress(), false); }
    /** idorid: user dragged the host slider. */
    public void idoridUserProgress(int p) {
        if (listener != null) listener.onStartTrackingTouch(this);
        super.setProgress(p);
        if (listener != null) { listener.onProgressChanged(this, getProgress(), true); listener.onStopTrackingTouch(this); }
    }
    protected String idoridType() { return "SeekBar"; }
}
