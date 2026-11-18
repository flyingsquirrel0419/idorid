package android.content.res;
public class Configuration {
    public static final int ORIENTATION_PORTRAIT = 1, ORIENTATION_LANDSCAPE = 2;
    public static final int UI_MODE_NIGHT_MASK = 0x30, UI_MODE_NIGHT_NO = 0x10, UI_MODE_NIGHT_YES = 0x20;
    public int orientation = ORIENTATION_PORTRAIT;
    public int uiMode = UI_MODE_NIGHT_NO;
    public float fontScale = 1f;
    public int screenWidthDp = 390, screenHeightDp = 844, smallestScreenWidthDp = 390, densityDpi = 160;
    public java.util.Locale locale = java.util.Locale.getDefault();
}
