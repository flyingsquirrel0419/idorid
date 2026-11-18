package java.util.function;
public interface BiConsumer<T,U> { void accept(T t,U u); default BiConsumer<T,U> andThen(final BiConsumer<? super T,? super U> a){java.util.Objects.requireNonNull(a);return new BiConsumer<T,U>(){public void accept(T t,U u){BiConsumer.this.accept(t,u);a.accept(t,u);}};} }
