package android.widget;
import android.content.Context;
import android.util.AttributeSet;
import android.view.Json;
import android.view.View;
import android.view.XmlAttrs;
public class ImageView extends View {
    public enum ScaleType { MATRIX, FIT_XY, FIT_START, FIT_CENTER, FIT_END, CENTER, CENTER_CROP, CENTER_INSIDE }
    private int src;
    private android.graphics.drawable.Drawable image; private android.content.res.ColorStateList imageTint; private android.graphics.PorterDuff.Mode tintMode; private android.graphics.Matrix matrix; private int imageAlpha=255, baseline=-1; private boolean cropToPadding;
    private int tint;
    private ScaleType scale = ScaleType.FIT_CENTER;
    public ImageView(Context c) { super(c); }
    public ImageView(Context c, AttributeSet a) { this(c, a, 0); }
    public ImageView(Context c, AttributeSet a, int s) {
        super(c, a, s);
        if (a instanceof XmlAttrs) {
            XmlAttrs x = (XmlAttrs) a;
            src = x.integer("src", x.integer("srcCompat", 0));
            adjustViewBounds = x.bool("adjustViewBounds", false);
            maxW = (int) x.dimension("maxWidth", Integer.MAX_VALUE);
            maxH = (int) x.dimension("maxHeight", Integer.MAX_VALUE);
        }
    }
    private boolean adjustViewBounds;
    private int maxW = Integer.MAX_VALUE, maxH = Integer.MAX_VALUE;
    public void setImageResource(int id) { if (id != src) { src = id; requestLayout(); } }
    public void setImageDrawable(android.graphics.drawable.Drawable d) { image=d; setImageResource(d == null ? 0 : d.idoridResource()); }
    public void setMaxWidth(int w) { maxW = w; requestLayout(); }
    public void setMaxHeight(int h) { maxH = h; requestLayout(); }
    public int getMaxWidth() { return maxW; }
    public int getMaxHeight() { return maxH; }
    public boolean getAdjustViewBounds() { return adjustViewBounds; }
    protected void onMeasure(int ws, int hs) {
        long size = src == 0 ? 0 : VMImage.size(src);
        int dw = (int) size, dh = (int) (size >>> 32);
        int padH = getPaddingLeft() + getPaddingRight(), padV = getPaddingTop() + getPaddingBottom();
        int wMode = MeasureSpec.getMode(ws), hMode = MeasureSpec.getMode(hs);
        int w = Math.max(dw + padH, getSuggestedMinimumWidth()), h = Math.max(dh + padV, getSuggestedMinimumHeight());
        w = Math.min(w, maxW); h = Math.min(h, maxH);
        int width = resolveSizeAndState(w, ws, 0) & MEASURED_SIZE_MASK;
        int height = resolveSizeAndState(h, hs, 0) & MEASURED_SIZE_MASK;
        if (adjustViewBounds && dw > 0 && dh > 0) {
            float ratio = (float) dw / dh;
            if (wMode == MeasureSpec.EXACTLY && hMode != MeasureSpec.EXACTLY) height = resolveSize(Math.min(maxH, (int) ((width - padH) / ratio) + padV), hs);
            else if (hMode == MeasureSpec.EXACTLY && wMode != MeasureSpec.EXACTLY) width = resolveSize(Math.min(maxW, (int) ((height - padV) * ratio) + padH), ws);
        }
        setMeasuredDimension(width, height);
    }
    public void setImageBitmap(android.graphics.Bitmap b) {}
    public void setScaleType(ScaleType s) { scale = s; }
    public ScaleType getScaleType() { return scale; }
    public void setColorFilter(int c) { tint = c; }
    public void setImageTintList(android.content.res.ColorStateList c) { imageTint=c; tint = c == null ? 0 : c.getDefaultColor(); }
    public void setAdjustViewBounds(boolean b) { adjustViewBounds = b; requestLayout(); }
    protected String idoridType() { return "ImageView"; }
    protected void idoridProps(StringBuilder sb) {
        if (src != 0) {
            Json.key(sb, "src"); sb.append(src);
            String n = null;
            try { n = getResources().getResourceName(src); } catch (RuntimeException e) {}
            if (n != null) { Json.key(sb, "srcName"); Json.str(sb, n); }
        }
        if (tint != 0) { Json.key(sb, "tint"); Json.str(sb, android.graphics.Color.toHex(tint)); }
    }
    public ImageView(Context c,AttributeSet a,int s,int r) { this(c,a,s); }
    public android.graphics.drawable.Drawable getDrawable() { return image; } public android.content.res.ColorStateList getImageTintList() { return imageTint; } public void setImageTintMode(android.graphics.PorterDuff.Mode m) { tintMode=m; } public android.graphics.PorterDuff.Mode getImageTintMode() { return tintMode; }
    public void setImageMatrix(android.graphics.Matrix m) { matrix=m; } public android.graphics.Matrix getImageMatrix() { return matrix; } public void setImageState(int[] state,boolean merge) {} public void setImageLevel(int level) { if(image!=null) image.setLevel(level); } public void setImageURI(android.net.Uri uri) {} public void setImageAlpha(int a) { imageAlpha=a; } public int getImageAlpha() { return imageAlpha; } public void setCropToPadding(boolean b) { cropToPadding=b; } public boolean getCropToPadding() { return cropToPadding; } public void setBaseline(int b) { baseline=b; } public int getBaseline() { return baseline; }

 public boolean hasOverlappingRendering() { return true; }
}
