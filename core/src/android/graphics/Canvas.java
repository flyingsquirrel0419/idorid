package android.graphics;
import android.view.Json;
public class Canvas {
 private final int width,height; private final StringBuilder ops=new StringBuilder(); private int saves=1;
 public Canvas() { this(0,0); } public Canvas(int w,int h) { width=w; height=h; }
 public int getWidth() { return width; } public int getHeight() { return height; } public String idoridOps() { return "["+ops.toString()+"]"; }
 private void begin(String op) { if(ops.length()>0) ops.append(','); ops.append('['); Json.str(ops,op); }
 private void num(float v) { ops.append(','); Json.num(ops,v); } private void str(String s) { ops.append(','); Json.str(ops,s); } private void end() { ops.append(']'); }
 private void paint(Paint p) { str(Color.toHex(p.getColor())); num(p.getStyle().ordinal()); num(p.getStrokeWidth()); }
 public int save() { begin("save"); end(); return saves++; } public void restore() { if(saves>1) { saves--; begin("restore"); end(); } }
 public void restoreToCount(int count) { while(saves>count) restore(); }
 public void translate(float x,float y) { begin("translate"); num(x); num(y); end(); }
 public void scale(float x,float y) { begin("scale"); num(x); num(y); end(); } public void scale(float x,float y,float px,float py) { translate(px,py); scale(x,y); translate(-px,-py); }
 public void rotate(float d) { begin("rotate"); num(d); end(); } public void rotate(float d,float x,float y) { translate(x,y); rotate(d); translate(-x,-y); }
 public boolean clipRect(float l,float t,float r,float b) { begin("clip"); num(l); num(t); num(r); num(b); end(); return true; } public boolean clipRect(RectF r) { return clipRect(r.left,r.top,r.right,r.bottom); }
 public void drawColor(int c) { begin("color"); str(Color.toHex(c)); end(); } public void drawRGB(int r,int g,int b) { drawColor(Color.rgb(r,g,b)); } public void drawARGB(int a,int r,int g,int b) { drawColor(Color.argb(a,r,g,b)); }
 public void drawRect(float l,float t,float r,float b,Paint p) { begin("rect"); num(l); num(t); num(r); num(b); paint(p); end(); } public void drawRect(RectF r,Paint p) { drawRect(r.left,r.top,r.right,r.bottom,p); } public void drawRect(Rect r,Paint p) { drawRect(r.left,r.top,r.right,r.bottom,p); }
 public void drawRoundRect(float l,float t,float r,float b,float rx,float ry,Paint p) { begin("rrect"); num(l); num(t); num(r); num(b); num(rx); num(ry); paint(p); end(); } public void drawRoundRect(RectF r,float rx,float ry,Paint p) { drawRoundRect(r.left,r.top,r.right,r.bottom,rx,ry,p); }
 public void drawOval(float l,float t,float r,float b,Paint p) { begin("oval"); num(l); num(t); num(r); num(b); paint(p); end(); } public void drawOval(RectF r,Paint p) { drawOval(r.left,r.top,r.right,r.bottom,p); } public void drawCircle(float x,float y,float r,Paint p) { drawOval(x-r,y-r,x+r,y+r,p); }
 public void drawArc(float l,float t,float r,float b,float start,float sweep,boolean center,Paint p) { begin("arc"); num(l); num(t); num(r); num(b); num(start); num(sweep); ops.append(center?",true":",false"); paint(p); end(); } public void drawArc(RectF r,float start,float sweep,boolean center,Paint p) { drawArc(r.left,r.top,r.right,r.bottom,start,sweep,center,p); }
 public void drawLine(float x,float y,float x2,float y2,Paint p) { begin("line"); num(x); num(y); num(x2); num(y2); str(Color.toHex(p.getColor())); num(p.getStrokeWidth()); num(p.getStrokeCap().ordinal()); end(); }
 public void drawLines(float[] a,Paint p) { drawLines(a,0,a.length,p); } public void drawLines(float[] a,int offset,int count,Paint p) { for(int i=offset;i+3<offset+count;i+=4) drawLine(a[i],a[i+1],a[i+2],a[i+3],p); }
 public void drawPath(Path path,Paint p) { begin("path"); str(path.idoridPath()); paint(p); num(path.getFillType().ordinal()%2); end(); }
 public void idoridDrawBitmap(int resId,float l,float t,float r,float b,int alpha) { begin("bitmap"); ops.append(',').append(resId); num(l); num(t); num(r); num(b); num(alpha); end(); }
 public void drawText(String s,float x,float y,Paint p) { begin("text"); str(s); num(x); num(y); str(Color.toHex(p.getColor())); num(p.getTextSize()); num(p.getTextAlign().ordinal()); ops.append(p.getTypeface()!=null && p.getTypeface().isBold()?",true":",false"); end(); }
}
