package android.view;
public final class MotionEvent {
    public static final int ACTION_DOWN = 0, ACTION_UP = 1, ACTION_MOVE = 2, ACTION_CANCEL = 3;
    private int action; private float x, y; private long downTime,eventTime;
    public static MotionEvent obtain(long dt, long et, int action, float x, float y, int meta) { MotionEvent e = new MotionEvent(); e.downTime=dt; e.eventTime=et; e.action = action; e.x = x; e.y = y; return e; }
    public static MotionEvent obtain(long dt,long et,int action,float x,float y,float pressure,float size,int meta,float xPrecision,float yPrecision,int deviceId,int edgeFlags) { return obtain(dt,et,action,x,y,meta); }
    public static MotionEvent obtain(MotionEvent e) { return obtain(e.downTime,e.eventTime,e.action,e.x,e.y,0); }
    public int getPointerCount() { return 1; }
    public int getPointerId(int index) { return 0; }
    public float getX(int index) { return x; } public float getY(int index) { return y; }
    public long getEventTime() { return eventTime; } public long getDownTime() { return downTime; }
    public int getAction() { return action; }
    public int getActionMasked() { return action; }
    public float getX() { return x; }
    public float getY() { return y; }
    public float getRawX() { return x; }
    public float getRawY() { return y; }
    public void recycle() {}
}
