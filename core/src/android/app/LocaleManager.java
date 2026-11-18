package android.app;
public class LocaleManager { private android.os.LocaleList locales=android.os.LocaleList.getEmptyLocaleList(); public android.os.LocaleList getApplicationLocales() { return locales; } public void setApplicationLocales(android.os.LocaleList l) { locales=l; }
}
