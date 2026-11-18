package android.util;
public class TypedValue {
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
