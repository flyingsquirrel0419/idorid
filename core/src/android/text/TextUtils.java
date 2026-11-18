package android.text;
public class TextUtils {
    public static final android.os.Parcelable.Creator<CharSequence> CHAR_SEQUENCE_CREATOR=new android.os.Parcelable.Creator<CharSequence>() { public CharSequence createFromParcel(android.os.Parcel p){return p.readString();} public CharSequence[] newArray(int n){return new CharSequence[n];} };
    public static void writeToParcel(CharSequence value,android.os.Parcel p,int flags){p.writeString(value==null?null:value.toString());}
    public static String substring(CharSequence source,int start,int end){return source.subSequence(start,end).toString();}
    public static int getLayoutDirectionFromLocale(java.util.Locale locale){if(locale==null)return 0;String l=locale.getLanguage();return l.equals("ar")||l.equals("fa")||l.equals("he")||l.equals("ur")?1:0;}
    public static CharSequence ellipsize(CharSequence text,TextPaint paint,float available,TruncateAt where){if(paint.measureText(text,0,text.length())<=available)return text;String ellipsis="…";float remain=available-paint.measureText(ellipsis);if(remain<=0)return "";String s=text.toString();if(where==TruncateAt.START){int cut=s.length();while(cut>0&&paint.measureText(s,cut-1,s.length())<=remain)cut--;return ellipsis+s.substring(cut);}if(where==TruncateAt.MIDDLE){int left=0,right=s.length();float used=0;while(left<right){float a=paint.measureText(s,left,left+1);if(used+a>remain)break;used+=a;left++;if(left<right){float b=paint.measureText(s,right-1,right);if(used+b>remain)break;used+=b;right--;}}return s.substring(0,left)+ellipsis+s.substring(right);}int end=0;while(end<s.length()&&paint.measureText(s,0,end+1)<=remain)end++;return s.substring(0,end)+ellipsis;}
    public enum TruncateAt { START, MIDDLE, END, MARQUEE }
    public static boolean isEmpty(CharSequence s) { return s == null || s.length() == 0; }
    public static boolean equals(CharSequence a, CharSequence b) {
        if (a == b) return true;
        if (a == null || b == null || a.length() != b.length()) return false;
        return a.toString().equals(b.toString());
    }
    public static String join(CharSequence d, Iterable<?> parts) {
        StringBuilder sb = new StringBuilder(); boolean f = true;
        for (Object p : parts) { if (!f) sb.append(d); sb.append(p); f = false; }
        return sb.toString();
    }
    public static String join(CharSequence d, Object[] parts) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < parts.length; i++) { if (i > 0) sb.append(d); sb.append(parts[i]); }
        return sb.toString();
    }
    public static String[] split(String s, String re) { return s.length() == 0 ? new String[0] : s.split(re, -1); }
    public static boolean isDigitsOnly(CharSequence s) { for (int i = 0; i < s.length(); i++) if (!Character.isDigit(s.charAt(i))) return false; return true; }
    public static int getTrimmedLength(CharSequence s) { return s.toString().trim().length(); }
    public static CharSequence concat(CharSequence... t) { StringBuilder sb = new StringBuilder(); for (CharSequence c : t) sb.append(c); return sb.toString(); }
    public static String htmlEncode(String s) { return s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;"); }
}
