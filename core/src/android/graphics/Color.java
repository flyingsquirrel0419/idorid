package android.graphics;
public class Color {
    public static final int BLACK = 0xFF000000, DKGRAY = 0xFF444444, GRAY = 0xFF888888, LTGRAY = 0xFFCCCCCC, WHITE = 0xFFFFFFFF,
        RED = 0xFFFF0000, GREEN = 0xFF00FF00, BLUE = 0xFF0000FF, YELLOW = 0xFFFFFF00, CYAN = 0xFF00FFFF, MAGENTA = 0xFFFF00FF, TRANSPARENT = 0;
    public static int rgb(int r, int g, int b) { return 0xff000000 | (r << 16) | (g << 8) | b; }
    public static int argb(int a, int r, int g, int b) { return (a << 24) | (r << 16) | (g << 8) | b; }
    public static int alpha(int c) { return c >>> 24; }
    public static int red(int c) { return (c >> 16) & 0xff; }
    public static int green(int c) { return (c >> 8) & 0xff; }
    public static int blue(int c) { return c & 0xff; }
    public static int parseColor(String s) {
        if (s.charAt(0) == '#') {
            long c = Long.parseLong(s.substring(1), 16);
            if (s.length() == 7) c |= 0xff000000L;
            else if (s.length() != 9) throw new IllegalArgumentException("Unknown color");
            return (int) c;
        }
        String n = s.toLowerCase();
        if (n.equals("red")) return RED; if (n.equals("blue")) return BLUE; if (n.equals("green")) return GREEN;
        if (n.equals("black")) return BLACK; if (n.equals("white")) return WHITE; if (n.equals("gray") || n.equals("grey")) return GRAY;
        if (n.equals("yellow")) return YELLOW; if (n.equals("cyan")) return CYAN; if (n.equals("magenta")) return MAGENTA;
        throw new IllegalArgumentException("Unknown color");
    }
    /** idorid: "#AARRGGBB" for the host. */
    public static String toHex(int c) {
        String h = Integer.toHexString(c);
        while (h.length() < 8) h = "0" + h;
        return "#" + h;
    }
}
