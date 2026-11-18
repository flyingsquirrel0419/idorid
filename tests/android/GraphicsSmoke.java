import android.graphics.*;
import android.graphics.drawable.ColorDrawable;
public class GraphicsSmoke {
 public static void main(String[] args) {
  Matrix m=new Matrix(); m.setTranslate(10,20); m.preScale(2,3); float[] p={1,2}; m.mapPoints(p);
  System.out.println("matrix="+(int)p[0]+","+(int)p[1]);
  Bitmap b=Bitmap.createBitmap(2,2,Bitmap.Config.ARGB_8888); b.eraseColor(Color.RED); b.setPixel(1,1,Color.BLUE);
  System.out.println("pixel="+Integer.toHexString(b.getPixel(1,1)));
  Rect r=new Rect(0,0,10,10); r.intersect(new Rect(5,4,12,12)); System.out.println("rect="+r.width()+"x"+r.height());
  Canvas c=new Canvas(b); ColorDrawable d=new ColorDrawable(Color.RED); d.setBounds(1,2,10,20); d.draw(c); System.out.println("drawable="+d.getBounds().width());
 }
}
