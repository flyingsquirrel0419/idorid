package android.view.animation;
import android.graphics.Path;
import android.graphics.PathMeasure;
public class PathInterpolator extends BaseInterpolator {
    private final float[] xs=new float[201],ys=new float[201];
    public PathInterpolator(float x,float y){for(int i=0;i<xs.length;i++){float t=i/200f,u=1-t;xs[i]=2*u*t*x+t*t;ys[i]=2*u*t*y+t*t;}}
    public PathInterpolator(float x1,float y1,float x2,float y2){for(int i=0;i<xs.length;i++){float t=i/200f,u=1-t;xs[i]=3*u*u*t*x1+3*u*t*t*x2+t*t*t;ys[i]=3*u*u*t*y1+3*u*t*t*y2+t*t*t;}}
    public PathInterpolator(Path path){PathMeasure m=new PathMeasure(path,false);float[] p=new float[2];for(int i=0;i<xs.length;i++){m.getPosTan(m.getLength()*i/200f,p,null);xs[i]=p[0];ys[i]=p[1];}}
    public float getInterpolation(float input){if(input<=0)return 0;if(input>=1)return 1;int lo=0,hi=xs.length-1;while(hi-lo>1){int mid=(lo+hi)/2;if(xs[mid]<input)lo=mid;else hi=mid;}float d=xs[hi]-xs[lo];return d==0?ys[lo]:ys[lo]+(input-xs[lo])*(ys[hi]-ys[lo])/d;}
}
