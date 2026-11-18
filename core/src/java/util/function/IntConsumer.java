package java.util.function;
public interface IntConsumer { void accept(int value); default IntConsumer andThen(final IntConsumer p){java.util.Objects.requireNonNull(p);return new IntConsumer(){public void accept(int v){IntConsumer.this.accept(v);p.accept(v);}};} }
