package android.text;
import android.graphics.Paint;
public class TextPaint extends Paint { public int bgColor,baselineShift,linkColor; public int[] drawableState; public float density=1; public TextPaint(){} public TextPaint(int flags){super(flags);} public TextPaint(Paint p){super(p);} public void set(TextPaint p){super.set(p);bgColor=p.bgColor;baselineShift=p.baselineShift;linkColor=p.linkColor;density=p.density;drawableState=p.drawableState;} public String getFontVariationSettings(){return null;} }
