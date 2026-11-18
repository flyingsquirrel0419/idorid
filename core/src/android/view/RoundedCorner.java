package android.view;
import android.graphics.Point;
public final class RoundedCorner { private final int position,radius,x,y; public RoundedCorner(int position,int radius,int x,int y){this.position=position;this.radius=radius;this.x=x;this.y=y;} public int getPosition(){return position;} public int getRadius(){return radius;} public Point getCenter(){return new Point(x,y);} }
