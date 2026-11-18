package android.view;
public interface WindowManager extends ViewManager {
    class LayoutParams extends ViewGroup.LayoutParams {
        public static final int FLAG_FULLSCREEN = 0x400, FLAG_KEEP_SCREEN_ON = 0x80, SOFT_INPUT_ADJUST_RESIZE = 0x10, SOFT_INPUT_STATE_HIDDEN = 2;
        public int flags, softInputMode;
        public float screenBrightness = -1;
        public LayoutParams() { super(MATCH_PARENT, MATCH_PARENT); }
    }
}
