package android.view;
public class Gravity {
    public static final int NO_GRAVITY = 0, LEFT = 3, RIGHT = 5, TOP = 48, BOTTOM = 80, CENTER_VERTICAL = 16, CENTER_HORIZONTAL = 1,
        CENTER = 17, START = 0x00800003, END = 0x00800005, FILL = 0x77, FILL_HORIZONTAL = 7, FILL_VERTICAL = 0x70;
    /** idorid: compact host description such as "center" or "left|top". */
    public static String describe(int g) {
        if (g == 0 || g == -1) return null;
        StringBuilder sb = new StringBuilder();
        int h = g & 7, v = g & 0x70;
        if ((g & CENTER) == CENTER) return "center";
        if (h == 1) sb.append("center_horizontal"); else if (h == 5) sb.append("right"); else if (h == 3) sb.append("left");
        if (v == 0x10) sb.append(sb.length() > 0 ? "|" : "").append("center_vertical");
        else if (v == 0x50) sb.append(sb.length() > 0 ? "|" : "").append("bottom");
        else if (v == 0x30) sb.append(sb.length() > 0 ? "|" : "").append("top");
        return sb.length() == 0 ? null : sb.toString();
    }
    static int parse(String s) {
        int g = 0;
        for (String p : s.split("\\|")) {
            p = p.trim();
            if (p.equals("center")) g |= CENTER; else if (p.equals("center_horizontal")) g |= CENTER_HORIZONTAL;
            else if (p.equals("center_vertical")) g |= CENTER_VERTICAL; else if (p.equals("left") || p.equals("start")) g |= LEFT;
            else if (p.equals("right") || p.equals("end")) g |= RIGHT; else if (p.equals("top")) g |= TOP; else if (p.equals("bottom")) g |= BOTTOM;
            else if (p.equals("fill")) g |= FILL; else if (p.equals("fill_horizontal")) g |= FILL_HORIZONTAL;
        }
        return g;
    }
}
