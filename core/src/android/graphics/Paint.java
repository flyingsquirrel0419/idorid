package android.graphics;
public class Paint {
 public static final int ANTI_ALIAS_FLAG=1;
 public enum Style { FILL, STROKE, FILL_AND_STROKE } public enum Align { LEFT, CENTER, RIGHT } public enum Cap { BUTT, ROUND, SQUARE }
 private int color=0xff000000; private Style style=Style.FILL; private Align align=Align.LEFT; private Cap cap=Cap.BUTT;
 private float strokeWidth=0, textSize=16; private boolean aa; private Typeface typeface;
 public Paint() {} public Paint(int flags) { aa=(flags & ANTI_ALIAS_FLAG)!=0; }
 public void setColor(int c) { color=c; } public int getColor() { return color; }
 public void setARGB(int a,int r,int g,int b) { color=Color.argb(a,r,g,b); }
 public void setAlpha(int a) { color=(color & 0xffffff) | ((a & 255)<<24); } public int getAlpha() { return color >>> 24; }
 public void setStyle(Style s) { style=s; } public Style getStyle() { return style; }
 public void setStrokeWidth(float w) { strokeWidth=w; } public float getStrokeWidth() { return strokeWidth; }
 public void setStrokeCap(Cap c) { cap=c; } public Cap getStrokeCap() { return cap; }
 public void setTextSize(float s) { textSize=s; } public float getTextSize() { return textSize; }
 public void setTextAlign(Align a) { align=a; } public Align getTextAlign() { return align; }
 public void setAntiAlias(boolean a) { aa=a; } public boolean isAntiAlias() { return aa; }
 public Typeface setTypeface(Typeface t) { typeface=t; return t; } public Typeface getTypeface() { return typeface; }
 public float measureText(String s) { return measureText(s,0,s.length()); }
 public float measureText(String s,int start,int end) { float n=0; for(int i=start;i<end;i++) { char c=s.charAt(i); n+= c==' ' ? .28f : c>='0' && c<='9' ? .56f : c=='i' || c=='l' || c=='I' || c=='!' || c=='.' || c==',' ? .3f : .55f; } return n*textSize; }
 public void getTextBounds(String s,int start,int end,Rect r) { r.set(0,(int)(-textSize*.8f),(int)measureText(s,start,end),(int)(textSize*.2f)); }
 public static class FontMetrics { public float top,ascent,descent,bottom,leading; }
 public FontMetrics getFontMetrics() { FontMetrics f=new FontMetrics(); getFontMetrics(f); return f; }
 public float getFontMetrics(FontMetrics f) { f.top=-textSize; f.ascent=-.8f*textSize; f.descent=.2f*textSize; f.bottom=.25f*textSize; f.leading=0; return f.descent-f.ascent; }
 public float ascent() { return -.8f*textSize; } public float descent() { return .2f*textSize; }
}
