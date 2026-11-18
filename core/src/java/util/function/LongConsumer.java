package java.util.function;
public interface LongConsumer { void accept(long value); default LongConsumer andThen(final LongConsumer p){java.util.Objects.requireNonNull(p);return new LongConsumer(){public void accept(long v){LongConsumer.this.accept(v);p.accept(v);}};} }
