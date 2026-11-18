package android.text;
public final class TextDirectionHeuristics {
    private static final class Direction implements TextDirectionHeuristic { private final boolean rtl; Direction(boolean rtl){this.rtl=rtl;} public boolean isRtl(CharSequence text,int start,int count){return rtl;} public boolean isRtl(char[] text,int start,int count){return rtl;} }
    public static final TextDirectionHeuristic LTR=new Direction(false), RTL=new Direction(true), FIRSTSTRONG_LTR=LTR, FIRSTSTRONG_RTL=RTL, ANYRTL_LTR=LTR, LOCALE=LTR;
}
