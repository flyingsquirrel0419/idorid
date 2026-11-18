package android.icu.text;
import java.util.Date;
import java.util.Locale;
import android.icu.util.ULocale;
import android.icu.util.TimeZone;
public class DateFormat { private final java.text.SimpleDateFormat format; private DateFormat(String pattern,Locale locale){format=new java.text.SimpleDateFormat(pattern,locale);} public static DateFormat getInstanceForSkeleton(String skeleton,Locale locale){return new DateFormat(skeleton,locale);} public static DateFormat getInstanceForSkeleton(String skeleton,ULocale locale){return getInstanceForSkeleton(skeleton,locale.toLocale());} public void setTimeZone(TimeZone zone){format.setTimeZone(zone.toJavaTimeZone());} public void setContext(DisplayContext context){} public String format(Date date){return format.format(date);} }
