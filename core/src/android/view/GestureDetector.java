package android.view;
import android.content.Context;
import android.os.Handler;
public class GestureDetector {
    public interface OnGestureListener { boolean onDown(MotionEvent e); void onShowPress(MotionEvent e); boolean onSingleTapUp(MotionEvent e); boolean onScroll(MotionEvent e1,MotionEvent e2,float dx,float dy); void onLongPress(MotionEvent e); boolean onFling(MotionEvent e1,MotionEvent e2,float vx,float vy); }
    public interface OnDoubleTapListener { boolean onSingleTapConfirmed(MotionEvent e); boolean onDoubleTap(MotionEvent e); boolean onDoubleTapEvent(MotionEvent e); }
    public static class SimpleOnGestureListener implements OnGestureListener,OnDoubleTapListener {
        public boolean onDown(MotionEvent e) { return false; } public void onShowPress(MotionEvent e) {} public boolean onSingleTapUp(MotionEvent e) { return false; } public boolean onScroll(MotionEvent a,MotionEvent b,float x,float y) { return false; } public void onLongPress(MotionEvent e) {} public boolean onFling(MotionEvent a,MotionEvent b,float x,float y) { return false; }
        public boolean onSingleTapConfirmed(MotionEvent e) { return false; } public boolean onDoubleTap(MotionEvent e) { return false; } public boolean onDoubleTapEvent(MotionEvent e) { return false; }
    }
    private final OnGestureListener listener; private OnDoubleTapListener doubleTap; private final Handler handler; private MotionEvent down, last, previousUp; private VelocityTracker velocity; private boolean moved,longPressed,doubleTapping,longPressEnabled=true;
    private final Runnable longPress = new Runnable() { public void run() { if (down!=null && !moved && longPressEnabled) { longPressed=true; listener.onLongPress(down); } } };
    private final Runnable showPress = new Runnable() { public void run() { if (down!=null && !moved) listener.onShowPress(down); } };
    private final Runnable confirmTap = new Runnable() { public void run() { if (previousUp!=null && doubleTap!=null) { doubleTap.onSingleTapConfirmed(previousUp); previousUp=null; } } };
    public GestureDetector(Context c,OnGestureListener l) { this(c,l,null); }
    public GestureDetector(Context c,OnGestureListener l,Handler h) { if (l==null) throw new NullPointerException(); listener=l; handler=h==null?new Handler():h; if (l instanceof OnDoubleTapListener) doubleTap=(OnDoubleTapListener)l; }
    public GestureDetector(OnGestureListener l) { this(null,l,null); }
    public void setOnDoubleTapListener(OnDoubleTapListener l) { doubleTap=l; }
    public void setIsLongpressEnabled(boolean enabled) { longPressEnabled=enabled; }
    public boolean isLongpressEnabled() { return longPressEnabled; }
    public boolean onTouchEvent(MotionEvent e) {
        int action=e.getActionMasked(); boolean handled=false;
        if (action==MotionEvent.ACTION_DOWN) {
            doubleTapping=false; if (previousUp!=null && doubleTap!=null && e.getEventTime()-previousUp.getEventTime()<=ViewConfiguration.getDoubleTapTimeout()) { float dx=e.getX()-previousUp.getX(),dy=e.getY()-previousUp.getY(); if (dx*dx+dy*dy<=10000) { handler.removeCallbacks(confirmTap); handled=doubleTap.onDoubleTap(previousUp); doubleTapping=true; previousUp=null; } }
            down=MotionEvent.obtain(e); last=MotionEvent.obtain(e); moved=longPressed=false; velocity=VelocityTracker.obtain(); velocity.addMovement(e); handler.postDelayed(showPress,ViewConfiguration.getTapTimeout()); if (longPressEnabled) handler.postDelayed(longPress,ViewConfiguration.getLongPressTimeout()); return listener.onDown(e) || handled;
        }
        if (down==null) return false; velocity.addMovement(e);
        if (doubleTapping && doubleTap!=null) handled=doubleTap.onDoubleTapEvent(e);
        if (action==MotionEvent.ACTION_MOVE && !longPressed) { float x=e.getX()-down.getX(), y=e.getY()-down.getY(); if (!moved && x*x+y*y>64) { moved=true; handler.removeCallbacks(longPress); handler.removeCallbacks(showPress); } if (moved) handled=listener.onScroll(down,e,last.getX()-e.getX(),last.getY()-e.getY()) || handled; last=MotionEvent.obtain(e); }
        if (action==MotionEvent.ACTION_UP) { handler.removeCallbacks(longPress); handler.removeCallbacks(showPress); if (!moved && !longPressed) { handled=listener.onSingleTapUp(e) || handled; if (!doubleTapping && doubleTap!=null) { previousUp=MotionEvent.obtain(e); handler.postDelayed(confirmTap,ViewConfiguration.getDoubleTapTimeout()); } } else if (moved && !longPressed) { velocity.computeCurrentVelocity(1000,8000); float x=velocity.getXVelocity(),y=velocity.getYVelocity(); if (Math.abs(x)>=50 || Math.abs(y)>=50) handled=listener.onFling(down,e,x,y) || handled; } down=null; velocity.recycle(); }
        if (action==MotionEvent.ACTION_CANCEL) { handler.removeCallbacks(longPress); handler.removeCallbacks(showPress); handler.removeCallbacks(confirmTap); down=null; previousUp=null; velocity.recycle(); }
        return handled;
    }
}
