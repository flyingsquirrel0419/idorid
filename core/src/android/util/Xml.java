package android.util;
import org.xmlpull.v1.XmlPullParser;
import org.xmlpull.v1.XmlSerializer;
public final class Xml { public static AttributeSet asAttributeSet(XmlPullParser parser){return parser instanceof AttributeSet?(AttributeSet)parser:null;} public static XmlPullParser newPullParser(){throw new UnsupportedOperationException("XML parser unavailable");} public static XmlSerializer newSerializer(){throw new UnsupportedOperationException("XML serializer unavailable");} }
