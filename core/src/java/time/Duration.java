package java.time;
public final class Duration implements Comparable<Duration>,java.io.Serializable {
    public static final Duration ZERO=new Duration(0); private final long nanos; private Duration(long nanos){this.nanos=nanos;} public static Duration ofMillis(long millis){return new Duration(millis*1000000L);} public static Duration ofSeconds(long seconds){return new Duration(seconds*1000000000L);} public static Duration ofNanos(long nanos){return new Duration(nanos);} public long toMillis(){return nanos/1000000L;} public long toNanos(){return nanos;} public long getSeconds(){return nanos/1000000000L;} public int getNano(){return (int)(nanos%1000000000L);} public int compareTo(Duration other){return nanos<other.nanos?-1:nanos>other.nanos?1:0;}
}
