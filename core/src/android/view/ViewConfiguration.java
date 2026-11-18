package android.view;
import android.content.Context;
public class ViewConfiguration {
    public static ViewConfiguration get(Context c) { return new ViewConfiguration(); }
    public static int getLongPressTimeout() { return 500; } public static int getTapTimeout() { return 100; } public static int getDoubleTapTimeout() { return 300; }
    public static int getTouchSlop() { return 8; } public static int getMinimumFlingVelocity() { return 50; } public static int getMaximumFlingVelocity() { return 8000; }
    public int getScaledTouchSlop() { return 8; } public int getScaledPagingTouchSlop() { return 16; } public int getScaledDoubleTapSlop() { return 100; }
    public int getScaledMinimumFlingVelocity() { return 50; } public int getScaledMaximumFlingVelocity() { return 8000; }
    public int getScaledEdgeSlop() { return 12; } public int getScaledWindowTouchSlop() { return 16; }
    public float getScaledHorizontalScrollFactor() { return 64; } public float getScaledVerticalScrollFactor() { return 64; } public int getScaledHoverSlop() { return 4; } public int getScaledMinimumFlingVelocity(int device,int axis,int source) { return 50; } public int getScaledMaximumFlingVelocity(int device,int axis,int source) { return 8000; } public boolean shouldShowMenuShortcutsWhenKeyboardPresent() { return false; }
    public boolean hasPermanentMenuKey() { return false; }
}
