package android.window;
public final class BackEvent { private final float x,y,progress;private final int edge; public BackEvent(float x,float y,float progress,int edge){this.x=x;this.y=y;this.progress=progress;this.edge=edge;} public float getTouchX(){return x;} public float getTouchY(){return y;} public float getProgress(){return progress;} public int getSwipeEdge(){return edge;} }
