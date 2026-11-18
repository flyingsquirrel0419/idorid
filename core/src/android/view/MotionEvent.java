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
    public int getActionMasked() { return action & 255; }
    public int getActionIndex() { return (action >> 8) & 255; }
    public int findPointerIndex(int id) { return id == 0 ? 0 : -1; }
    public int getDeviceId() { return 0; }
    public int getSource() { return 0; }
    public float getAxisValue(int axis) { return axis == 0 ? x : axis == 1 ? y : 0; }
    public void setAction(int a) { action=a; }
    public void setLocation(float x,float y) { this.x=x; this.y=y; }
    public void offsetLocation(float dx,float dy) { x+=dx; y+=dy; }
    public static MotionEvent obtainNoHistory(MotionEvent e) { return obtain(e); }
    public float getX() { return x; }
    public float getY() { return y; }
    public float getRawX() { return x; }
    public float getRawY() { return y; }
    public void recycle() {}
}
