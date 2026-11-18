package android.text;
import android.view.VMText;
public abstract class Layout {
    public enum Alignment { ALIGN_NORMAL, ALIGN_OPPOSITE, ALIGN_CENTER }
    protected final CharSequence text; protected final TextPaint paint; protected final int width; protected final Alignment alignment;
    protected Layout(CharSequence text,TextPaint paint,int width,Alignment alignment,float spacingMult,float spacingAdd){this.text=text;this.paint=paint;this.width=width;this.alignment=alignment;}
    public CharSequence getText(){return text;} public TextPaint getPaint(){return paint;} public int getWidth(){return width;} public Alignment getAlignment(){return alignment;}
    public int getHeight(){return VMText.heightOf(VMText.measure(text.toString(),paint.getTextSize(),0,width));}
    public abstract int getLineCount(); public abstract int getLineStart(int line); public abstract int getLineTop(int line);
    public int getLineEnd(int line){return line+1<getLineCount()?getLineStart(line+1):text.length();} public int getLineBottom(int line){return getLineTop(line+1);} public int getLineForOffset(int offset){for(int i=1;i<getLineCount();i++)if(getLineStart(i)>offset)return i-1;return getLineCount()-1;}
    public void draw(android.graphics.Canvas canvas) {} public int getEllipsisCount(int line){return 0;} public int getLineBaseline(int line){return getLineBottom(line)-(int)(paint.getTextSize()*0.25f);}
    public float getLineWidth(int line){return paint.measureText(text,getLineStart(line),getLineEnd(line));}
    public static float getDesiredWidth(CharSequence text,TextPaint paint){return paint.measureText(text,0,text.length());}
}
