package java.util.function;
public interface BiFunction<T,U,R> { R apply(T t,U u); default <V> BiFunction<T,U,V> andThen(final Function<? super R,? extends V> a){java.util.Objects.requireNonNull(a);return new BiFunction<T,U,V>(){public V apply(T t,U u){return a.apply(BiFunction.this.apply(t,u));}};} }
