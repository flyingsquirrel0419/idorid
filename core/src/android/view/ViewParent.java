package android.view;
public interface ViewParent {
    ViewParent getParent();
    void requestLayout();
    void requestDisallowInterceptTouchEvent(boolean b);
}
