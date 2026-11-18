package android.view;
public final class VelocityTracker {
    private final long[] times = new long[20]; private final float[] xs = new float[20], ys = new float[20]; private int count; private float vx,vy;
    public static VelocityTracker obtain() { return new VelocityTracker(); }
    public void recycle() { clear(); }
    public void clear() { count=0; vx=vy=0; }
    public void addMovement(MotionEvent e) { if (count==20) { for (int i=1;i<count;i++) { times[i-1]=times[i]; xs[i-1]=xs[i]; ys[i-1]=ys[i]; } count--; } times[count]=e.getEventTime(); xs[count]=e.getX(); ys[count]=e.getY(); count++; }
    public void computeCurrentVelocity(int units) { computeCurrentVelocity(units, Float.MAX_VALUE); }
    public void computeCurrentVelocity(int units,float max) { vx=vy=0; if (count<2) return; int first=count-2; while (first>0 && times[count-1]-times[first-1]<=100) first--; long dt=times[count-1]-times[first]; if (dt<=0) return; vx=Math.max(-max,Math.min(max,(xs[count-1]-xs[first])*units/dt)); vy=Math.max(-max,Math.min(max,(ys[count-1]-ys[first])*units/dt)); }
    public boolean isAxisSupported(int axis) { return axis==0 || axis==1; } public float getAxisVelocity(int axis) { return axis==0 ? vx : axis==1 ? vy : 0; } public float getAxisVelocity(int axis,int id) { return getAxisVelocity(axis); }
    public float getXVelocity() { return vx; } public float getYVelocity() { return vy; }
    public float getXVelocity(int id) { return vx; } public float getYVelocity(int id) { return vy; }
}
