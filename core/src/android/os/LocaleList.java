package android.os;
import java.util.Locale;
public final class LocaleList {
    private final Locale[] locales;
    public LocaleList(Locale... locales) { this.locales=locales.clone(); }
    public static LocaleList getDefault() { return new LocaleList(Locale.getDefault()); }
    public static LocaleList getAdjustedDefault() { return getDefault(); }
    public static LocaleList getEmptyLocaleList() { return new LocaleList(); }
    public int size() { return locales.length; } public boolean isEmpty() { return locales.length==0; }
    public Locale get(int index) { return index<0 || index>=locales.length?null:locales[index]; }
    public int indexOf(Locale locale) { for(int i=0;i<locales.length;i++) if(locales[i].equals(locale))return i;return -1; }
    public boolean equals(Object o) { if(!(o instanceof LocaleList))return false; LocaleList l=(LocaleList)o; if(size()!=l.size())return false; for(int i=0;i<size();i++)if(!locales[i].equals(l.locales[i]))return false;return true; }
    public int hashCode() { int h=1;for(Locale l:locales)h=31*h+l.hashCode();return h; }
}
