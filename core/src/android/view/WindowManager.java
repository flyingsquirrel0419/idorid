package android.view;
public interface WindowManager extends ViewManager {
    Display getDefaultDisplay();
    class LayoutParams extends ViewGroup.LayoutParams {
        public static final int FLAG_FULLSCREEN = 0x400, FLAG_KEEP_SCREEN_ON = 0x80, SOFT_INPUT_ADJUST_RESIZE = 0x10, SOFT_INPUT_STATE_HIDDEN = 2;
        public int flags, softInputMode, format, gravity, layoutInDisplayCutoutMode, type, windowAnimations, x, y;
        public String packageName; public android.os.IBinder token; private CharSequence title;
        public float screenBrightness = -1;
        public LayoutParams() { super(MATCH_PARENT, MATCH_PARENT); }
        public LayoutParams(int width,int height,int x,int y,int type,int flags,int format) { super(width,height); this.x=x;this.y=y;this.type=type;this.flags=flags;this.format=format; }
        public void setTitle(CharSequence title) { this.title=title; } public CharSequence getTitle() { return title; }
    }
 default WindowMetrics getCurrentWindowMetrics() { Display display=getDefaultDisplay(); return new WindowMetrics(new android.graphics.Rect(0,0,display.getWidth(),display.getHeight()), WindowInsets.CONSUMED); }
}
