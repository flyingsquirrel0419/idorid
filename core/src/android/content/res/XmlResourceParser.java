package android.content.res;

public interface XmlResourceParser extends android.util.AttributeSet, AutoCloseable {
    String getAttributeNamespace(int index);
    void close();
}
