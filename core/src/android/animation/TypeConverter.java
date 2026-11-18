package android.animation;
public abstract class TypeConverter<T,V> { private final Class<T> from; private final Class<V> to; public TypeConverter(Class<T> from,Class<V> to){this.from=from;this.to=to;} public abstract V convert(T value); public Class<T> getSourceType(){return from;} public Class<V> getTargetType(){return to;} }
