package android.content.res;

import android.os.Parcel;
import android.os.Parcelable;

public class Configuration implements Parcelable, Comparable<Configuration> {
    public static final int ORIENTATION_UNDEFINED = 0, ORIENTATION_PORTRAIT = 1, ORIENTATION_LANDSCAPE = 2;
    public static final int UI_MODE_TYPE_MASK = 0x0f, UI_MODE_TYPE_UNDEFINED = 0, UI_MODE_TYPE_NORMAL = 1;
    public static final int UI_MODE_NIGHT_MASK = 0x30, UI_MODE_NIGHT_UNDEFINED = 0, UI_MODE_NIGHT_NO = 0x10, UI_MODE_NIGHT_YES = 0x20;
    public static final int SCREENLAYOUT_SIZE_MASK = 0x0f, SCREENLAYOUT_SIZE_NORMAL = 2, SCREENLAYOUT_LAYOUTDIR_MASK = 0xc0, SCREENLAYOUT_LAYOUTDIR_LTR = 0x40,
        SCREENLAYOUT_LAYOUTDIR_RTL = 0x80, SCREENLAYOUT_LONG_MASK = 0x30, SCREENLAYOUT_LONG_YES = 0x20, SCREENLAYOUT_ROUND_MASK = 0x300, SCREENLAYOUT_ROUND_NO = 0x100;
    public static final int KEYBOARD_NOKEYS = 1, KEYBOARDHIDDEN_YES = 2, HARDKEYBOARDHIDDEN_YES = 2, NAVIGATION_NONAV = 1, TOUCHSCREEN_FINGER = 3;
    public static final int DENSITY_DPI_UNDEFINED = 0, SCREEN_WIDTH_DP_UNDEFINED = 0, SCREEN_HEIGHT_DP_UNDEFINED = 0, SMALLEST_SCREEN_WIDTH_DP_UNDEFINED = 0;
    public static final int COLOR_MODE_WIDE_COLOR_GAMUT_NO = 1, COLOR_MODE_HDR_NO = 4;
    public int orientation = ORIENTATION_PORTRAIT;
    public int uiMode = UI_MODE_TYPE_NORMAL | UI_MODE_NIGHT_NO;
    public float fontScale = 1f;
    public int screenWidthDp = 390, screenHeightDp = 844, smallestScreenWidthDp = 390, densityDpi = 160;
    public int screenLayout = SCREENLAYOUT_SIZE_NORMAL | SCREENLAYOUT_LAYOUTDIR_LTR | SCREENLAYOUT_LONG_YES | SCREENLAYOUT_ROUND_NO;
    public int keyboard = KEYBOARD_NOKEYS, keyboardHidden = KEYBOARDHIDDEN_YES, hardKeyboardHidden = HARDKEYBOARDHIDDEN_YES, navigation = NAVIGATION_NONAV, navigationHidden, touchscreen = TOUCHSCREEN_FINGER, mcc, mnc, colorMode = 5;
    public java.util.Locale locale = java.util.Locale.getDefault();
    public Configuration() {}
    public Configuration(Configuration o) { setTo(o); }
    public void setTo(Configuration o) {
        orientation = o.orientation; uiMode = o.uiMode; fontScale = o.fontScale; screenWidthDp = o.screenWidthDp; screenHeightDp = o.screenHeightDp;
        smallestScreenWidthDp = o.smallestScreenWidthDp; densityDpi = o.densityDpi; screenLayout = o.screenLayout; locale = o.locale;
        keyboard = o.keyboard; keyboardHidden = o.keyboardHidden; hardKeyboardHidden = o.hardKeyboardHidden; navigation = o.navigation; touchscreen = o.touchscreen; colorMode = o.colorMode;
    }
    public void setToDefaults() { setTo(new Configuration()); }
    public int updateFrom(Configuration o) { int d = diff(o); setTo(o); return d; }
    public int diff(Configuration o) {
        int d = 0;
        if (o.orientation != orientation) d |= 0x80;
        if (o.uiMode != uiMode) d |= 0x200;
        if (o.screenWidthDp != screenWidthDp || o.screenHeightDp != screenHeightDp) d |= 0x400;
        if (o.densityDpi != densityDpi) d |= 0x1000;
        if (o.fontScale != fontScale) d |= 0x40000000;
        if (o.locale != null && !o.locale.equals(locale)) d |= 0x4;
        return d;
    }
    public static boolean needNewResources(int configChanges, int interestingChanges) { return (configChanges & interestingChanges) != 0; }
    public boolean isNightModeActive() { return (uiMode & UI_MODE_NIGHT_MASK) == UI_MODE_NIGHT_YES; }
    public boolean isLayoutSizeAtLeast(int size) { return (screenLayout & SCREENLAYOUT_SIZE_MASK) >= size; }
    public boolean isScreenRound() { return false; }
    public boolean isScreenWideColorGamut() { return false; }
    public boolean isScreenHdr() { return false; }
    public int getLayoutDirection() { return 0; }
    public void setLayoutDirection(java.util.Locale l) {}
    public void setLocale(java.util.Locale l) { locale = l; }
    public android.os.LocaleList getLocales() { return new android.os.LocaleList(locale); }
    public void setLocales(android.os.LocaleList l) { if (l != null && l.size() > 0) locale = l.get(0); }
    public int compareTo(Configuration o) { return diff(o) == 0 ? 0 : 1; }
    public boolean equals(Configuration o) { return o != null && diff(o) == 0; }
    public boolean equals(Object o) { return o instanceof Configuration && diff((Configuration) o) == 0; }
    public int hashCode() { return orientation * 31 + uiMode * 17 + screenWidthDp; }
    public String toString() { return "{" + fontScale + " ?mcc?mnc " + locale + " sw" + smallestScreenWidthDp + "dp w" + screenWidthDp + "dp h" + screenHeightDp + "dp " + densityDpi + "dpi port night=" + isNightModeActive() + "}"; }
    public int describeContents() { return 0; }
    public void writeToParcel(Parcel dest, int flags) {}
    public void readFromParcel(Parcel src) {}
    public static final Parcelable.Creator<Configuration> CREATOR = new Parcelable.Creator<Configuration>() {
        public Configuration createFromParcel(Parcel p) { return new Configuration(); }
        public Configuration[] newArray(int n) { return new Configuration[n]; }
    };
 public int fontWeightAdjustment = Integer.MAX_VALUE;
}
