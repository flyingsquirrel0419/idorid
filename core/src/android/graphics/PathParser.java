package android.graphics;

/** Parser for Android/SVG path data. All output uses absolute path commands. */
public final class PathParser {
 private final String s; private int pos; private float x,y,sx,sy,cx,cy; private char previous;
 private PathParser(String data) { s=data; }
 public static Path createPathFromPathData(String data) {
  Path p=new Path(); if(data==null) return p;
  try { new PathParser(data).parse(p); } catch (RuntimeException ignored) { }
  return p;
 }
 private void space() { while(pos<s.length() && (Character.isWhitespace(s.charAt(pos)) || s.charAt(pos)==',')) pos++; }
 private boolean command(char c) { return "MmLlHhVvCcSsQqTtAaZz".indexOf(c)>=0; }
 private float number() {
  space(); int start=pos; if(pos<s.length() && (s.charAt(pos)=='-'||s.charAt(pos)=='+')) pos++;
  boolean digit=false; while(pos<s.length() && Character.isDigit(s.charAt(pos))) { pos++; digit=true; }
  if(pos<s.length() && s.charAt(pos)=='.') { pos++; while(pos<s.length() && Character.isDigit(s.charAt(pos))) { pos++; digit=true; } }
  if(!digit) throw new IllegalArgumentException();
  if(pos<s.length() && (s.charAt(pos)=='e'||s.charAt(pos)=='E')) { int e=pos++; if(pos<s.length() && (s.charAt(pos)=='+'||s.charAt(pos)=='-')) pos++; int d=pos; while(pos<s.length() && Character.isDigit(s.charAt(pos))) pos++; if(d==pos) pos=e; }
  return Float.parseFloat(s.substring(start,pos));
 }
 private float flag() { space(); if(pos>=s.length() || (s.charAt(pos)!='0' && s.charAt(pos)!='1')) throw new IllegalArgumentException(); return s.charAt(pos++)-'0'; }
 private void parse(Path p) {
  char cmd=0;
  while(true) {
   space(); if(pos>=s.length()) break;
   char ch=s.charAt(pos); if(command(ch)) { cmd=ch; pos++; } else if(cmd==0) throw new IllegalArgumentException();
   boolean rel=Character.isLowerCase(cmd); char u=Character.toUpperCase(cmd); float ox=x,oy=y,a,b,c,d,e,f;
   switch(u) {
    case 'M': a=number(); b=number(); x=rel?ox+a:a; y=rel?oy+b:b; p.moveTo(x,y); sx=x;sy=y; cmd=rel?'l':'L'; break;
    case 'L': a=number();b=number();x=rel?ox+a:a;y=rel?oy+b:b;p.lineTo(x,y);break;
    case 'H': a=number();x=rel?ox+a:a;p.lineTo(x,y);break;
    case 'V': a=number();y=rel?oy+a:a;p.lineTo(x,y);break;
    case 'C': a=number();b=number();c=number();d=number();e=number();f=number();cx=rel?ox+c:c;cy=rel?oy+d:d;x=rel?ox+e:e;y=rel?oy+f:f;p.cubicTo(rel?ox+a:a,rel?oy+b:b,cx,cy,x,y);break;
    case 'S': a=number();b=number();c=number();d=number(); e=(previous=='C'||previous=='S')?2*ox-cx:ox; f=(previous=='C'||previous=='S')?2*oy-cy:oy;cx=rel?ox+a:a;cy=rel?oy+b:b;x=rel?ox+c:c;y=rel?oy+d:d;p.cubicTo(e,f,cx,cy,x,y);break;
    case 'Q': a=number();b=number();c=number();d=number();cx=rel?ox+a:a;cy=rel?oy+b:b;x=rel?ox+c:c;y=rel?oy+d:d;p.quadTo(cx,cy,x,y);break;
    case 'T': a=number();b=number();cx=(previous=='Q'||previous=='T')?2*ox-cx:ox;cy=(previous=='Q'||previous=='T')?2*oy-cy:oy;x=rel?ox+a:a;y=rel?oy+b:b;p.quadTo(cx,cy,x,y);break;
    case 'A': a=number();b=number();c=number();d=flag();e=flag();float ex=number(),ey=number();x=rel?ox+ex:ex;y=rel?oy+ey:ey;arc(p,ox,oy,x,y,a,b,c,d!=0,e!=0);break;
    case 'Z': p.close();x=sx;y=sy;cmd=0;break;
    default: throw new IllegalArgumentException();
   }
   previous=u;
  }
 }
 private static void arc(Path p,double x0,double y0,double x1,double y1,double rx,double ry,double angle,boolean large,boolean sweep) {
  rx=Math.abs(rx);ry=Math.abs(ry);if(x0==x1 && y0==y1)return;if(rx==0||ry==0){p.lineTo((float)x1,(float)y1);return;}
  double phi=angle*Math.PI/180, co=Math.cos(phi),si=Math.sin(phi), dx=(x0-x1)/2,dy=(y0-y1)/2;
  double xp=co*dx+si*dy,yp=-si*dx+co*dy, k=xp*xp/(rx*rx)+yp*yp/(ry*ry);
  if(k>1){k=Math.sqrt(k);rx*=k;ry*=k;}
  double rx2=rx*rx,ry2=ry*ry, denom=rx2*yp*yp+ry2*xp*xp;
  double factor=(large==sweep?-1:1)*Math.sqrt(Math.max(0,(rx2*ry2-denom)/denom));
  double cpx=factor*rx*yp/ry,cpy=-factor*ry*xp/rx;
  double centerX=co*cpx-si*cpy+(x0+x1)/2,centerY=si*cpx+co*cpy+(y0+y1)/2;
  double ux=(xp-cpx)/rx,uy=(yp-cpy)/ry,vx=(-xp-cpx)/rx,vy=(-yp-cpy)/ry;
  double start=Math.atan2(uy,ux),delta=Math.atan2(ux*vy-uy*vx,ux*vx+uy*vy);
  if(sweep && delta<0)delta+=2*Math.PI;else if(!sweep && delta>0)delta-=2*Math.PI;
  int n=(int)Math.ceil(Math.abs(delta)/(Math.PI/2));double step=delta/n;
  for(int i=0;i<n;i++) { double t=start+i*step,t2=t+step,ct=Math.cos(t),st=Math.sin(t),ct2=Math.cos(t2),st2=Math.sin(t2),q=4.0/3*Math.tan(step/4);
   double ax=centerX+rx*co*ct-ry*si*st,ay=centerY+rx*si*ct+ry*co*st;
   double bx=centerX+rx*co*ct2-ry*si*st2,by=centerY+rx*si*ct2+ry*co*st2;
   double dax=-rx*co*st-ry*si*ct,day=-rx*si*st+ry*co*ct,dbx=-rx*co*st2-ry*si*ct2,dby=-rx*si*st2+ry*co*ct2;
   if(i==n-1){bx=x1;by=y1;} p.cubicTo((float)(ax+q*dax),(float)(ay+q*day),(float)(bx-q*dbx),(float)(by-q*dby),(float)bx,(float)by);
  }
 }
}
