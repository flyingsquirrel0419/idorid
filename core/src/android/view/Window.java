package android.view;
public class Window {
    public static final int FEATURE_NO_TITLE = 1, FEATURE_ACTION_BAR = 8;
    public void setFlags(int f, int m) {}
    public void addFlags(int f) {}
    public void clearFlags(int f) {}
    public void setStatusBarColor(int c) {}
    public void setNavigationBarColor(int c) {}
    public void setSoftInputMode(int m) {}
    public void setBackgroundDrawableResource(int r) {}
    public View getDecorView() { return decor; }
    View decor;
    public WindowManager.LayoutParams getAttributes() { return new WindowManager.LayoutParams(); }
    public void setAttributes(WindowManager.LayoutParams p) {}
}
