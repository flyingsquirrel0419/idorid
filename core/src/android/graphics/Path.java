package android.graphics;
public class Path {
 public enum FillType { WINDING, EVEN_ODD, INVERSE_WINDING, INVERSE_EVEN_ODD } public enum Direction { CW, CCW }
 private StringBuilder s=new StringBuilder(); private float x,y,startX,startY; private FillType fill=FillType.WINDING;
 private void cmd(String c,float... a) { if(s.length()>0) s.append(' '); s.append(c); for(float v:a) { s.append(' '); android.view.Json.num(s,v); } }
 public void moveTo(float x,float y) { cmd("M",x,y); this.x=startX=x; this.y=startY=y; }
 public void lineTo(float x,float y) { cmd("L",x,y); this.x=x; this.y=y; }
 public void quadTo(float a,float b,float x,float y) { cmd("Q",a,b,x,y); this.x=x; this.y=y; }
 public void cubicTo(float a,float b,float c,float d,float x,float y) { cmd("C",a,b,c,d,x,y); this.x=x; this.y=y; }
 public void close() { cmd("Z"); x=startX; y=startY; }
 public void rMoveTo(float dx,float dy) { moveTo(x+dx,y+dy); } public void rLineTo(float dx,float dy) { lineTo(x+dx,y+dy); }
 public void reset() { s=new StringBuilder(); x=y=startX=startY=0; fill=FillType.WINDING; }
 public void setFillType(FillType f) { fill=f; } public FillType getFillType() { return fill; }
 public String idoridPath() { return s.toString(); }
 public void addRect(float l,float t,float r,float b,Direction d) { moveTo(l,t); if(d==Direction.CW) { lineTo(r,t); lineTo(r,b); lineTo(l,b); } else { lineTo(l,b); lineTo(r,b); lineTo(r,t); } close(); }
 public void addRect(RectF r,Direction d) { addRect(r.left,r.top,r.right,r.bottom,d); }
 public void addCircle(float x,float y,float r,Direction d) { addOval(x-r,y-r,x+r,y+r,d); }
 public void addOval(RectF r,Direction d) { addOval(r.left,r.top,r.right,r.bottom,d); }
 public void addOval(float l,float t,float r,float b,Direction d) { float x=(l+r)/2,y=(t+b)/2,rx=(r-l)/2,ry=(b-t)/2,k=.55228475f; moveTo(x+rx,y); float sign=d==Direction.CW?1:-1; cubicTo(x+rx,y+sign*k*ry,x+k*rx,y+sign*ry,x,y+sign*ry); cubicTo(x-k*rx,y+sign*ry,x-rx,y+sign*k*ry,x-rx,y); cubicTo(x-rx,y-sign*k*ry,x-k*rx,y-sign*ry,x,y-sign*ry); cubicTo(x+k*rx,y-sign*ry,x+rx,y-sign*k*ry,x+rx,y); close(); }
 public void addRoundRect(RectF r,float rx,float ry,Direction d) { addRoundRect(r.left,r.top,r.right,r.bottom,rx,ry,d); }
 public void addRoundRect(float l,float t,float r,float b,float rx,float ry,Direction d) { rx=Math.min(Math.abs(rx),(r-l)/2); ry=Math.min(Math.abs(ry),(b-t)/2); if(d==Direction.CCW) { moveTo(l+rx,t); quadTo(l,t,l,t+ry); lineTo(l,b-ry); quadTo(l,b,l+rx,b); lineTo(r-rx,b); quadTo(r,b,r,b-ry); lineTo(r,t+ry); quadTo(r,t,r-rx,t); lineTo(l+rx,t); close(); return; } moveTo(l+rx,t); lineTo(r-rx,t); quadTo(r,t,r,t+ry); lineTo(r,b-ry); quadTo(r,b,r-rx,b); lineTo(l+rx,b); quadTo(l,b,l,b-ry); lineTo(l,t+ry); quadTo(l,t,l+rx,t); close(); }
}
