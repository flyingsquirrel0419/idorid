package android.util;
public class TypedValue {
    public static final int TYPE_NULL=0, TYPE_REFERENCE=1, TYPE_ATTRIBUTE=2, TYPE_STRING=3, TYPE_FLOAT=4, TYPE_DIMENSION=5, TYPE_FRACTION=6, TYPE_FIRST_INT=16, TYPE_INT_DEC=16, TYPE_INT_HEX=17, TYPE_INT_BOOLEAN=18, TYPE_FIRST_COLOR_INT=28, TYPE_INT_COLOR_ARGB8=28, TYPE_INT_COLOR_RGB8=29, TYPE_INT_COLOR_ARGB4=30, TYPE_INT_COLOR_RGB4=31, TYPE_LAST_COLOR_INT=31, TYPE_LAST_INT=31;
    public static final int DATA_NULL_UNDEFINED=0, DATA_NULL_EMPTY=1, COMPLEX_UNIT_FRACTION=0, COMPLEX_UNIT_FRACTION_PARENT=1;
    public int type, data, assetCookie, resourceId, changingConfigurations, density;
    public CharSequence string;
    public float getFloat() { return Float.intBitsToFloat(data); }
    public float getDimension(DisplayMetrics metrics) { return complexToDimension(data,metrics); }
    public float getFraction(float base,float parentBase) { return complexToFraction(data,base,parentBase); }
    public static float complexToFloat(int complex) { float[] mult={1.0f/256,1.0f/32768,1.0f/8388608,1.0f/2147483648.0f}; return (complex & 0xffffff00)*mult[(complex >> 4)&3]; }
    public static float complexToDimension(int data,DisplayMetrics metrics) { return applyDimension(data & 15,complexToFloat(data),metrics); }
    public static int complexToDimensionPixelOffset(int data,DisplayMetrics metrics) { return (int)complexToDimension(data,metrics); }
    public static int complexToDimensionPixelSize(int data,DisplayMetrics metrics) { float v=complexToDimension(data,metrics); int n=(int)(v+(v>=0 ? 0.5f:-0.5f)); return n!=0 ? n : v==0 ? 0 : v>0 ? 1:-1; }
    public static float complexToFraction(int data,float base,float parentBase) { float v=complexToFloat(data); return (data & 15)==COMPLEX_UNIT_FRACTION ? v*base : (data & 15)==COMPLEX_UNIT_FRACTION_PARENT ? v*parentBase : 0; }
    public void setTo(TypedValue other) { type=other.type; data=other.data; assetCookie=other.assetCookie; resourceId=other.resourceId; changingConfigurations=other.changingConfigurations; density=other.density; string=other.string; }
    public CharSequence coerceToString() { return type==TYPE_STRING ? string : type==TYPE_NULL ? null : String.valueOf(data); }
    public static final int COMPLEX_UNIT_PX = 0, COMPLEX_UNIT_DIP = 1, COMPLEX_UNIT_SP = 2, COMPLEX_UNIT_PT = 3, COMPLEX_UNIT_IN = 4, COMPLEX_UNIT_MM = 5;
    public static float applyDimension(int unit, float value, DisplayMetrics m) {
        switch (unit) {
            case COMPLEX_UNIT_PX: return value;
            case COMPLEX_UNIT_DIP: return value * m.density;
            case COMPLEX_UNIT_SP: return value * m.scaledDensity;
            case COMPLEX_UNIT_PT: return value * m.xdpi / 72f;
            case COMPLEX_UNIT_IN: return value * m.xdpi;
            case COMPLEX_UNIT_MM: return value * m.xdpi / 25.4f;
        }
        return 0;
    }
}
